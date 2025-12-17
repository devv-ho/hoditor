use crate::{input_handler::Command, log};
use std::collections::HashMap;

pub struct CmdDispatcher {
    root: CmdNode,
    query: String,
}

impl CmdDispatcher {
    pub fn new() -> Self {
        Self {
            root: CmdNode::default(),
            query: String::new(),
        }
    }

    pub fn register(&mut self, sequence: &str, command: Command) {
        self.root.insert(sequence.chars(), command);
    }

    pub fn push(&mut self, ch: char) {
        self.query.push(ch);
    }

    pub fn get(&mut self) -> Option<Command> {
        log!("[CD][get] query:{{ {} }}", self.query);
        let query = self.query.clone();
        let query: Vec<_> = query.split(' ').collect();
        let cmd = query[0].to_string();

        let mut digits_end = 0;
        for (i, ch) in cmd.char_indices() {
            if !ch.is_ascii_digit() {
                break;
            }
            digits_end = i + 1;
        }

        let (digits, cmd) = if digits_end > 0 {
            cmd.split_at(digits_end)
        } else {
            ("1", cmd.as_str())
        };

        log!(
            "[CD][get] digits_end:{}, digits:{}, cmd:{}",
            digits_end,
            &digits,
            &cmd
        );

        if cmd.is_empty() {
            log!("Cmd Not Completed");
            return Some(Command::DoNothing);
        }

        let digits = digits.parse::<usize>().unwrap();

        log!("cmd: {}, query:{{{:?}}}", &cmd, &query);

        let result = self.root.find(&cmd);
        log!("{:?}", result);
        match result {
            CmdFindResult::Invalid => {
                self.clear();
                None
            }
            CmdFindResult::Incomplete => None,
            CmdFindResult::Complete(cmd) => {
                self.clear();
                match cmd {
                    Command::OpenFile(_) => Some(Command::OpenFile(query[1].to_string())),
                    Command::MoveCursor { dx, dy } => Some(Command::MoveCursor {
                        dx: dx * digits as i32,
                        dy: dy * digits as i32,
                    }),
                    _ => Some(cmd),
                }
            }
        }
    }

    pub fn get_query(&self) -> String {
        self.query.clone()
    }

    pub fn clear(&mut self) {
        self.query.clear();
    }
}

#[derive(Debug)]
enum CmdFindResult {
    Invalid,
    Incomplete,
    Complete(Command),
}

#[derive(Default)]
struct CmdNode {
    children: HashMap<char, CmdNode>,
    cmd: Option<Command>,
}

impl CmdNode {
    fn insert(&mut self, mut chars: impl Iterator<Item = char>, command: Command) {
        match chars.next() {
            Some(ch) => self.children.entry(ch).or_default().insert(chars, command),
            None => self.cmd = Some(command),
        }
    }

    fn find(&self, query: &str) -> CmdFindResult {
        log!("{query}");
        if query == "" {
            return CmdFindResult::Invalid;
        }

        let mut node = self;
        for ch in query.chars() {
            if let Some(next_node) = node.children.get(&ch) {
                node = next_node;
            } else {
                return CmdFindResult::Invalid;
            }
        }

        log!("{:?}", node.cmd);

        match node.cmd.clone() {
            Some(cmd) => CmdFindResult::Complete(cmd),
            None => CmdFindResult::Incomplete,
        }
    }
}
