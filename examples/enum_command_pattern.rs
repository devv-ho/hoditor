// Example: Command Pattern with Enums
// This shows how to use enums instead of trait objects for commands

use std::fmt::Debug;

// The context/state that commands operate on
struct EditorState {
    cursor_x: usize,
    cursor_y: usize,
    content: String,
}

// Commands as an enum - simple, type-safe, no heap allocation
#[derive(Debug, Clone)]
enum Command {
    MoveCursor { dx: i32, dy: i32 },
    InsertChar(char),
    DeleteChar,
    Save,
    Quit,
}

impl Command {
    fn execute(&self, state: &mut EditorState) {
        match self {
            Command::MoveCursor { dx, dy } => {
                state.cursor_x = (state.cursor_x as i32 + dx).max(0) as usize;
                state.cursor_y = (state.cursor_y as i32 + dy).max(0) as usize;
                println!("Moved to ({}, {})", state.cursor_x, state.cursor_y);
            }
            Command::InsertChar(ch) => {
                state.content.push(*ch);
                println!("Inserted '{}'", ch);
            }
            Command::DeleteChar => {
                state.content.pop();
                println!("Deleted char");
            }
            Command::Save => {
                println!("Saved: {}", state.content);
            }
            Command::Quit => {
                println!("Quit");
            }
        }
    }
}

// Dispatcher maps key sequences to commands
struct Dispatcher {
    // Instead of storing trait objects, we store the enum directly
    mappings: Vec<(&'static str, Command)>,
}

impl Dispatcher {
    fn new() -> Self {
        Self {
            mappings: vec![
                ("h", Command::MoveCursor { dx: -1, dy: 0 }),
                ("j", Command::MoveCursor { dx: 0, dy: 1 }),
                ("k", Command::MoveCursor { dx: 0, dy: -1 }),
                ("l", Command::MoveCursor { dx: 1, dy: 0 }),
                ("w", Command::Save),
                ("q", Command::Quit),
            ],
        }
    }

    fn get(&self, key: &str) -> Option<Command> {
        self.mappings
            .iter()
            .find(|(k, _)| *k == key)
            .map(|(_, cmd)| cmd.clone())
    }
}

fn main() {
    let mut state = EditorState {
        cursor_x: 0,
        cursor_y: 0,
        content: String::new(),
    };

    let dispatcher = Dispatcher::new();

    // Simulate key presses
    let keys = vec!["h", "j", "w"];

    for key in keys {
        if let Some(cmd) = dispatcher.get(key) {
            println!("Executing: {:?}", cmd);
            cmd.execute(&mut state);
        }
    }
}

// Benefits of enum approach:
// 1. No heap allocation (Box)
// 2. Clone is trivial
// 3. Pattern matching is exhaustive
// 4. Easy to serialize/deserialize
// 5. Can store data in variants
// 6. No trait objects = no vtable overhead
