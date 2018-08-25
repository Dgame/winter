use basic::Empty;

#[derive(Debug, Clone, Copy)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
}

impl Empty for Coord {
    fn empty() -> Self {
        Self::new(0, 0)
    }
}

impl Coord {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn to_1d(&self, width: usize) -> usize {
        (self.x + width * self.y) as usize
    }

    pub fn index_to_2d(index: usize, width: usize) -> Self {
        let x = index % width;
        let y = index / width;

        Self::new(x, y)
    }
}
