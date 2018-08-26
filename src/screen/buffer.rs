use basic::Size;
use cli::Cell;
use std::ops::Deref;

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

impl Deref for Buffer {
    type Target = Vec<Cell>;

    fn deref(&self) -> &Vec<Cell> {
        &self.cells
    }
}

impl AsMut<[Cell]> for Buffer {
    fn as_mut(&mut self) -> &mut [Cell] {
        self.cells.as_mut()
    }
}
