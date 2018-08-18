use basic::Size;
use cell::Cell;
use std::slice::Iter;

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

    pub fn clear(&mut self) {
        for cell in self.cells.iter_mut() {
            *cell = Cell::default();
        }
    }

    pub fn need_update(&self, index: usize, cell: Cell) -> bool {
        self.cells[index] != cell
    }

    pub fn shift_front(&mut self, offset: usize) {
        let cells: Vec<Cell> = self
            .cells
            .iter()
            .skip(offset)
            .take_while(|cell| !cell.is_empty())
            .cloned()
            .collect();
        let mut ci = offset;
        for cell in cells {
            self.cells[ci + 1] = cell;
            ci += 1;
        }
    }

    pub fn shift_back(&mut self, offset: usize) {
        let cells: Vec<Cell> = self
            .cells
            .iter()
            .skip(offset + 1)
            .take_while(|cell| !cell.is_empty())
            .cloned()
            .collect();
        let mut ci = offset;
        for cell in cells {
            self.cells[ci] = cell;
            ci += 1;
        }
        self.cells[ci] = Cell::default();
    }

    pub fn write(&mut self, index: usize, cell: Cell) {
        self.cells[index] = cell;
    }
}
