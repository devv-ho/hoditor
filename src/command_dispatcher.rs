use crate::input_handler::Command;
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
        let result = self.root.find(&self.query);
        if result.is_some() {
            self.clear();
        }
        result
    }

    pub fn clear(&mut self) {
        self.query.clear();
    }
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

    fn find(&self, query: &str) -> Option<Command> {
        let mut node = self;
        for ch in query.chars() {
            node = node.children.get(&ch)?;
        }
        node.cmd.clone()
    }
}

#[cfg(test)]
mod test {
    use crate::command_dispatcher::CmdDispatcher;
    use crate::input_handler::Command;

    #[test]
    fn register_cmd() {
        let mut dispatcher = CmdDispatcher::new();
        dispatcher.register("abc", Command::DoNothing);

        dispatcher.push('a');
        assert!(dispatcher.get().is_none());

        dispatcher.push('b');
        assert!(dispatcher.get().is_none());

        dispatcher.push('c');
        assert!(dispatcher.get().is_some());
    }
}
