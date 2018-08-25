use basic::{Coord, CursorMove, Empty};

#[derive(Debug, Clone, Copy)]
pub struct Cursor {
    pos: Coord,
    nearest: usize,
    farthest: usize,
}

impl Empty for Cursor {
    fn empty() -> Self {
        Self::new(Coord::empty(), 0)
    }
}

impl Cursor {
    pub fn new(pos: Coord, nearest: usize) -> Self {
        Self {
            pos,
            nearest,
            farthest: 0,
        }
    }

    pub fn pos(&self) -> Coord {
        self.pos
    }

    pub fn nearest(&self) -> usize {
        self.nearest
    }

    pub fn farthest(&self) -> usize {
        self.farthest
    }

    pub fn at_end(&mut self) -> bool {
        self.farthest <= self.pos.x
    }

    pub fn reduce_offset(&mut self) {
        if self.farthest > self.pos.x {
            self.farthest -= 1;
        }
    }

    pub fn move_back(&mut self) {
        self.move_left();
        self.reduce_offset();
    }

    pub fn move_front(&mut self) {
        self.pos.x += 1;
        self.farthest += 1;
    }

    pub fn can_move_right(&self) -> bool {
        self.pos.x < self.farthest
    }

    pub fn can_move_left(&self) -> bool {
        self.pos.x > self.nearest
    }
}

impl CursorMove for Cursor {
    fn move_left(&mut self) {
        if self.can_move_left() {
            self.pos.x -= 1;
        }
    }

    fn move_right(&mut self) {
        if self.can_move_right() {
            self.pos.x += 1;
        }
    }
}
