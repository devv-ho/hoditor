use crate::{
    app::Context,
    cursor::{CursorStyle, Position},
    highlighter::{Highlighter, style_to_crossterm_color},
    logger::Logger,
};
use crossterm::{
    cursor::SetCursorStyle,
    execute, queue,
    style::{Print, SetBackgroundColor, SetForegroundColor},
    terminal,
};
use std::{error::Error, io::Write};

pub struct WindowSize {
    pub width: usize,
    pub height: usize,
}

pub struct Renderer<W: Write> {
    writer: W,
    win_size: WindowSize,
    highlighter: Highlighter,
    last_viewport_offset: usize,
}

impl<W: Write> Renderer<W> {
    pub fn new(writer: W, file_name: &str) -> Self {
        let win_size = crossterm::terminal::size().unwrap();
        let win_size = WindowSize {
            width: win_size.0 as usize,
            height: win_size.1 as usize,
        };

        let file_extension = std::path::Path::new(file_name)
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("txt")
            .to_string();
        let highlighter = Highlighter::new(&file_extension);

        Self {
            writer,
            win_size,
            highlighter,
            last_viewport_offset: 0,
        }
    }

    pub fn init(&mut self, context: &Context) -> Result<(), Box<dyn Error>> {
        crossterm::terminal::enable_raw_mode()?;
        crossterm::execute!(self.writer, crossterm::terminal::EnterAlternateScreen)?;

        self.set_bg_color()?;
        self.render(context)?;

        Ok(())
    }

    pub fn render(&mut self, context: &Context) -> Result<(), Box<dyn Error>> {
        Logger::log(format!(
            "{:?} / {} / {}",
            context.cursor.pos(),
            context.buffer.get(context.cursor.row()),
            context.viewport.offset,
        ))?;

        queue!(self.writer, crossterm::cursor::Hide)?;

        let scroll_delta = context.viewport.offset as i32 - self.last_viewport_offset as i32;

        if scroll_delta == 0 {
            self.draw_lines(context)?;
        } else if scroll_delta > 0 && scroll_delta < self.win_size.height as i32 {
            let lines_to_scroll = scroll_delta as u16;
            queue!(self.writer, terminal::ScrollUp(lines_to_scroll))?;

            self.draw_lines_range(
                context,
                self.win_size.height - scroll_delta as usize,
                self.win_size.height,
            )?;
        } else if scroll_delta < 0 && -scroll_delta < self.win_size.height as i32 {
            let lines_to_scroll = (-scroll_delta) as u16;
            queue!(self.writer, terminal::ScrollDown(lines_to_scroll))?;
            self.draw_lines_range(context, 0, (-scroll_delta) as usize)?;
        } else {
            self.draw_lines(context)?;
        }

        self.last_viewport_offset = context.viewport.offset;

        self.draw_cursor(context)?;
        queue!(self.writer, crossterm::cursor::Show)?;

        self.writer.flush()?;

        Ok(())
    }

    fn set_bg_color(&mut self) -> Result<(), Box<dyn Error>> {
        let tokyonight_bg = crossterm::style::Color::Rgb {
            r: 0x1a,
            g: 0x1b,
            b: 0x26,
        };

        execute!(
            self.writer,
            SetBackgroundColor(tokyonight_bg),
            terminal::Clear(terminal::ClearType::All)
        )?;

        Ok(())
    }

    fn draw_lines(&mut self, context: &Context) -> Result<(), Box<dyn Error>> {
        self.draw_lines_range(context, 0, self.win_size.height)
    }

    fn draw_lines_range(
        &mut self,
        context: &Context,
        screen_start: usize,
        screen_end: usize,
    ) -> Result<(), Box<dyn Error>> {
        let line_num_width = (context.buffer.len() - 1).ilog10() as usize + 1;

        queue!(
            self.writer,
            crossterm::cursor::MoveTo(0, screen_start as u16)
        )?;

        for screen_row in screen_start..screen_end {
            let buffer_line = context.viewport.offset + screen_row;
            let mut line = format!(
                "{line_num:>width$}",
                line_num = buffer_line,
                width = line_num_width
            );

            if buffer_line < context.buffer.len() {
                line = line + &format!(" {}", context.buffer.get(buffer_line));
            }

            let highlighted = self.highlighter.highlight_line(&line);

            let mut chars_written = 0;

            for (style, text) in highlighted {
                let (fg, bg) = style_to_crossterm_color(style);
                chars_written += text.len();
                queue!(
                    self.writer,
                    SetForegroundColor(fg),
                    SetBackgroundColor(bg),
                    Print(&text)
                )?;
            }

            if chars_written < self.win_size.width {
                let padding = " ".repeat(self.win_size.width - chars_written);
                queue!(self.writer, Print(padding))?;
            }

            queue!(
                self.writer,
                crossterm::cursor::MoveDown(1),
                crossterm::cursor::MoveToColumn(0)
            )?;
        }

        Ok(())
    }

    fn draw_cursor(&mut self, context: &Context) -> Result<(), Box<dyn Error>> {
        let line_num_width = (context.buffer.len() - 1).ilog10() as usize + 1;
        let line_offset = line_num_width + 1;

        let cursor_buffer = context.cursor.pos();
        let cursor_ui = Position {
            row: cursor_buffer.row - context.viewport.offset,
            col: line_offset + cursor_buffer.col,
        };
        let cursor_style_on_crossterm = match context.cursor.style() {
            CursorStyle::Block => SetCursorStyle::SteadyBlock,
            CursorStyle::Bar => SetCursorStyle::SteadyBar,
        };

        Logger::log(format!("{:?}", cursor_ui))?;
        queue!(
            self.writer,
            crossterm::cursor::MoveTo(cursor_ui.col as u16, cursor_ui.row as u16),
            cursor_style_on_crossterm,
        )?;

        Ok(())
    }
}
