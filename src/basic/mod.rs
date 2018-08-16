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

#[derive(Debug, Clone)]
pub struct Viewport {
    pub coord: Coord,
    pub size: Size,
}

impl Viewport {
    pub fn with(coord: Coord, size: Size) -> Self {
        Self { coord, size }
    }

    pub fn new(x: usize, y: usize, w: usize, h: usize) -> Self {
        Self::with(Coord::new(x, y), Size::new(w, h))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
}

impl Coord {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Self {
        Self::new(0, 0)
    }

    pub fn to_1d(&self, size: Size) -> usize {
        (self.x + size.width * self.y) as usize
    }

    pub fn index_to_2d(index: usize, size: Size) -> Self {
        let x = index % size.width;
        let y = index / size.width;

        Self::new(x, y)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Offset {
    pub start: usize,
    pub end: usize,
}

impl Offset {
    pub fn new(start: usize) -> Self {
        Self { start, end: 0 }
    }

    pub fn empty() -> Self {
        Self::new(0)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Cursor {
    pub index: usize,
    pub offset: Offset,
}

impl Cursor {
    pub fn new(offset: Offset) -> Self {
        Self {
            index: offset.start,
            offset,
        }
    }

    pub fn empty() -> Self {
        Self::new(Offset::empty())
    }

    pub fn move_right(&mut self) {
        self.index += 1;
        self.offset.end += 1;
    }

    pub fn move_left(&mut self) {
        if self.index > self.offset.start {
            self.index -= 1;
        }
    }
}
