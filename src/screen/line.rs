use basic::{Cursor, CursorMove, Empty};
use cli::{Cell, Display, Write};
use memory::{MutSlice, SetAt};
use screen::{CursorDel, CursorShift};

pub struct Line {
    cursor: Cursor,
    buffer: MutSlice<Cell>,
}

impl Line {
    pub fn first(buffer: MutSlice<Cell>) -> Self {
        Self::new(Cursor::empty(), buffer)
    }

    pub fn new(cursor: Cursor, buffer: MutSlice<Cell>) -> Self {
        Self { cursor, buffer }
    }

    pub fn cursor(&self) -> Cursor {
        self.cursor
    }

    pub fn get(&self) -> &[Cell] {
        let nearest = self.cursor.nearest();
        let farthest = self.cursor.farthest();

        &self.buffer.as_ref()[nearest..farthest]
    }
}

impl ToString for Line {
    fn to_string(&self) -> String {
        self.get().iter().map(|cell| cell.ch).collect()
    }
}

impl CursorShift for Line {
    fn shift_left(&mut self) {
        let index = self.cursor.index();
        let length = self.cursor.farthest();

        let cells: Vec<Cell> = self
            .buffer
            .as_ref()
            .iter()
            .skip(index + 1)
            .take(length)
            .cloned()
            .collect();
        let mut ci = index;
        for cell in cells {
            self.buffer.set_at(ci, cell);
            ci += 1;
        }
        self.buffer.set_at(ci, Cell::default());
    }

    fn shift_right(&mut self) {
        let index = self.cursor.index();
        let length = self.cursor.farthest();

        let cells: Vec<Cell> = self
            .buffer
            .as_ref()
            .iter()
            .skip(index)
            .take(length)
            .cloned()
            .collect();
        let mut ci = index;
        for cell in cells {
            self.buffer.set_at(ci + 1, cell);
            ci += 1;
        }
    }
}

impl CursorMove for Line {
    fn move_left(&mut self) {
        self.cursor.move_left()
    }

    fn move_right(&mut self) {
        self.cursor.move_right()
    }
}

impl CursorDel for Line {
    fn del_left(&mut self) {
        if self.cursor.can_move_left() {
            self.cursor.move_back();
            self.shift_left();
        }
    }

    fn del_right(&mut self) {
        if self.cursor.can_move_right() {
            self.shift_left();
            self.cursor.reduce_offset();
        }
    }
}

impl Write for Line {
    fn write_text<T: Into<Display>>(&mut self, text: T) {
        if !self.cursor.at_end() {
            self.shift_right();
        }

        let mut index = self.cursor.index();
        for cell in text.into().iter() {
            self.write_cell(index, *cell);
            index += 1;
        }
    }

    fn write_cell(&mut self, index: usize, cell: Cell) {
        self.buffer.set_at(index, cell);
        self.cursor.move_front();
    }
}
