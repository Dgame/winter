use basic::{CursorMove, Empty};

#[derive(Debug, Clone, Copy)]
pub struct Cursor {
    index: usize,
    nearest: usize,
    farthest: usize,
}

impl Empty for Cursor {
    fn empty() -> Self {
        Self::new(0, 0)
    }
}

impl Cursor {
    pub fn new(index: usize, nearest: usize) -> Self {
        Self {
            index,
            nearest,
            farthest: 0,
        }
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn nearest(&self) -> usize {
        self.nearest
    }

    pub fn farthest(&self) -> usize {
        self.farthest
    }

    pub fn at_end(&mut self) -> bool {
        self.farthest <= self.index
    }

    pub fn reduce_offset(&mut self) {
        if self.farthest > self.index {
            self.farthest -= 1;
        }
    }

    pub fn move_back(&mut self) {
        self.move_left();
        self.reduce_offset();
    }

    pub fn move_front(&mut self) {
        self.index += 1;
        self.farthest += 1;
    }

    pub fn can_move_right(&self) -> bool {
        self.index < self.farthest
    }

    pub fn can_move_left(&self) -> bool {
        self.index > self.nearest
    }
}

impl CursorMove for Cursor {
    fn move_left(&mut self) {
        if self.can_move_left() {
            self.index -= 1;
        }
    }

    fn move_right(&mut self) {
        if self.can_move_right() {
            self.index += 1;
        }
    }
}
