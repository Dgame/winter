use basic::{Cursor, CursorMove, Empty};
use cli::{Cell, Text, Write};
use memory::MutSlice;
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

    pub fn get(&self) -> String {
        let nearest = self.cursor.nearest();
        let farthest = self.cursor.farthest();

        let s: String = self
            .buffer
            .to_slice()
            .iter()
            .skip(nearest)
            .take(farthest)
            .map(|cell| cell.ch)
            .collect();

        s.trim().to_string()
    }
}

impl CursorShift for Line {
    fn shift_left(&mut self) {
        let index = self.cursor.pos().x;
        let length = self.cursor.farthest();

        let cells: Vec<Cell> = self
            .buffer
            .to_slice()
            .iter()
            .skip(index + 1)
            .take(length)
            .cloned()
            .collect();
        let mut ci = index;
        for cell in cells {
            self.buffer.set(ci, cell);
            ci += 1;
        }
        self.buffer.set(ci, Cell::default());
    }

    fn shift_right(&mut self) {
        let index = self.cursor.pos().x;
        let length = self.cursor.farthest();

        let cells: Vec<Cell> = self
            .buffer
            .to_slice()
            .iter()
            .skip(index)
            .take(length)
            .cloned()
            .collect();
        let mut ci = index;
        for cell in cells {
            self.buffer.set(ci + 1, cell);
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
    fn write_text<T: Into<Text>>(&mut self, text: T) {
        if !self.cursor.at_end() {
            self.shift_right();
        }

        let mut index = self.cursor.pos().x;
        for cell in text.into().iter() {
            self.write_cell(index, *cell);
            index += 1;
        }
    }

    fn write_cell(&mut self, index: usize, cell: Cell) {
        self.buffer.set(index, cell);
        self.cursor.move_front();
    }
}
