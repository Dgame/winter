use basic::Position;
use basic::{size_t, Size};

pub struct Viewport {
    size: Size,
    pos: Position,
}

impl Viewport {
    pub fn new(pos: Position, size: Size) -> Self {
        Self { size, pos }
    }

    pub fn half(&self) -> Self {
        Self {
            pos: self.pos,
            size: self.size.half(),
        }
    }

    pub fn get_size(&self) -> Size {
        self.size
    }

    pub fn set_size(&mut self, size: Size) {
        self.size = size;
    }

    pub fn set_abs_position(&mut self, pos: Position) {
        self.pos = pos;
    }

    pub fn set_rel_position(&mut self, pos: Position) {
        self.pos.x += pos.x;
        self.pos.y += pos.y;
    }

    pub fn get_position(&self) -> Position {
        self.pos
    }

    pub fn width(&self) -> size_t {
        self.size.width
    }

    pub fn height(&self) -> size_t {
        self.size.height
    }

    pub fn x(&self) -> size_t {
        self.pos.x
    }

    pub fn y(&self) -> size_t {
        self.pos.y
    }
}
