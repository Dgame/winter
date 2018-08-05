use basic::Size;
use cell::{Cell, DEFAULT_CH};

pub struct Buffer {
    pub front: Vec<Cell>,
    pub back: Vec<Cell>,
}

impl Buffer {
    pub fn new(size: Size) -> Self {
        let cap = size.get_required_capacity();
        Self {
            front: Vec::with_capacity(cap),
            back: Vec::with_capacity(cap),
        }
    }

    pub fn init(&mut self) {
        let cap = self.front.capacity();
        let mut i = 0;
        while i < cap {
            i += 1;
            self.front.push(Cell::default());
            self.back.push(Cell::default());
        }
    }

    pub fn clear(&mut self) {
        self.clear_front();
        self.clear_back();
    }

    pub fn clear_front(&mut self) {
        for cell in &mut self.front {
            cell.ch = DEFAULT_CH;
        }
    }

    pub fn clear_back(&mut self) {
        for cell in &mut self.back {
            cell.ch = DEFAULT_CH;
        }
    }
}
