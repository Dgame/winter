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
    pub pos: Coord,
    pub size: Size,
}

impl Viewport {
    pub fn with(coord: Coord, size: Size) -> Self {
        Self { pos: coord, size }
    }

    pub fn new(x: usize, y: usize, w: usize, h: usize) -> Self {
        Self::with(Coord::new(x, y), Size::new(w, h))
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
    gap: usize,
    offset: usize,
}

impl Cursor {
    pub fn new(index: usize, gap: usize) -> Self {
        Self {
            index,
            gap,
            offset: 0,
        }
    }

    pub fn empty() -> Self {
        Self::new(0, 0)
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn diff(&self) -> usize {
        self.offset - self.index
    }

    pub fn reduce_offset(&mut self) {
        self.offset -= 1;
    }

    pub fn can_move_right(&self) -> bool {
        self.index < self.offset
    }

    pub fn do_move(&mut self) {
        self.index += 1;
        self.offset += 1;
    }

    pub fn move_right(&mut self) {
        if self.can_move_right() {
            self.index += 1;
        }
    }

    pub fn can_move_left(&self) -> bool {
        self.index > self.gap
    }

    pub fn move_left(&mut self) {
        if self.can_move_left() {
            self.index -= 1;
        }
    }
}
