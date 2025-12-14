use anyhow::{Context as AnyhowContext, Result};

use crate::{
    app, buffer::Buffer, cursor::Cursor, input_handler::EventHandler, log, renderer::Renderer,
    state::State,
};
use std::io::Write;

pub struct Application<W: Write> {
    file_name: String,
    buffer: Buffer,
    app_state: State,
    cursor: Cursor,
    viewport: Viewport,
    renderer: Renderer<W>,
    event_handler: EventHandler,
}

impl<W: Write> Application<W> {
    pub fn new(writer: W, file_name: &str) -> Self {
        log!("Create App");
        let buffer = Buffer::from_file(file_name);
        let app_state = State::new();
        let cursor = Cursor::new();
        let renderer = Renderer::new(writer, file_name);
        let viewport = Viewport::new();

        Self {
            file_name: file_name.to_string(),
            buffer,
            app_state,
            cursor,
            viewport,
            renderer,
            event_handler: EventHandler::new(),
        }
    }

    pub fn init(&mut self) -> Result<()> {
        log!("Init App");
        let mode = self.app_state.mode();
        let app_context = Context {
            cursor: &mut self.cursor,
            buffer: &mut self.buffer,
            app_state: &mut self.app_state,
            viewport: &mut self.viewport,
            file_name: &mut self.file_name,
            cmd_buffer: &self.event_handler.get_cmd_buffer(mode),
        };

        self.renderer.init(&app_context);
        log!("App Initialized");

        Ok(())
    }

    pub fn run(&mut self) -> Result<()> {
        loop {
            if crossterm::event::poll(std::time::Duration::from_millis(10)).unwrap() {
                let event = crossterm::event::read().unwrap();
                let mode = self.app_state.mode();
                let cmd_buffer = self.event_handler.get_cmd_buffer(mode);

                self.app_state.set_should_render(false);

                let mut app_context = Some(Context {
                    cursor: &mut self.cursor,
                    buffer: &mut self.buffer,
                    app_state: &mut self.app_state,
                    viewport: &mut self.viewport,
                    file_name: &mut self.file_name,
                    cmd_buffer: &cmd_buffer,
                });

                let cmd = self.event_handler.handle(event, mode);
                cmd.execute(&mut app_context);

                if let Some(ref ctx) = app_context
                    && ctx.app_state.should_render()
                {
                    log!("Render!");
                    self.renderer.render(ctx);
                }
            }

            if self.app_state.should_terminate() {
                break;
            }
        }

        Ok(())
    }

    pub fn drop(&self) -> Result<()> {
        crossterm::terminal::disable_raw_mode()?;

        Ok(())
    }
}

pub struct Context<'a> {
    pub cursor: &'a mut Cursor,
    pub buffer: &'a mut Buffer,
    pub app_state: &'a mut State,
    pub viewport: &'a mut Viewport,
    pub file_name: &'a mut String,
    pub cmd_buffer: &'a String,
}

impl<'a> std::fmt::Display for Context<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Cursor {{ {}, {} }}, Buffer [ {} ], State {{ mode:{:?} }}, Viewport {{ offset:{}, height:{}}}",
            self.cursor.row(),
            self.cursor.col(),
            self.buffer.get(self.cursor.row()),
            self.app_state.mode(),
            self.viewport.offset,
            self.viewport.height,
        )
    }
}

pub struct Viewport {
    pub height: usize,
    pub offset: usize,
}

impl Viewport {
    pub fn new() -> Self {
        let (_, win_height) = crossterm::terminal::size()
            .with_context(|| format!(""))
            .unwrap();

        Self {
            height: win_height as usize - config::UI::STATUS_BAR_HEIGHT,
            offset: 0,
        }
    }

    pub fn update(&mut self, cursor_row: usize, eof: usize) {
        log!(
            "[VP][update] offset:{}, height:{}, curosr_row:{}, eof:{}",
            self.offset,
            self.height,
            cursor_row,
            eof
        );

        self.offset = if cursor_row < self.offset + config::UI::SCROLL_HEIGHT {
            if cursor_row < config::UI::SCROLL_HEIGHT {
                0
            } else {
                cursor_row - config::UI::SCROLL_HEIGHT
            }
        } else if cursor_row >= self.offset + self.height - config::UI::SCROLL_HEIGHT {
            if cursor_row + config::UI::SCROLL_HEIGHT >= eof {
                eof - self.height
            } else {
                cursor_row + config::UI::SCROLL_HEIGHT - self.height
            }
        } else {
            self.offset
        }
    }
}

pub mod config {
    pub enum UI {}
    impl UI {
        pub const STATUS_BAR_HEIGHT: usize = 2;
        pub const SCROLL_HEIGHT: usize = 15;
    }
}
