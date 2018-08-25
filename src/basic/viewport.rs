use basic::{Coord, Size};

#[derive(Debug, Clone)]
pub struct Viewport {
    pos: Coord,
    size: Size,
}

impl Viewport {
    pub fn with(coord: Coord, size: Size) -> Self {
        Self { pos: coord, size }
    }

    pub fn new(x: usize, y: usize, w: usize, h: usize) -> Self {
        Self::with(Coord::new(x, y), Size::new(w, h))
    }

    pub fn pos(&self) -> Coord {
        self.pos
    }

    pub fn size(&self) -> Size {
        self.size
    }

    pub fn resize(&mut self, size: Size) {
        self.size = size;
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
