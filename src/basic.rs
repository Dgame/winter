#[allow(non_camel_case_types)]
pub type size_t = i16;

#[derive(Clone, Copy)]
pub struct Size {
    pub width: size_t,
    pub height: size_t,
}

impl Size {
    pub fn new(width: size_t, height: size_t) -> Self {
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

#[derive(Clone, Copy)]
pub struct Position {
    pub x: size_t,
    pub y: size_t,
}

impl Position {
    pub fn new(x: size_t, y: size_t) -> Self {
        Self { x, y }
    }
    pub fn zero() -> Self {
        Self::new(0, 0)
    }
}
