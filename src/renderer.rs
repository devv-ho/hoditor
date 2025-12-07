use crate::{
    app::Context,
    cursor::{CursorStyle, Position},
    logger::Logger,
};
use anyhow::{Context as AnyhowContext, Error, Result};
use crossterm::{
    cursor::SetCursorStyle,
    execute, queue,
    style::{Print, SetBackgroundColor, SetForegroundColor},
    terminal,
};
use std::io::Write;

pub struct WindowSize {
    pub width: usize,
    pub height: usize,
}

pub struct Renderer<W: Write> {
    writer: W,
    win_size: WindowSize,
    line_num_width: usize,
    file_extension: String,
    last_viewport_offset: usize,
}

pub const STATUS_BAR_HEIGHT: usize = 2usize;
impl<W: Write> Renderer<W> {
    pub fn new(writer: W, file_name: &str) -> Self {
        let (width, height) = crossterm::terminal::size()
            .with_context(|| format!("Error Reading Window Size"))
            .unwrap();

        let win_size = WindowSize {
            width: width as usize,
            height: height as usize,
        };

        let file_extension = std::path::Path::new(file_name)
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("txt")
            .to_string();

        Self {
            writer,
            win_size,
            line_num_width: 0,
            file_extension,
            last_viewport_offset: 0,
        }
    }

    pub fn init(&mut self, context: &Context) {
        crossterm::terminal::enable_raw_mode()
            .with_context(|| format!("Error While Enabling Raw Mode"))
            .unwrap();
        crossterm::execute!(self.writer, crossterm::event::EnableMouseCapture)
            .with_context(|| format!("Error while Enabling Mouse Capture"))
            .unwrap();
        crossterm::execute!(self.writer, crossterm::terminal::EnterAlternateScreen)
            .with_context(|| format!("Error While Entering AlternateScreen"))
            .unwrap();

        self.set_bg_color();
        self.line_num_width = (context.buffer.len() - 1).ilog10() as usize + 1;

        self.render(context);
    }

    pub fn render(&mut self, context: &Context) {
        Logger::log(format!("Render Start"));
        queue!(self.writer, crossterm::cursor::Hide)
            .with_context(|| format!("Error While Hiding Cursor"))
            .unwrap();

        let scroll_delta = context.viewport.offset as i32 - self.last_viewport_offset as i32;

        Logger::log(format!("scroll : {scroll_delta}"));
        if scroll_delta == 0 {
            self.draw_lines(context);
        } else if scroll_delta > 0 && scroll_delta < context.viewport.height as i32 {
            Logger::log(format!("Scroll Up"));
            let lines_to_scroll = scroll_delta as u16;
            queue!(self.writer, terminal::ScrollUp(lines_to_scroll))
                .with_context(|| format!("Error While Scrolling Up"))
                .unwrap();

            self.draw_lines_range(
                context,
                context.viewport.height - scroll_delta as usize,
                context.viewport.height,
            );
        } else if scroll_delta < 0 && -scroll_delta < context.viewport.height as i32 {
            let lines_to_scroll = (-scroll_delta) as u16;
            queue!(self.writer, terminal::ScrollDown(lines_to_scroll))
                .with_context(|| format!("Error While Scrolling Down"))
                .unwrap();
            self.draw_lines_range(context, 0, (-scroll_delta) as usize);
        } else {
            self.draw_lines(context);
        }

        self.last_viewport_offset = context.viewport.offset;

        self.draw_status_bar(context);

        self.draw_cursor(context);
        queue!(self.writer, crossterm::cursor::Show)
            .with_context(|| format!("Error While Showing Cursor"))
            .unwrap();

        self.writer
            .flush()
            .with_context(|| format!("Error While Flusing Terminal Queue"))
            .unwrap();
    }

    fn set_bg_color(&mut self) {
        let tokyonight_bg = crossterm::style::Color::Rgb {
            r: 0x1a,
            g: 0x1b,
            b: 0x26,
        };

        execute!(
            self.writer,
            SetBackgroundColor(tokyonight_bg),
            terminal::Clear(terminal::ClearType::All)
        )
        .with_context(|| format!("Error While Setting BG Color"))
        .unwrap();
    }

    fn draw_lines(&mut self, context: &Context) {
        self.draw_lines_range(context, 0, context.viewport.height)
    }

    fn draw_lines_range(&mut self, context: &Context, screen_start: usize, screen_end: usize) {
        queue!(
            self.writer,
            crossterm::cursor::MoveTo(0, screen_start as u16)
        )
        .with_context(|| format!("Error While Queuing Cursor Move. {context}"))
        .unwrap();

        for screen_row in screen_start..screen_end {
            let buffer_line = context.viewport.offset + screen_row;
            let line = format!(
                "{line_num:>width$}",
                line_num = buffer_line,
                width = self.line_num_width
            );

            let line = line + " " + context.buffer.get(buffer_line);

            queue!(
                self.writer,
                terminal::Clear(terminal::ClearType::CurrentLine),
                Print(&line)
            )
            .with_context(|| {
                format!(
                    "Error While Printing Line. screen_row:{}, buffer_row:{}, line:{}, Context:{}",
                    screen_row, buffer_line, &line, context
                )
            })
            .unwrap();

            queue!(
                self.writer,
                crossterm::cursor::MoveDown(1),
                crossterm::cursor::MoveToColumn(0)
            ).with_context(||
                format!(
                    "Error While Moving Cursor To Next Line. screen_row:{}, buffer_row:{}, line:{}, Context:{}",
                    screen_row,
                    buffer_line,
                    &line,
                    context)
            )
            .unwrap();
        }
    }

    fn draw_status_bar(&mut self, context: &Context) {
        queue!(
            self.writer,
            crossterm::cursor::MoveTo(0, (self.win_size.height - STATUS_BAR_HEIGHT) as u16),
            crossterm::terminal::Clear(crossterm::terminal::ClearType::CurrentLine),
            Print(format!("mode: {:?}", context.app_state.mode())),
            crossterm::cursor::MoveDown(1),
            crossterm::terminal::Clear(crossterm::terminal::ClearType::CurrentLine),
        )
        .with_context(|| format!("Error While Drawing Status Bar"))
        .unwrap();
    }

    fn draw_cursor(&mut self, context: &Context) {
        let cursor_buffer = context.cursor.pos();
        let cursor_ui = Position {
            row: cursor_buffer.row - context.viewport.offset,
            col: self.line_num_width + 1 + cursor_buffer.col,
        };
        let cursor_style_on_crossterm = match context.cursor.style() {
            CursorStyle::Block => SetCursorStyle::SteadyBlock,
            CursorStyle::Bar => SetCursorStyle::SteadyBar,
        };

        Logger::log(format!("{:?}", cursor_ui)).ok();
        queue!(
            self.writer,
            crossterm::cursor::MoveTo(cursor_ui.col as u16, cursor_ui.row as u16),
            cursor_style_on_crossterm,
        )
        .with_context(|| format!("Error While Drawing Cursor. Context:{context}"))
        .unwrap();
    }
}
