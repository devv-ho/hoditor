use crossterm::event as TerminalEvent;
use crossterm::event::KeyCode;
use crossterm::execute;
use std::io::Read;
use std::{fs::File, path};

fn main() -> std::io::Result<()> {
    let file_path = path::Path::new("/Users/apple/dev/projects/hoditor/src/main.rs");
    let mut content = String::new();
    File::open(file_path)
        .unwrap()
        .read_to_string(&mut content)
        .unwrap();

    let mut content: Vec<String> = content.lines().map(String::from).collect();

    crossterm::terminal::enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, crossterm::terminal::EnterAlternateScreen)?;
    execute!(stdout, crossterm::cursor::Show)?;

    run(&mut std::io::stdout(), &mut content).unwrap();

    // std::fs::write(file_path, content.join("\n")).unwrap();

    execute!(stdout, crossterm::terminal::LeaveAlternateScreen)?;
    crossterm::terminal::disable_raw_mode()?;
    Ok(())
}

fn run<W: std::io::Write>(screen: &mut W, content: &mut Vec<String>) -> std::io::Result<()> {
    let mut running = true;

    let mut mode = Mode::Cmd;
    let mut cursor_pos = Pos::new(0, 0);
    let mut visible_start = 0usize;

    while running {
        render(screen, content, &cursor_pos, &mode, visible_start).unwrap();
        running = process_input(&mut mode, &mut cursor_pos, content, &mut visible_start).unwrap();
    }

    Ok(())
}

#[derive(Debug)]
enum Mode {
    Edit,
    Cmd,
}

struct Pos {
    pub row: usize,
    pub col: usize,
}

impl Pos {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

fn process_input(
    mode: &mut Mode,
    cursor: &mut Pos,
    content: &mut Vec<String>,
    visible_start: &mut usize,
) -> std::io::Result<bool> {
    if let TerminalEvent::Event::Key(key) = TerminalEvent::read()? {
        match mode {
            Mode::Edit => edit(mode, content, key.code, cursor, visible_start),
            Mode::Cmd => {
                if perform_cmd(mode, cursor, key.code, content, visible_start) == false {
                    return Ok(false);
                }
            }
        }
    }

    Ok(true)
}

fn edit(
    mode: &mut Mode,
    content: &mut Vec<String>,
    key: KeyCode,
    cursor: &mut Pos,
    visible_start: &mut usize,
) {
    match key {
        KeyCode::Esc => *mode = Mode::Cmd,

        KeyCode::Char(ch) => {
            content.get_mut(cursor.row).unwrap().insert(cursor.col, ch);
            cursor.col += 1;
        }

        KeyCode::Backspace => {
            if cursor.col == 0 {
                if cursor.row > 0 {
                    content[cursor.row - 1] =
                        format!("{}{}", content[cursor.row - 1], content[cursor.row]);
                    content.remove(cursor.row);

                    cursor.row -= 1;
                    cursor.col = content[cursor.row].len();
                }
            } else {
                content.get_mut(cursor.row).unwrap().remove(cursor.col - 1);
                cursor.col -= 1;
            }
        }

        KeyCode::Enter => {
            let cur_line = content.get_mut(cursor.row).unwrap();
            let new_line = cur_line[cursor.col..].to_string();

            content[cursor.row] = cur_line[..cursor.col].to_string();
            content.insert(cursor.row + 1, new_line);

            cursor.col = 0;
            cursor.row += 1;
        }

        KeyCode::Up => {
            if cursor.row == 0 {
                if *visible_start > 0 {
                    *visible_start -= 1;
                }
            } else if let Some(above_line) = content.get(cursor.row - 1) {
                cursor.row -= 1;

                if cursor.col > above_line.len() {
                    cursor.col = above_line.len();
                }
            }
        }

        KeyCode::Down => {
            if let Some(below_line) = content.get(cursor.row + 1) {
                cursor.row += 1;

                if cursor.col > below_line.len() {
                    cursor.col = below_line.len();
                }
            }

            if cursor.row > 49 {
                cursor.row = 49;
            }
        }

        KeyCode::Left => {
            if cursor.col > 0 {
                cursor.col -= 1;
            }
        }

        KeyCode::Right => {
            if let Some(current_line) = content.get(cursor.row) {
                if cursor.col < current_line.len() {
                    cursor.col += 1;
                }
            }
        }
        _ => {}
    }
}

fn perform_cmd(
    mode: &mut Mode,
    cursor: &mut Pos,
    key: KeyCode,
    content: &mut Vec<String>,
    visible_start: &mut usize,
) -> bool {
    match key {
        KeyCode::Char(ch) => match ch {
            'i' => {
                *mode = Mode::Edit;
            }

            'h' => {
                if cursor.col > 0 {
                    cursor.col -= 1;
                }
            }

            'j' => {
                if let Some(below_line) = content.get(cursor.row + 1) {
                    cursor.row += 1;

                    if cursor.col > below_line.len() {
                        cursor.col = below_line.len();
                    }
                }

                if cursor.row > 49 {
                    cursor.row = 49;
                    if (*visible_start < content.len()) {
                        *visible_start += 1usize;
                    }
                }
            }

            'k' => {
                if cursor.row == 0 {
                    if *visible_start > 0 {
                        *visible_start -= 1;
                    }
                } else if let Some(above_line) = content.get(cursor.row - 1) {
                    cursor.row -= 1;

                    if cursor.col > above_line.len() {
                        cursor.col = above_line.len();
                    }
                }
            }

            'l' => {
                if let Some(current_line) = content.get(cursor.row) {
                    if cursor.col < current_line.len() {
                        cursor.col += 1;
                    }
                }
            }

            'q' => return false,

            _ => {}
        },

        KeyCode::Up => {
            if cursor.row > 0
                && let Some(above_line) = content.get(cursor.row - 1)
            {
                cursor.row -= 1;

                if cursor.col > above_line.len() {
                    cursor.col = above_line.len();
                }
            }
        }

        KeyCode::Down => {
            if let Some(below_line) = content.get(cursor.row + 1) {
                cursor.row += 1;

                if cursor.col > below_line.len() {
                    cursor.col = below_line.len();
                }
            }

            if cursor.row > 49 {
                cursor.row = 49;
            }
        }

        KeyCode::Left => {
            if cursor.col > 0 {
                cursor.col -= 1;
            }
        }

        KeyCode::Right => {
            if let Some(current_line) = content.get(cursor.row) {
                if cursor.col < current_line.len() {
                    cursor.col += 1;
                }
            }
        }

        _ => {}
    }

    true
}

fn render<W: std::io::Write>(
    screen: &mut W,
    content: &mut Vec<String>,
    cursor: &Pos,
    mode: &Mode,
    visible_start: usize,
) -> std::io::Result<()> {
    crossterm::execute!(screen, crossterm::cursor::MoveTo(0, 0))?;
    crossterm::execute!(
        screen,
        crossterm::terminal::Clear(crossterm::terminal::ClearType::All)
    )?;

    match mode {
        Mode::Edit => {
            crossterm::execute!(screen, crossterm::cursor::SetCursorStyle::SteadyBar)?;
        }

        Mode::Cmd => {
            crossterm::execute!(screen, crossterm::cursor::SetCursorStyle::SteadyBlock)?;
        }
    }

    let window_size = crossterm::terminal::size().unwrap();
    let visible_end = visible_start + (window_size.1 as usize) - 10;

    content
        .iter()
        .enumerate()
        .skip(visible_start)
        .take(window_size.1 as usize - 10)
        .for_each(|(line_num, line)| {
            let line_size = std::cmp::min(window_size.0 as usize - 10, line.len());
            println!("\r{line_num}\t {}", &line[..line_size]);
        });

    println!("\rCursor: ({}, {})", cursor.row, cursor.col);
    println!("\rMode: {:?}", mode);
    println!("\rContent Size: {}", content.len());
    println!("\rWindow Size: ({}, {})", window_size.0, window_size.1);

    crossterm::execute!(
        screen,
        crossterm::cursor::MoveTo(cursor.col as u16 + 9, cursor.row as u16)
    )?;

    Ok(())
}
