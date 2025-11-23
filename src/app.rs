use crate::{
    buffer::Buffer, cursor::Cursor, input_handler::EventHandler, input_handler::Viewport,
    renderer::Renderer, state::State,
};
use std::{error::Error, io::Write};

pub struct Application<W: Write> {
    buffer: Buffer,
    app_state: State,
    cursor: Cursor,
    viewport: Viewport,
    input_handler: EventHandler,
    renderer: Renderer<W>,
}

impl<W: Write> Application<W> {
    pub fn new(writer: W, file_name: &str) -> Result<Self, Box<dyn Error>> {
        let buffer = Buffer::from_file(file_name)?;
        let app_state = State::new();
        let cursor = Cursor::new();

        let renderer = Renderer::new(writer, file_name);

        let (_, win_height) = crossterm::terminal::size()?;
        let viewport = Viewport {
            height: win_height as usize,
            offset: 0,
        };

        let input_handler = EventHandler::new();

        Ok(Self {
            buffer,
            app_state,
            cursor,
            viewport,
            input_handler,
            renderer,
        })
    }

    pub fn init(&mut self) -> Result<(), Box<dyn Error>> {
        let app_context = Context {
            cursor: &mut self.cursor,
            buffer: &mut self.buffer,
            app_state: &mut self.app_state,
            viewport: &mut self.viewport,
        };

        self.renderer.init(&app_context)?;

        Ok(())
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        loop {
            if crossterm::event::poll(std::time::Duration::from_millis(10)).unwrap() {
                let event = crossterm::event::read().unwrap();

                let command = self.input_handler.handle(event, self.app_state.mode());

                let mut app_context = Context {
                    cursor: &mut self.cursor,
                    buffer: &mut self.buffer,
                    app_state: &mut self.app_state,
                    viewport: &mut self.viewport,
                };

                command.execute(&mut app_context);

                self.renderer.render(&app_context)?;
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
}
