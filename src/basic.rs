#[derive(Debug, Clone, Copy)]
pub struct Size {
    pub width: usize,
    pub height: usize,
}

impl Size {
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }
    pub fn empty() -> Self {
        Self::new(0, 0)
    }

    pub fn get_required_capacity(&self) -> usize {
        self.width as usize * self.height as usize
    }

    pub fn half(&self) -> Self {
        Self {
            width: self.width / 2,
            height: self.height / 2,
        }
    }

    pub fn with_half_width(&self) -> Self {
        Self {
            width: self.width / 2,
            height: self.height,
        }
    }

    pub fn with_half_height(&self) -> Self {
        Self {
            width: self.width,
            height: self.height / 2,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
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
