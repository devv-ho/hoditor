use crate::highlighter::{Highlighter, style_to_crossterm_color};
use crate::logger::Logger;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute, queue,
    style::{Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal,
};
use std::{
    env,
    fs::File,
    io::prelude::*,
    io::{BufReader, Write, stdout},
    time::Duration,
};

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

    let debug_win_size = 5;
    let debug_win_offset = win_size.1 - debug_win_size;
    let debug_mode_idx = debug_win_offset + 1;
    let debug_cursor_idx = debug_mode_idx + 1;

    let col_start = (line_num_width + 2) as u16;

    let (mut cur_row, mut cur_col) = (0, 0);

    execute!(stdout, cursor::MoveTo(col_start, 0))?;

    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    // Extract file extension for syntax highlighting
    let extension = std::path::Path::new(filename)
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("txt");

    let highlighter = Highlighter::new();

    let f = File::open(filename).expect("file not found");

    let buf_reader = BufReader::new(f);
    let mut text_content: Vec<String> = buf_reader.lines().map(|line| line.unwrap()).collect();

    let mut view_port_offset = 0;
    queue!(stdout, cursor::MoveTo(0, 0))?;
    for i in view_port_offset..(view_port_offset + win_size.1) {
        queue!(
            stdout,
            terminal::Clear(terminal::ClearType::CurrentLine),
            Print(format!("{:>width$}", i, width = line_num_width as usize)),
            cursor::MoveDown(1),
            cursor::MoveToColumn(0),
        )?;
    }

    queue!(stdout, cursor::MoveTo(col_start, 0))?;
    for i in view_port_offset..(view_port_offset + win_size.1) {
        if i as usize >= text_content.len() {
            break;
        }

        // Render line with syntax highlighting
        let line = &text_content[i as usize];
        let highlighted = highlighter.highlight_line(line, extension);

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
            cursor::MoveDown(1),
            cursor::MoveToColumn(col_start)
        )?;
    }

    match mode {
        Mode::Cmd => {
            queue!(stdout, cursor::SetCursorStyle::SteadyBlock)?;
        }
        Mode::Edit => {
            queue!(stdout, cursor::SetCursorStyle::SteadyBar)?;
        }
    }

    queue!(
        stdout,
        cursor::MoveTo(col_start, debug_win_offset),
        Print("=".repeat((win_size.0 - col_start) as usize)),
        cursor::MoveTo(col_start, debug_mode_idx),
        Print(format!("Mode : {:?}", mode)),
        cursor::MoveTo(col_start, debug_cursor_idx),
        Print(format!(
            "Cursor (terminal) : ({}, {})",
            cur_row,
            cur_col + col_start
        )),
        cursor::MoveDown(1),
        cursor::MoveToColumn(col_start),
        Print(format!(
            "Cursor (file) : ({}, {})",
            cur_row + view_port_offset,
            cur_col
        ))
    )?;
    stdout.flush()?;

    execute!(stdout, cursor::MoveTo(cur_col + col_start, cur_row as u16))?;

    let mut should_break = false;

    loop {
        let mut render = false;
        if event::poll(Duration::from_millis(10))? {
            let event = event::read()?;

            // Process Event
            match event {
                Event::Key(key) => {
                    Logger::log(format!("[main] Key Read. {:?}", key.code))?;
                    match mode {
                        Mode::Cmd => match key.code {
                            KeyCode::Char(c) => {
                                if c == 'i' {
                                    Logger::log(format!(
                                        "[main] Change Mode. from {:?} to {:?}",
                                        Mode::Cmd,
                                        Mode::Edit,
                                    ))?;

                                    mode = Mode::Edit;
                                } else if c == 'h' {
                                    if cur_col > 0 {
                                        cur_col -= 1;
                                    }
                                } else if c == 'j' {
                                    if cur_row + view_port_offset < text_content.len() as u16 - 1 {
                                        if cur_row < debug_win_offset - 1 {
                                            cur_row += 1;

                                            if (text_content[(cur_row + view_port_offset) as usize]
                                                .len()
                                                as u16)
                                                < cur_col as u16
                                            {
                                                cur_col = text_content
                                                    [(cur_row + view_port_offset) as usize]
                                                    .len()
                                                    as u16;
                                            }
                                        } else {
                                            view_port_offset += 1;
                                            render = true;
                                        }
                                    } else if (view_port_offset as usize) < text_content.len() - 1 {
                                        view_port_offset += 1;
                                        render = true;
                                    }
                                } else if c == 'k' {
                                    if cur_row > 0 {
                                        cur_row -= 1;

                                        if text_content[(cur_row + view_port_offset) as usize].len()
                                            < cur_col as usize
                                        {
                                            cur_col = text_content
                                                [(cur_row + view_port_offset) as usize]
                                                .len()
                                                as u16;
                                        }
                                    } else if view_port_offset > 0 {
                                        view_port_offset -= 1;
                                        render = true;
                                    }
                                } else if c == 'l' {
                                    if text_content[(cur_row + view_port_offset) as usize].len() > 0
                                        && cur_col
                                            < text_content[(cur_row + view_port_offset) as usize]
                                                .len()
                                                as u16
                                    {
                                        cur_col += 1;
                                    }
                                }
                            }

                            KeyCode::Esc => {
                                Logger::log(String::from("[main] Break App Loop"))?;
                                should_break = true;
                            }

                            _ => {}
                        },
                        Mode::Edit => {
                            render = true;
                            match key.code {
                                KeyCode::Char(c) => {
                                    text_content[(cur_row + view_port_offset) as usize]
                                        .insert(cur_col as usize, c);
                                    cur_col += 1;
                                }

                                KeyCode::Backspace => {
                                    if cur_col > 0 {
                                        text_content[(cur_row + view_port_offset) as usize]
                                            .remove((cur_col - 1) as usize);
                                        cur_col -= 1;
                                    } else if (cur_row + view_port_offset) > 0 {
                                        let upper_line = text_content
                                            [(cur_row + view_port_offset) as usize - 1]
                                            .clone();

                                        let next_col = upper_line.len();

                                        text_content[(cur_row + view_port_offset) as usize - 1] =
                                            upper_line
                                                + text_content
                                                    [(cur_row + view_port_offset) as usize]
                                                    .as_str();

                                        text_content.remove((cur_row + view_port_offset) as usize);

                                        cur_row -= 1;
                                        cur_col = next_col as u16;
                                    }
                                }

                                KeyCode::Enter => {
                                    let new_line_text = text_content
                                        [(cur_row + view_port_offset) as usize]
                                        .split_off(cur_col as usize);
                                    text_content.insert(
                                        (cur_row + view_port_offset) as usize + 1,
                                        new_line_text,
                                    );

                                    if cur_row < debug_win_offset - 1 {
                                        cur_row += 1;
                                        cur_col = 0;
                                    } else {
                                        view_port_offset += 1;
                                        cur_col = 0;
                                    }
                                }

                                KeyCode::Esc => {
                                    Logger::log(format!(
                                        "[main] Change Mode. from {:?} to {:?}",
                                        Mode::Edit,
                                        Mode::Cmd,
                                    ))?;

                                    mode = Mode::Cmd;
                                }

                                _ => {}
                            }
                        }
                    }
                }
                _ => {
                    Logger::log(format!("[main] Unknown Event Read. {:?}", event))?;
                }
            }
        }

        if render {
            queue!(stdout, cursor::MoveTo(0, 0))?;
            for i in view_port_offset..(view_port_offset + win_size.1) {
                queue!(
                    stdout,
                    terminal::Clear(terminal::ClearType::CurrentLine),
                    Print(format!("{:>width$}", i, width = line_num_width as usize)),
                    cursor::MoveDown(1),
                    cursor::MoveToColumn(0),
                )?;
            }
            queue!(stdout, cursor::MoveTo(col_start, 0))?;
            for i in view_port_offset..(view_port_offset + debug_win_offset) {
                if i as usize >= text_content.len() {
                    break;
                }

                // Render line with syntax highlighting
                let line = &text_content[i as usize];
                let highlighted = highlighter.highlight_line(line, extension);

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
                    cursor::MoveDown(1),
                    cursor::MoveToColumn(col_start)
                )?;
            }

            match mode {
                Mode::Cmd => {
                    queue!(stdout, cursor::SetCursorStyle::SteadyBlock)?;
                }
                Mode::Edit => {
                    queue!(stdout, cursor::SetCursorStyle::SteadyBar)?;
                }
            }

            queue!(
                stdout,
                cursor::MoveTo(col_start, debug_win_offset),
                Print("=".repeat((win_size.0 - col_start) as usize)),
                cursor::MoveTo(col_start, debug_mode_idx),
                Print(format!("Mode : {:?}", mode)),
                cursor::MoveTo(col_start, debug_cursor_idx),
                Print(format!("Cursor : ({}, {})", cur_row, cur_col + col_start)),
                cursor::MoveDown(1),
                cursor::MoveToColumn(col_start),
                Print(format!(
                    "Cursor (file) : ({}, {})",
                    cur_row + view_port_offset,
                    cur_col
                ))
            )?;
            stdout.flush()?;
        }

        execute!(stdout, cursor::MoveTo(cur_col + col_start, cur_row as u16))?;

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

    Logger::log(String::from("[main] Terminate App"))?;

    Ok(())
}
