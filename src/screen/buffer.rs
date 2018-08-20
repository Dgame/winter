use basic::Size;
use std::slice::Iter;
use cli::Cell;

pub struct Buffer {
    cells: Vec<Cell>,
}

impl Buffer {
    pub fn new(size: Size) -> Self {
        let cap = size.width * size.height;
        let mut cells = Vec::with_capacity(cap);
        while cells.len() < cap {
            cells.push(Cell::default());
        }

        Self { cells }
    }

    pub fn iter(&self) -> Iter<Cell> {
        self.cells.iter()
    }

    pub fn mut_slice(&mut self, offset: usize) -> &mut [Cell] {
        &mut self.cells[offset..]
    }

    pub fn length(&self) -> usize {
        self.cells.len()
    }

    pub fn clear(&mut self) {
        for cell in self.cells.iter_mut() {
            *cell = Cell::default();
        }
    }

    pub fn need_update(&self, index: usize, cell: Cell) -> bool {
        self.cells[index] != cell
    }

    pub fn write(&mut self, index: usize, cell: Cell) {
        self.cells[index] = cell;
    }
}
