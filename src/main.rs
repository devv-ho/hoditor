use crate::buffer::Buffer;
use crate::cursor::Cursor;
use crate::highlighter::{Highlighter, style_to_crossterm_color};
use crate::logger::Logger;

use crossterm::{
    cursor as tui_cursor,
    event::{self, Event, KeyCode},
    execute, queue,
    style::{Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal,
};
use std::ops::ControlFlow;
use std::{
    env,
    fs::File,
    io::{BufWriter, Write, stdout},
    time::Duration,
};

pub mod buffer;
pub mod cursor;
pub mod highlighter;
pub mod logger;

#[derive(Debug, Clone, Copy)]
enum Mode {
    Cmd,
    Edit,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Logger::init()?;

    Logger::log(String::from("[main] Start App"))?;

    let mut mode = Mode::Cmd;
    let mut stdout = stdout();
    terminal::enable_raw_mode()?;
    execute!(stdout, crossterm::terminal::EnterAlternateScreen)?;

    // Set TokyoNight background color for entire terminal
    let tokyonight_bg = crossterm::style::Color::Rgb {
        r: 0x1a,
        g: 0x1b,
        b: 0x26,
    };
    execute!(
        stdout,
        SetBackgroundColor(tokyonight_bg),
        terminal::Clear(terminal::ClearType::All)
    )?;

    let win_size = terminal::size()?;

    Logger::log(format!(
        "[main] Window Size [ width : {}, height : {} ]",
        win_size.1, win_size.0
    ))?;

    let line_num_width = 2u16;
    for i in 0..(win_size.1) {
        queue!(
            stdout,
            Print(format!("{:>width$}", i, width = line_num_width as usize))
        )?;
        if i < win_size.1 - 1 {
            queue!(stdout, Print("\r\n"))?;
        }
    }
    stdout.flush()?;

    let debug_win_size = 5usize;
    let text_win_size = (win_size.1 as usize) - debug_win_size;

    let col_start = (line_num_width + 2) as u16;

    let mut cursor = Cursor::new();

    execute!(stdout, tui_cursor::MoveTo(col_start, 0))?;

    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    // Extract file extension for syntax highlighting
    let extension = std::path::Path::new(filename)
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("txt");

    let mut buffer = Buffer::from(extension);
    let highlighter = Highlighter::new();

    let mut viewport_offset = 0usize;
    queue!(stdout, tui_cursor::MoveTo(0, 0))?;
    for i in viewport_offset..(viewport_offset + (win_size.1 as usize)) {
        queue!(
            stdout,
            terminal::Clear(terminal::ClearType::CurrentLine),
            Print(format!("{:>width$}", i, width = line_num_width as usize)),
            tui_cursor::MoveDown(1),
            tui_cursor::MoveToColumn(0),
        )?;
    }

    queue!(stdout, tui_cursor::MoveTo(col_start, 0))?;
    for i in viewport_offset..(viewport_offset + (win_size.1 as usize)) {
        if i >= buffer.len() {
            break;
        }

        let highlighted = highlighter.highlight_line(buffer.get(i), extension);

        for (style, text) in highlighted {
            let (fg, bg) = style_to_crossterm_color(style);
            queue!(
                stdout,
                SetForegroundColor(fg),
                SetBackgroundColor(bg),
                Print(&text)
            )?;
        }

        queue!(
            stdout,
            SetBackgroundColor(tokyonight_bg),
            tui_cursor::MoveDown(1),
            tui_cursor::MoveToColumn(col_start)
        )?;
    }

    match mode {
        Mode::Cmd => {
            queue!(stdout, tui_cursor::SetCursorStyle::SteadyBlock)?;
        }
        Mode::Edit => {
            queue!(stdout, tui_cursor::SetCursorStyle::SteadyBar)?;
        }
    }

    stdout.flush()?;

    execute!(
        stdout,
        tui_cursor::MoveTo(cursor.row_u16() + col_start, cursor.col_u16())
    )?;

    let mut should_break = false;

    loop {
        let mut render = false;
        if event::poll(Duration::from_millis(10))? {
            let event = event::read()?;

            // Process Event
            match event {
                Event::Key(key) => match mode {
                    Mode::Cmd => match key.code {
                        KeyCode::Char(c) => {
                            if c == 'i' {
                                mode = Mode::Edit;
                            } else if c == 'h' {
                                if cursor.col > 0 {
                                    cursor.move_left(1);
                                }
                            } else if c == 'j' {
                                if cursor.row < buffer.len() - 1 {
                                    cursor.move_down(1);
                                    if cursor.col > buffer.len_of(cursor.row) {
                                        cursor.col = buffer.len_of(cursor.row);
                                    }
                                }
                            } else if c == 'k' {
                                if cursor.row > 0 {
                                    cursor.move_up(1);
                                    if cursor.col > buffer.len_of(cursor.row) {
                                        cursor.col = buffer.len_of(cursor.row);
                                    }
                                }
                            } else if c == 'l' {
                                if cursor.col < buffer.len_of(cursor.row) {
                                    cursor.move_right(1);
                                }
                            }
                        }

                        KeyCode::Esc => {
                            should_break = true;
                        }

                        _ => {}
                    },

                    Mode::Edit => {
                        render = true;
                        match key.code {
                            KeyCode::Char(ch) => {
                                buffer.insert_char(cursor.row, cursor.col, ch);
                                cursor.move_right(1);
                            }

                            KeyCode::Backspace => {
                                if cursor.col > 0 {
                                    buffer.remove_char(cursor.row, cursor.col);
                                    cursor.move_left(1);
                                } else if cursor.row > 0 {
                                    let cur_line = buffer.get(cursor.row).clone();

                                    buffer.insert_string(cursor.row - 1, cur_line.len(), &cur_line);
                                    buffer.remove(cursor.row);

                                    cursor.move_up(1);
                                    cursor.move_to_col(buffer.len_of(cursor.row));
                                }
                            }

                            KeyCode::Enter => {
                                let new_lined_string = buffer.get_string(
                                    cursor.row,
                                    cursor.col,
                                    buffer.len_of(cursor.row) - cursor.col,
                                );

                                buffer.insert(cursor.row + 1, &new_lined_string);

                                buffer.remove_string(
                                    cursor.row,
                                    cursor.col,
                                    buffer.len_of(cursor.row) - cursor.col,
                                );

                                cursor.move_down(1);
                                cursor.move_to_col(0);
                            }

                            KeyCode::Tab => {
                                buffer.insert_string(cursor.row, cursor.col, &String::from("    "));

                                cursor.move_right(4);
                            }

                            KeyCode::Esc => {
                                mode = Mode::Cmd;
                            }

                            _ => {}
                        }
                    }
                },
                _ => {
                    Logger::log(format!("[main] Unknown Event Read. {:?}", event))?;
                }
            }

            if cursor.row >= text_win_size {
                viewport_offset = cursor.row - text_win_size + 1;
                render = true;
            }
        }

        if render {
            queue!(stdout, tui_cursor::Hide)?;

            queue!(stdout, tui_cursor::MoveTo(0, 0))?;
            for i in 0..(win_size.1 as usize) {
                let line_idx = i + viewport_offset;

                queue!(
                    stdout,
                    terminal::Clear(terminal::ClearType::CurrentLine),
                    Print(format!(
                        "{:>width$}",
                        line_idx,
                        width = line_num_width as usize
                    )),
                    tui_cursor::MoveDown(1),
                    tui_cursor::MoveToColumn(0),
                )?;
            }

            queue!(stdout, tui_cursor::MoveTo(col_start, 0))?;
            for i in 0..text_win_size {
                let line_idx = i + viewport_offset;

                if line_idx >= buffer.len() {
                    break;
                }

                let highlighted = highlighter.highlight_line(buffer.get(line_idx), extension);

                for (style, text) in highlighted {
                    let (fg, bg) = style_to_crossterm_color(style);
                    queue!(
                        stdout,
                        SetForegroundColor(fg),
                        SetBackgroundColor(bg),
                        Print(&text)
                    )?;
                }

                queue!(
                    stdout,
                    SetBackgroundColor(tokyonight_bg),
                    tui_cursor::MoveDown(1),
                    tui_cursor::MoveToColumn(col_start)
                )?;
            }

            queue!(stdout, tui_cursor::Show)?;
            stdout.flush()?;
        }

        match mode {
            Mode::Cmd => {
                execute!(stdout, tui_cursor::SetCursorStyle::SteadyBlock)?;
            }
            Mode::Edit => {
                execute!(stdout, tui_cursor::SetCursorStyle::SteadyBar)?;
            }
        }

        execute!(
            stdout,
            tui_cursor::MoveTo(
                (cursor.col - viewport_offset) as u16 + col_start,
                cursor.row as u16
            )
        )?;

        if should_break {
            break;
        }
    }

    terminal::disable_raw_mode()?;
    execute!(
        stdout,
        ResetColor,
        crossterm::terminal::LeaveAlternateScreen
    )?;

    // Write the modified content back to the file
    let f_write = File::create(filename)?;
    let mut buf_writer = BufWriter::new(f_write);
    for i in 0..buffer.len() {
        buf_writer.write_all(buffer.get(i).as_bytes())?;
        buf_writer.write_all(b"\n")?;
    }
    buf_writer.flush()?;

    Logger::log(String::from("[main] Terminate App"))?;

    Ok(())
}
