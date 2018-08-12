use basic::Position;
use basic::Size;

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

    pub fn width(&self) -> usize {
        self.size.width
    }

    pub fn height(&self) -> usize {
        self.size.height
    }

    pub fn x(&self) -> usize {
        self.pos.x
    }

    pub fn y(&self) -> usize {
        self.pos.y
    }
}
