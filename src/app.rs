use crate::{
    buffer::Buffer,
    cursor::Cursor,
    input_handler::EventHandler,
    renderer::{Renderer, STATUS_BAR_HEIGHT},
    state::State,
};
use std::{error::Error, io::Write};

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
    pub fn new(writer: W, file_name: &str) -> Result<Self, Box<dyn Error>> {
        let buffer = Buffer::from_file(file_name)?;
        let app_state = State::new();
        let cursor = Cursor::new();

        let renderer = Renderer::new(writer, file_name);

        let (_, win_height) = crossterm::terminal::size()?;
        let viewport = Viewport {
            height: (win_height as usize - STATUS_BAR_HEIGHT),
            offset: 0,
        };

        Ok(Self {
            file_name: file_name.to_string(),
            buffer,
            app_state,
            cursor,
            viewport,
            renderer,
            event_handler: EventHandler::new(),
        })
    }

    pub fn init(&mut self) -> Result<(), Box<dyn Error>> {
        let app_context = Context {
            cursor: &mut self.cursor,
            buffer: &mut self.buffer,
            app_state: &mut self.app_state,
            viewport: &mut self.viewport,
            file_name: &mut self.file_name,
        };

        self.renderer.init(&app_context)?;

        Ok(())
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        loop {
            if crossterm::event::poll(std::time::Duration::from_millis(10)).unwrap() {
                let event = crossterm::event::read().unwrap();
                let mode = self.app_state.mode();

                let mut app_context = Some(Context {
                    cursor: &mut self.cursor,
                    buffer: &mut self.buffer,
                    app_state: &mut self.app_state,
                    viewport: &mut self.viewport,
                    file_name: &mut self.file_name,
                });

                let cmd = self.event_handler.handle(event, mode);
                cmd.execute(&mut app_context);

                if let Some(ref ctx) = app_context {
                    self.renderer.render(ctx).unwrap();
                }
            }

            if self.app_state.should_terminate() {
                break;
            }
        }

        Ok(())
    }

    pub fn drop(&self) -> Result<(), Box<dyn Error>> {
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
}

pub struct Viewport {
    pub height: usize,
    pub offset: usize,
}
