pub struct State {
    mode: Mode,
    should_render: bool,
    should_terminate: bool,
}

impl State {
    pub fn new() -> Self {
        Self {
            mode: Mode::Normal,
            should_render: true,
            should_terminate: false,
        }
    }

    pub fn mode(&self) -> Mode {
        self.mode
    }

    pub fn set_mode(&mut self, mode: Mode) {
        self.mode = mode;
    }

    pub fn set_should_render(&mut self, should_render: bool) {
        self.should_render = should_render;
    }

    pub fn should_render(&self) -> bool {
        self.should_render
    }

    pub fn should_terminate(&self) -> bool {
        self.should_terminate
    }

    pub fn terminate_app(&mut self) {
        self.should_terminate = true;
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Mode {
    Normal,
    Cmd,
    Edit,
}
