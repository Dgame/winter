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
pub struct Cursor {
    index: usize,
    nearest: usize,
    farthest: usize,
} // TODO: y_offset mit aufnehmen? Bzw. Coord(index, y_offset)

impl Cursor {
    pub fn new(index: usize, nearest: usize) -> Self {
        Self {
            index,
            nearest,
            farthest: 0,
        }
    }

    pub fn empty() -> Self {
        Self::new(0, 0)
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

    pub fn diff(&self) -> usize {
        self.farthest - self.index
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

    pub fn can_move_right(&self) -> bool {
        self.index < self.farthest
    }

    pub fn move_ahead(&mut self) {
        self.index += 1;
        self.farthest += 1;
    }

    pub fn move_right(&mut self) {
        if self.can_move_right() {
            self.index += 1;
        }
    }

    pub fn can_move_left(&self) -> bool {
        self.index > self.nearest
    }

    pub fn move_left(&mut self) {
        if self.can_move_left() {
            self.index -= 1;
        }
    }
}
