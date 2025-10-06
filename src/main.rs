use crossterm::event::{self, Event, KeyCode};
use crossterm::{
    cursor, execute,
    terminal::{self, Clear, ClearType},
};
use std::{
    env, fs,
    io::{Write, stdout},
    sync::mpsc,
    thread,
};

fn main() {
    execute!(stdout(), Clear(ClearType::All)).unwrap();
    execute!(stdout(), cursor::EnableBlinking).unwrap();
    terminal::enable_raw_mode().unwrap();

    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let contents = fs::read_to_string(file_path).unwrap();

    let contents: Vec<_> = contents.lines().map(String::from).collect();

    execute!(stdout(), cursor::MoveTo(0, 0), cursor::Show).unwrap();
    contents.iter().for_each(|line| println!("\r{line}"));
    execute!(stdout(), cursor::MoveTo(0, 0), cursor::Show).unwrap();
    let (tx, rx) = mpsc::channel();

    // thread 1: producer
    let producer = thread::spawn(move || {
        loop {
            if let Ok(Event::Key(key)) = event::read() {
                tx.send(key.code).unwrap();
                if key.code == KeyCode::Esc {
                    break;
                }
            }
        }
    });

    // thread 2: consumer
    let consumer = thread::spawn(move || {
        let mut lines = contents.clone();
        let (mut row, mut col): (usize, usize) = (0, 0);
        lines.push(String::new());

        for code in rx {
            match code {
                KeyCode::Esc => break,

                KeyCode::Char(c) => {
                    if let Some(line) = lines.get_mut(row as usize) {
                        line.push(c);
                        print!("\r{}", *line);
                        stdout().flush().unwrap();
                        col += 1;
                    } else {
                        println!("None!");
                    }
                }

                KeyCode::Enter => {
                    lines.push(String::new());
                    println!("\r");
                    let _ = stdout().flush();
                    col = 0;
                    row += 1;
                }

                KeyCode::Backspace => {
                    if lines[row].is_empty() && lines.len() > 1 {
                        lines.remove(row);
                        row -= 1;
                        col = lines[row].len();
                        execute!(stdout(), cursor::MoveTo(col as u16, row as u16)).unwrap();
                    } else {
                        lines[row].pop();
                        execute!(stdout(), Clear(ClearType::CurrentLine)).unwrap();
                        print!("\r{}", lines[row]);
                        stdout().flush().unwrap();
                        if col > 0 {
                            col -= 1;
                        }
                    }
                }

                _ => {}
            }
        }
        lines
    });

    // wait for threads
    let final_lines = consumer.join().unwrap();
    let _ = producer.join();

    // 🔑 cleanup: always restore terminal state
    terminal::disable_raw_mode().unwrap();
    execute!(stdout(), cursor::Show).unwrap();
    println!(); // ✅ 확실히 줄 바꿔주기

    // Save to file
    let content = final_lines.join("\n");
    fs::write(file_path, content).unwrap();
}
