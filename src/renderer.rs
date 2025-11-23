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

        // Queue everything first (no immediate flush)
        queue!(self.writer, crossterm::cursor::Hide)?;
        self.draw_lines(context)?;
        self.draw_cursor(context)?;
        queue!(self.writer, crossterm::cursor::Show)?;

        // Single flush at the end
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
        let line_num_width = (context.buffer.len() - 1).ilog10() as usize + 1;

        queue!(self.writer, crossterm::cursor::MoveTo(0, 0))?;
        for i in context.viewport.offset..(context.viewport.offset + self.win_size.height) {
            let mut line = format!("{line_num:>width$}", line_num = i, width = line_num_width);

            if i < context.buffer.len() {
                line = line + &format!(" {}", context.buffer.get(i));
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
