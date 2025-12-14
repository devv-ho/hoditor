use crate::{
    app::Context,
    buffer::Buffer,
    cmd_dispatcher::{self, CmdDispatcher},
    cursor::CursorStyle,
    log,
    state::Mode,
};
use std::{
    env,
    fs::File,
    io::{BufWriter, Write},
    os::unix::process::CommandExt,
    process::Command as ProcessCommand,
};

use crossterm::event::{Event, KeyCode, MouseEventKind};

pub struct EventHandler {
    normal_dispatcher: CmdDispatcher,
    cmd_dispatcher: CmdDispatcher,
}

impl EventHandler {
    pub fn new() -> Self {
        let mut normal_dispatcher = CmdDispatcher::new();
        normal_dispatcher.register("h", Command::MoveCursor { dx: -1, dy: 0 });
        normal_dispatcher.register("j", Command::MoveCursor { dx: 0, dy: 1 });
        normal_dispatcher.register("k", Command::MoveCursor { dx: 0, dy: -1 });
        normal_dispatcher.register("l", Command::MoveCursor { dx: 1, dy: 0 });
        normal_dispatcher.register("gg", Command::MoveCursorSOF);
        normal_dispatcher.register("G", Command::MoveCursorEOF);
        normal_dispatcher.register("i", Command::ChangeMode(Mode::Edit));
        normal_dispatcher.register("o", Command::InsertEmptyLineBelow);
        normal_dispatcher.register("O", Command::InsertEmptyLineAbove);
        normal_dispatcher.register("A", Command::MoveCursorToLineEnd);
        normal_dispatcher.register(":", Command::ChangeMode(Mode::Cmd));

        let mut cmd_dispatcher = CmdDispatcher::new();
        cmd_dispatcher.register("e", Command::OpenFile(String::new()));
        cmd_dispatcher.register("w", Command::Save);
        cmd_dispatcher.register("W", Command::SaveAndRestart);
        cmd_dispatcher.register("q", Command::TerminateApp);

        log!("Event Handler Created");

        Self {
            normal_dispatcher,
            cmd_dispatcher,
        }
    }

    pub fn get_cmd_buffer(&self, mode: Mode) -> String {
        match mode {
            Mode::Cmd => self.cmd_dispatcher.get_query(),
            Mode::Normal => self.normal_dispatcher.get_query(),
            Mode::Edit => String::new(),
        }
    }

    pub fn handle(&mut self, event: Event, mode: Mode) -> Command {
        log!("Event: {:?}", event);

        match mode {
            Mode::Edit => Self::handle_edit_event(event),
            Mode::Normal => self.handle_normal_event(event),
            Mode::Cmd => self.handle_cmd_event(event),
        }
    }

    fn handle_edit_event(event: Event) -> Command {
        match event {
            Event::Key(key) => match key.code {
                KeyCode::Char(ch) => Command::InsertChar(ch),
                KeyCode::Backspace => Command::RemoveChar,
                KeyCode::Enter => Command::InsertNewLine,
                KeyCode::Tab => Command::InsertTab,
                KeyCode::Esc => Command::ChangeMode(Mode::Normal),
                KeyCode::Right => Command::MoveCursor { dx: 1, dy: 0 },
                KeyCode::Left => Command::MoveCursor { dx: -1, dy: 0 },
                KeyCode::Up => Command::MoveCursor { dx: 0, dy: -1 },
                KeyCode::Down => Command::MoveCursor { dx: 0, dy: 1 },
                _ => Command::DoNothing,
            },
            Event::Mouse(mouse) => match mouse.kind {
                MouseEventKind::ScrollUp => Command::ScrollUp,
                MouseEventKind::ScrollDown => Command::ScrollDown,
                MouseEventKind::Down(_) => Command::MoveCursorToMouse {
                    row: mouse.row as usize,
                    col: mouse.column as usize,
                },
                _ => Command::DoNothing,
            },
            _ => Command::DoNothing,
        }
    }

    fn handle_normal_event(&mut self, event: Event) -> Command {
        match event {
            Event::Key(key) => match key.code {
                KeyCode::Char(ch) => {
                    self.normal_dispatcher.push(ch);
                    self.normal_dispatcher.get().unwrap_or(Command::DoNothing)
                }
                KeyCode::Esc => Command::TerminateApp,
                _ => Command::DoNothing,
            },
            Event::Mouse(mouse) => match mouse.kind {
                MouseEventKind::ScrollUp => Command::ScrollUp,
                MouseEventKind::ScrollDown => Command::ScrollDown,
                MouseEventKind::Down(_) => Command::MoveCursorToMouse {
                    row: mouse.row as usize,
                    col: mouse.column as usize,
                },
                _ => Command::DoNothing,
            },
            _ => Command::DoNothing,
        }
    }

    fn handle_cmd_event(&mut self, event: Event) -> Command {
        match event {
            Event::Key(key) => match key.code {
                KeyCode::Char(ch) => {
                    self.cmd_dispatcher.push(ch);
                    Command::DoNothing
                }
                KeyCode::Enter => self.cmd_dispatcher.get().unwrap_or(Command::DoNothing),
                KeyCode::Esc => Command::ChangeMode(Mode::Normal),

                _ => Command::DoNothing,
            },
            _ => Command::DoNothing,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Command {
    DoNothing,
    MoveCursor { dx: i32, dy: i32 },
    MoveCursorSOF,
    MoveCursorEOF,
    InsertChar(char),
    InsertTab,
    RemoveChar,
    InsertNewLine,
    InsertEmptyLineBelow,
    InsertEmptyLineAbove,
    MoveCursorToLineEnd,
    MoveCursorToMouse { row: usize, col: usize },
    ScrollUp,
    ScrollDown,
    ChangeMode(Mode),
    TerminateApp,
    Save,
    SaveAndRestart,
    OpenFile(String),
    Undo,
}

impl Command {
    pub fn execute(&self, context: &mut Option<Context>) {
        if let Some(context) = context {
            match self {
                Command::DoNothing => {}
                Command::MoveCursor { dx, dy } => {
                    let cursor = &mut context.cursor;
                    let buffer = &mut context.buffer;

                    // Handle horizontal movement
                    if *dx < 0 {
                        let mut clamped_dx = *dx;
                        if clamped_dx.abs() > cursor.col() as i32 {
                            clamped_dx = -1 * cursor.col() as i32;
                        }
                        cursor.move_left(clamped_dx.abs() as usize);
                    } else if *dx > 0 {
                        let mut clamped_dx = *dx;
                        if cursor.col() as i32 + clamped_dx > buffer.len_of(cursor.row()) as i32 {
                            clamped_dx = buffer.len_of(cursor.row()) as i32 - cursor.col() as i32;
                        }

                        cursor.move_right(clamped_dx as usize);
                    }

                    // Handle vertical movement
                    if *dy < 0 {
                        let mut clamped_dy = *dy;
                        if dy.abs() as usize > cursor.row() {
                            clamped_dy = -1 * cursor.row() as i32;
                        }

                        cursor.move_up(clamped_dy.abs() as usize);
                        if cursor.col() > buffer.len_of(cursor.row()) {
                            cursor.set_col(buffer.len_of(cursor.row()));
                        }
                    } else {
                        let mut clamped_dy = *dy;
                        if cursor.row() as i32 + *dy >= buffer.len() as i32 {
                            clamped_dy = buffer.len() as i32 - cursor.row() as i32 - 1;
                        }

                        cursor.move_down(clamped_dy as usize);
                        if cursor.col() > buffer.len_of(cursor.row()) {
                            cursor.set_col(buffer.len_of(cursor.row()));
                        }
                    }

                    context.viewport.update(cursor.row(), buffer.len());

                    context.app_state.set_should_render(true);
                }
                Command::MoveCursorSOF => {
                    context.viewport.offset = 0;
                    context.cursor.set_row(0);
                    context
                        .cursor
                        .set_col(context.buffer.len_of(context.cursor.row()));
                    context.app_state.set_should_render(true);
                }
                Command::MoveCursorEOF => {
                    context.viewport.offset = context.buffer.len() - context.viewport.height;
                    context.cursor.set_row(context.buffer.len() - 1);
                    context
                        .cursor
                        .set_col(context.buffer.len_of(context.cursor.row()));
                    context.app_state.set_should_render(true);
                }
                Command::InsertChar(ch) => {
                    context
                        .buffer
                        .insert_char(context.cursor.row(), context.cursor.col(), *ch);
                    context.cursor.move_right(1);
                    context.app_state.set_should_render(true);
                }
                Command::InsertTab => {
                    context.buffer.insert_string(
                        context.cursor.row(),
                        context.cursor.col(),
                        &String::from("    "),
                    );
                    context.cursor.move_right(4);
                    context.app_state.set_should_render(true);
                }
                Command::RemoveChar => {
                    let cursor = &mut context.cursor;
                    let buffer = &mut context.buffer;

                    if cursor.col() > 0 {
                        buffer.remove_char(cursor.row(), cursor.col() - 1);
                        cursor.move_left(1);
                    } else if cursor.row() > 0 {
                        let next_col = buffer.len_of(cursor.row() - 1);
                        let cur_line = buffer.get(cursor.row()).clone();
                        buffer.insert_string(cursor.row() - 1, next_col, &cur_line);
                        buffer.remove(cursor.row());
                        cursor.move_up(1);
                        cursor.move_to_col(next_col);
                    }
                    context.app_state.set_should_render(true);
                }
                Command::InsertNewLine => {
                    let cursor = &mut context.cursor;
                    let buffer = &mut context.buffer;

                    let rear = buffer.get_string(
                        cursor.row(),
                        cursor.col(),
                        buffer.len_of(cursor.row()) - cursor.col(),
                    );
                    buffer.remove_string(cursor.row(), cursor.col(), rear.len());
                    buffer.insert(cursor.row() + 1, &rear);
                    cursor.move_down(1);
                    cursor.move_to_col(0);
                    context.app_state.set_should_render(true);
                }
                Command::InsertEmptyLineBelow => {
                    context
                        .buffer
                        .insert(context.cursor.row() + 1, &String::new());
                    context.cursor.move_down(1);
                    context.cursor.move_to_col(0);
                    context.app_state.set_mode(Mode::Edit);
                    context.cursor.set_style(CursorStyle::Bar);
                    context.app_state.set_should_render(true);
                }
                Command::InsertEmptyLineAbove => {
                    context.buffer.insert(context.cursor.row(), &String::new());
                    context.cursor.move_to_col(0);
                    context.app_state.set_mode(Mode::Edit);
                    context.cursor.set_style(CursorStyle::Bar);
                    context.app_state.set_should_render(true);
                }
                Command::MoveCursorToLineEnd => {
                    context
                        .cursor
                        .move_to_col(context.buffer.len_of(context.cursor.row()));
                    context.app_state.set_mode(Mode::Edit);
                    context.cursor.set_style(CursorStyle::Bar);
                    context.app_state.set_should_render(true);
                }
                Command::MoveCursorToMouse { row, col } => {
                    let line_num_len = (context.buffer.len() - 1).ilog10() as usize + 1;

                    let actual_col = if *col <= line_num_len {
                        0
                    } else if *col - line_num_len - 1
                        > context.buffer.len_of(*row + context.viewport.offset)
                    {
                        context.buffer.len_of(*row + context.viewport.offset)
                    } else {
                        *col - line_num_len - 1
                    };

                    context
                        .cursor
                        .move_to(*row + context.viewport.offset, actual_col);
                    context.app_state.set_should_render(true);
                }
                Command::ScrollUp => {
                    if context.viewport.offset > 0 {
                        context.viewport.offset -= 1;
                        context.cursor.move_up(1);
                        if context.cursor.col() > context.buffer.len_of(context.cursor.row()) {
                            context
                                .cursor
                                .move_to_col(context.buffer.len_of(context.cursor.row()));
                        }
                    }
                    context.app_state.set_should_render(true);
                }
                Command::ScrollDown => {
                    if context.viewport.offset + context.viewport.height < context.buffer.len() {
                        context.viewport.offset += 1;
                        context.cursor.move_down(1);
                    }
                    if context.cursor.col() > context.buffer.len_of(context.cursor.row()) {
                        context
                            .cursor
                            .move_to_col(context.buffer.len_of(context.cursor.row()));
                    }
                    context.app_state.set_should_render(true);
                }
                Command::ChangeMode(mode) => {
                    context.app_state.set_mode(*mode);
                    match mode {
                        Mode::Cmd | Mode::Normal => context.cursor.set_style(CursorStyle::Block),
                        Mode::Edit => context.cursor.set_style(CursorStyle::Bar),
                    }
                    context.app_state.set_should_render(true);
                }
                Command::TerminateApp => {
                    context.app_state.terminate_app();
                }
                Command::Save => {
                    let f_write = File::create(&context.file_name).unwrap();
                    let mut buf_writer = BufWriter::new(f_write);
                    for i in 0..context.buffer.len() {
                        buf_writer
                            .write_all(context.buffer.get(i).as_bytes())
                            .unwrap();
                        buf_writer.write_all(b"\n").unwrap();
                    }
                    buf_writer.flush().unwrap();
                    context.app_state.set_mode(Mode::Normal);
                    context.app_state.set_should_render(true);
                }
                Command::SaveAndRestart => {
                    // Save the file first
                    let f_write = File::create(&context.file_name).unwrap();
                    let mut buf_writer = BufWriter::new(f_write);
                    for i in 0..context.buffer.len() {
                        buf_writer
                            .write_all(context.buffer.get(i).as_bytes())
                            .unwrap();
                        buf_writer.write_all(b"\n").unwrap();
                    }
                    buf_writer.flush().unwrap();

                    let _ = crossterm::terminal::disable_raw_mode();
                    let _ = crossterm::execute!(
                        std::io::stdout(),
                        crossterm::event::DisableMouseCapture,
                        crossterm::terminal::LeaveAlternateScreen
                    );

                    let build_status = ProcessCommand::new("cargo")
                        .args(["build"])
                        .status()
                        .expect("Failed to run cargo build");

                    if !build_status.success() {
                        println!("Build failed! Press Enter to return to editor...");
                        let _ = std::io::stdin().read_line(&mut String::new());

                        let args: Vec<String> = env::args().collect();
                        let exe = &args[0];
                        let err = ProcessCommand::new(exe).args(&args[1..]).exec();
                        panic!("Failed to restart: {}", err);
                    }

                    let args: Vec<String> = env::args().collect();
                    let exe = &args[0];
                    let err = ProcessCommand::new(exe).args(&args[1..]).exec();
                    panic!("Failed to restart: {}", err);
                }
                Command::OpenFile(file) => {
                    log!("{file}");
                    context.file_name.clear();
                    context.file_name.push_str(file);
                    context.buffer.replace(file);
                    context.cursor.move_to(0, 0);
                    context.viewport.offset = 0;
                    context.app_state.set_should_render(true);
                    context.app_state.set_mode(Mode::Normal);
                }
                Command::Undo => {}
            }
        }
    }
}
