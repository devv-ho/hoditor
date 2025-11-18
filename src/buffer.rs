use std::{fs::File, io::BufReader, io::prelude::*, path::Path};

pub struct Buffer {
    lines: Vec<String>,
}

impl Buffer {
    pub fn from(file_path: &str) -> Self {
        println!("{file_path}");
        let f = File::open(file_path).expect("file not found");

        let buf_reader = BufReader::new(&f);
        let mut buffer: Vec<String> = buf_reader.lines().map(|line| line.unwrap()).collect();
        if buffer.is_empty() {
            buffer.push(String::new());
        }

        Self { lines: buffer }
    }

    pub fn len(&self) -> usize {
        self.lines.len()
    }

    pub fn len_of(&self, row: usize) -> usize {
        if row >= self.lines.len() {
            panic!(
                "row out-of-bound. [ buffer_len:{}, row:{} ]",
                self.lines.len(),
                row
            );
        }

        self.lines[row].len()
    }

    pub fn get(&self, row: usize) -> &String {
        if row >= self.lines.len() {
            panic!(
                "row out-of-bound. [ buffer_len:{}, row:{} ]",
                self.lines.len(),
                row
            );
        }

        &self.lines[row]
    }

    pub fn get_string(&self, row: usize, col: usize, size: usize) -> String {
        if row >= self.lines.len() {
            panic!(
                "row out-of-bound. [ buffer_len:{}, row:{} ]",
                self.lines.len(),
                row
            );
        } else if col >= self.lines[row].len() {
            panic!(
                "col out-of-bound. [ line_len:{}, col:{} ]",
                self.lines[row].len(),
                col
            );
        } else if col + size >= self.lines[row].len() {
            panic!(
                "size out-of-bound. [ line_len:{}, col:{}, size:{} ]",
                self.lines[row].len(),
                col,
                size,
            )
        }

        let (_, target) = self.lines[row].split_at(col);
        let (target, _) = target.split_at(size);

        target.to_string()
    }

    pub fn insert(&mut self, row: usize, string: &String) {
        if row >= self.lines.len() {
            panic!(
                "row out-of-bound. [ buffer_len:{}, row:{} ]",
                self.lines.len(),
                row
            );
        }

        self.lines.insert(row, string.clone());
    }

    pub fn insert_char(&mut self, row: usize, col: usize, ch: char) {
        if row >= self.lines.len() {
            panic!(
                "row out-of-bound. [ buffer_len:{}, row:{} ]",
                self.lines.len(),
                row
            );
        } else if col > self.lines[row].len() {
            panic!(
                "col out-of-bound. [ line_len:{}, col:{} ]",
                self.lines[row].len(),
                col
            );
        }

        self.lines[row].insert(col, ch);
    }

    pub fn insert_string(&mut self, row: usize, col: usize, string: &String) {
        if row >= self.lines.len() {
            panic!(
                "row out-of-bound. [ buffer_len:{}, row:{} ]",
                self.lines.len(),
                row
            );
        } else if col > self.lines[row].len() {
            panic!(
                "col out-of-bound. [ line_len:{}, col:{} ]",
                self.lines[row].len(),
                col
            );
        }

        let (front, rear) = self.lines[row].split_at(col);

        self.lines[row] = front.to_string() + string + rear;
    }

    pub fn remove(&mut self, row: usize) {
        if row >= self.lines.len() {
            panic!(
                "row out-of-bound. [ buffer_len:{}, row:{} ]",
                self.lines.len(),
                row
            );
        }

        self.lines.remove(row);
    }

    pub fn remove_char(&mut self, row: usize, col: usize) {
        if row >= self.lines.len() {
            panic!(
                "row out-of-bound. [ buffer_len:{}, row:{} ]",
                self.lines.len(),
                row
            );
        } else if col >= self.lines[row].len() {
            panic!(
                "col out-of-bound. [ line_len:{}, col:{} ]",
                self.lines[row].len(),
                col
            );
        }

        self.lines[row].remove(col);
    }

    pub fn remove_string(&mut self, row: usize, col: usize, size: usize) {
        if row >= self.lines.len() {
            panic!(
                "row out-of-bound. [ buffer_len:{}, row:{} ]",
                self.lines.len(),
                row
            );
        } else if col >= self.lines[row].len() {
            panic!(
                "col out-of-bound. [ line_len:{}, col:{} ]",
                self.lines[row].len(),
                col
            );
        } else if col + size >= self.lines[row].len() {
            panic!(
                "size out-of-bound. [ line_len:{}, col:{}, size:{} ]",
                self.lines[row].len(),
                col,
                size,
            )
        }

        let (front, tmp_rear) = self.lines[row].split_at(col);
        let (_, rear) = tmp_rear.split_at(size);

        self.lines[row] = front.to_string() + rear;
    }
}
