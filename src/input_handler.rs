use crate::{app::Context, cursor::CursorStyle, state::Mode};

use crossterm::event::{Event, KeyCode, KeyEvent};

pub struct EventHandler;
impl EventHandler {
    pub fn new() -> Self {
        Self
    }

    pub fn handle(&self, event: Event, mode: Mode) -> Box<dyn Executable> {
        let Event::Key(key) = event else {
            return Box::new(DoNothing);
        };

        match mode {
            Mode::Edit => self.handle_edit_event(key),
            Mode::Cmd => self.handle_cmd_event(key),
        }
    }

    pub fn handle_edit_event(&self, key: KeyEvent) -> Box<dyn Executable> {
        match key.code {
            KeyCode::Char(ch) => Box::new(InsertChar { ch }),

            KeyCode::Backspace => Box::new(RemoveChar),

            KeyCode::Enter => Box::new(InsertNewLine),

            KeyCode::Tab => Box::new(InsertTab),

            KeyCode::Esc => Box::new(ChangeMode { mode: Mode::Cmd }),

            _ => Box::new(DoNothing),
        }
    }

    fn handle_cmd_event(&self, key: KeyEvent) -> Box<dyn Executable> {
        match key.code {
            KeyCode::Char(ch) => match ch {
                'h' => Box::new(MoveCursorLeft),

                'j' => Box::new(MoveCursorDown),

                'k' => Box::new(MoveCursorUp),

                'l' => Box::new(MoveCursorRight),

                'i' => Box::new(ChangeMode { mode: Mode::Edit }),

                _ => Box::new(DoNothing),
            },

            KeyCode::Esc => Box::new(TerminateApp),

            _ => Box::new(DoNothing),
        }
    }
}

pub struct Viewport {
    pub height: usize,
    pub offset: usize,
}

pub trait Executable: std::fmt::Debug {
    fn execute(&self, context: &mut Context);
}

#[derive(Debug)]
struct DoNothing;
impl Executable for DoNothing {
    fn execute(&self, _: &mut Context) {}
}

#[derive(Debug)]
struct MoveCursorLeft;
impl Executable for MoveCursorLeft {
    fn execute(&self, context: &mut Context) {
        let cursor = &mut context.cursor;

        if cursor.col() > 0 {
            cursor.move_left(1);
        }
    }
}

#[derive(Debug)]
struct MoveCursorRight;
impl Executable for MoveCursorRight {
    fn execute(&self, context: &mut Context) {
        let cursor = &mut context.cursor;
        let buf = &mut context.buffer;

        if cursor.col() < buf.len_of(cursor.row()) {
            cursor.move_right(1);
        }
    }
}

#[derive(Debug)]
struct MoveCursorUp;
impl Executable for MoveCursorUp {
    fn execute(&self, context: &mut Context) {
        let cursor = &mut context.cursor;
        let buf = &mut context.buffer;

        if cursor.row() > 0 {
            cursor.move_up(1);
            if cursor.col() > buf.len_of(cursor.row()) {
                cursor.set_col(buf.len_of(cursor.row()));
            }
        }

        if cursor.row() < context.viewport.offset {
            context.viewport.offset -= 1;
        }
    }
}

#[derive(Debug)]
struct MoveCursorDown;
impl Executable for MoveCursorDown {
    fn execute(&self, context: &mut Context) {
        let cursor = &mut context.cursor;
        let buf = &mut context.buffer;

        if cursor.row() < buf.len() - 1 {
            cursor.move_down(1);
            if cursor.col() > buf.len_of(cursor.row()) {
                cursor.set_col(buf.len_of(cursor.row()));
            }
        }

        if cursor.row() >= context.viewport.offset + context.viewport.height {
            context.viewport.offset += 1;
        }
    }
}

#[derive(Debug)]
struct InsertChar {
    ch: char,
}
impl Executable for InsertChar {
    fn execute(&self, context: &mut Context) {
        let cursor = &mut context.cursor;
        let buf = &mut context.buffer;

        buf.insert_char(cursor.row(), cursor.col(), self.ch);
        cursor.move_right(1);
    }
}

#[derive(Debug)]
struct InsertTab;
impl Executable for InsertTab {
    fn execute(&self, context: &mut Context) {
        let cursor = &mut context.cursor;
        let buf = &mut context.buffer;

        buf.insert_string(cursor.row(), cursor.col(), &String::from("    "));
        cursor.move_right(4);
    }
}

#[derive(Debug)]
struct RemoveChar;
impl Executable for RemoveChar {
    fn execute(&self, context: &mut Context) {
        let cursor = &mut context.cursor;
        let buf = &mut context.buffer;

        if cursor.col() > 0 {
            buf.remove_char(cursor.row(), cursor.col() - 1);

            cursor.move_left(1);
        } else if cursor.row() > 0 {
            let next_col = buf.len_of(cursor.row() - 1);
            let cur_line = buf.get(cursor.row()).clone();

            buf.insert_string(cursor.row() - 1, next_col, &cur_line);
            buf.remove(cursor.row());

            cursor.move_up(1);
            cursor.move_to_col(next_col);
        }
    }
}

#[derive(Debug)]
struct InsertNewLine;
impl Executable for InsertNewLine {
    fn execute(&self, context: &mut Context) {
        let cursor = &mut context.cursor;
        let buf = &mut context.buffer;

        let rear = buf.get_string(
            cursor.row(),
            cursor.col(),
            buf.len_of(cursor.row()) - cursor.col(),
        );

        buf.remove_string(cursor.row(), cursor.col(), rear.len());
        buf.insert(cursor.row() + 1, &rear);
        cursor.move_down(1);
        cursor.move_to_col(0);
    }
}

#[derive(Debug)]
struct ChangeMode {
    mode: Mode,
}
impl Executable for ChangeMode {
    fn execute(&self, context: &mut Context) {
        context.app_state.set_mode(self.mode);

        match self.mode {
            Mode::Cmd => context.cursor.set_style(CursorStyle::Block),
            Mode::Edit => context.cursor.set_style(CursorStyle::Bar),
        }
    }
}

#[derive(Debug)]
struct TerminateApp;
impl Executable for TerminateApp {
    fn execute(&self, context: &mut Context) {
        context.app_state.terminate_app();
    }
}
