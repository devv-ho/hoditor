pub struct Cursor {
    pub row: usize,
    pub col: usize,
}

impl Cursor {
    pub fn new() -> Self {
        Self { row: 0, col: 0 }
    }

    pub fn row_u16(&self) -> u16 {
        self.row as u16
    }

    pub fn col_u16(&self) -> u16 {
        self.col as u16
    }

    pub fn move_down(&mut self, n: usize) {
        self.row += n;
    }

    pub fn move_up(&mut self, n: usize) {
        if n > self.row {
            panic!("row out-of-bound. [ row:{}, n:{} ]", self.row, n);
        }

        self.row -= n;
    }

    pub fn move_left(&mut self, n: usize) {
        if n > self.col {
            panic!("row out-of-bound. [ row:{}, n:{} ]", self.row, n);
        }

        self.col -= n;
    }

    pub fn move_right(&mut self, n: usize) {
        self.col += n;
    }

    pub fn move_to(&mut self, row: usize, col: usize) {
        self.row = row;
        self.col = col;
    }

    pub fn move_to_row(&mut self, row: usize) {
        self.row = row;
    }

    pub fn move_to_col(&mut self, col: usize) {
        self.col = col;
    }
}
