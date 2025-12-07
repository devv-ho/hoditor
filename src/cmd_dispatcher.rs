use crate::{input_handler::Command, logger::Logger};
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
        let query = self.query.clone();
        let query: Vec<_> = query.split(' ').collect();
        let cmd = query[0].to_string();
        Logger::log(format!("cmd: {}, query:{{{:?}}}", &cmd, &query));
        let result = self.root.find(&cmd);
        Logger::log(format!("{:?}", result));
        match result {
            CmdFindResult::Invalid => {
                self.clear();
                None
            }
            CmdFindResult::Incomplete => None,
            CmdFindResult::Complete(cmd) => {
                self.clear();
                if matches!(cmd, Command::OpenFile(_)) {
                    Some(Command::OpenFile(query[1].to_string()))
                } else {
                    Some(cmd)
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
        Logger::log(format!("{query}"));
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

        Logger::log(format!("{:?}", node.cmd));

        match node.cmd.clone() {
            Some(cmd) => CmdFindResult::Complete(cmd),
            None => CmdFindResult::Incomplete,
        }
    }
}
