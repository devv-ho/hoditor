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
    line_num_width: usize,
    highlighter: Highlighter,
    last_viewport_offset: usize,
}

pub const STATUS_BAR_HEIGHT: usize = 2usize;
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
            line_num_width: 0,
            highlighter,
            last_viewport_offset: 0,
        }
    }

    pub fn init(&mut self, context: &Context) -> Result<(), Box<dyn Error>> {
        crossterm::terminal::enable_raw_mode()?;
        crossterm::execute!(self.writer, crossterm::event::EnableMouseCapture)?;
        crossterm::execute!(self.writer, crossterm::terminal::EnterAlternateScreen)?;

        self.set_bg_color()?;
        self.line_num_width = (context.buffer.len() - 1).ilog10() as usize + 1;

        self.render(context)?;

        Ok(())
    }

    pub fn render(&mut self, context: &Context) -> Result<(), Box<dyn Error>> {
        queue!(self.writer, crossterm::cursor::Hide)?;

        let scroll_delta = context.viewport.offset as i32 - self.last_viewport_offset as i32;

        if scroll_delta == 0 {
            self.draw_lines(context)?;
        } else if scroll_delta > 0 && scroll_delta < context.viewport.height as i32 {
            let lines_to_scroll = scroll_delta as u16;
            queue!(self.writer, terminal::ScrollUp(lines_to_scroll))?;

            self.draw_lines_range(
                context,
                context.viewport.height - scroll_delta as usize,
                context.viewport.height,
            )?;
        } else if scroll_delta < 0 && -scroll_delta < context.viewport.height as i32 {
            let lines_to_scroll = (-scroll_delta) as u16;
            queue!(self.writer, terminal::ScrollDown(lines_to_scroll))?;
            self.draw_lines_range(context, 0, (-scroll_delta) as usize)?;
        } else {
            self.draw_lines(context)?;
        }

        self.last_viewport_offset = context.viewport.offset;

        self.draw_status_bar(context)?;

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
        self.draw_lines_range(context, 0, context.viewport.height)
    }

    fn draw_lines_range(
        &mut self,
        context: &Context,
        screen_start: usize,
        screen_end: usize,
    ) -> Result<(), Box<dyn Error>> {
        queue!(
            self.writer,
            crossterm::cursor::MoveTo(0, screen_start as u16)
        )?;

        for screen_row in screen_start..screen_end {
            let buffer_line = context.viewport.offset + screen_row;
            let mut line = format!(
                "{line_num:>width$}",
                line_num = buffer_line,
                width = self.line_num_width
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

    fn draw_status_bar(&mut self, context: &Context) -> Result<(), Box<dyn Error>> {
        queue!(
            self.writer,
            crossterm::cursor::MoveTo(0, (self.win_size.height - STATUS_BAR_HEIGHT) as u16),
            crossterm::terminal::Clear(crossterm::terminal::ClearType::CurrentLine),
            Print(format!("mode: {:?}", context.app_state.mode())),
            crossterm::cursor::MoveDown(1),
            crossterm::terminal::Clear(crossterm::terminal::ClearType::CurrentLine),
        )?;

        Ok(())
    }

    fn draw_cursor(&mut self, context: &Context) -> Result<(), Box<dyn Error>> {
        let cursor_buffer = context.cursor.pos();
        let cursor_ui = Position {
            row: cursor_buffer.row - context.viewport.offset,
            col: self.line_num_width + 1 + cursor_buffer.col,
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
