mod coord;
mod cursor;
mod viewport;

pub use self::coord::Coord;
pub use self::cursor::Cursor;
pub use self::viewport::Viewport;

pub trait Empty {
    fn empty() -> Self;
}

pub trait CursorMove {
    fn move_left(&mut self);
    fn move_right(&mut self);
}

#[derive(Debug, Clone, Copy)]
pub struct Size {
    pub width: usize,
    pub height: usize,
}

impl Size {
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }
}
