#[derive(Debug, Clone, Copy)]
pub enum CursorStyle {
    Block,
    Bar,
}

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct Cursor {
    pos: Position,
    style: CursorStyle,
}

pub const SCROLL_HEIGHT: usize = 15;
impl Cursor {
    pub fn new() -> Self {
        Self {
            pos: Position { row: 0, col: 0 },
            style: CursorStyle::Block,
        }
    }

    pub fn pos(&self) -> Position {
        self.pos
    }

    pub fn row(&self) -> usize {
        self.pos.row
    }

    pub fn col(&self) -> usize {
        self.pos.col
    }

    pub fn set_row(&mut self, row: usize) {
        self.pos.row = row;
    }

    pub fn set_col(&mut self, col: usize) {
        self.pos.col = col;
    }

    pub fn move_to(&mut self, row: usize, col: usize) {
        self.pos.row = row;
        self.pos.col = col;
    }

    pub fn move_down(&mut self, n: usize) {
        self.pos.row += n;
    }

    pub fn move_up(&mut self, n: usize) {
        if n > self.pos.row {
            panic!("row out-of-bound. [ row:{}, n:{} ]", self.pos.row, n);
        }

        self.pos.row -= n;
    }

    pub fn move_left(&mut self, n: usize) {
        if n > self.pos.col {
            panic!("row out-of-bound. [ row:{}, n:{} ]", self.pos.row, n);
        }

        self.pos.col -= n;
    }

    pub fn move_right(&mut self, n: usize) {
        self.pos.col += n;
    }

    pub fn move_to_col(&mut self, col: usize) {
        self.pos.col = col;
    }

    pub fn set_style(&mut self, style: CursorStyle) {
        self.style = style;
    }

    pub fn style(&self) -> CursorStyle {
        self.style
    }
}
