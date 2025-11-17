struct Editor {
    mode: Mode,
    buffer: Buffer,
    viewport: Viewport,
    cursor: Cursor,
    input_handler: Handler,
    renderer: Renderer,
}

impl Editor {
    pub fn init() {
        /// initialize (e.g. mode, cursor pos, rendering ...)
    }

    pub fn run() {
        let mut is_running = true;
        while is_running {
            /// poll crossterm event
            /// read event
            let actions: Vec<Action> = input_handler.handle(event);
            let mut modified_lines: Vec<Renderable> = Vec::new(); 
            actions.iter().for_each(|action| {
                let modified = match action {
                    EditBuffer => buffer.conduct(action),
                    MoveViewport => viewport.conduct(action),
                    MoveCursor => cursor.conduct(action)
                };
                modified_lines.push_back(modified);
            });
            renderer.render(modified, cursor, mode);
        }
    }
}

fn main() {
    /// open file
    let app = Editor::new();
    
    app.init();
    app.run();
}
