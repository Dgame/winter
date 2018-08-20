use basic::{Coord, Cursor, Size};
use memory::MutSlice;
use cli::Cell;

pub struct Line {
    cursor: Cursor,
    size: Size, // TODO: Auslagern
    buffer: MutSlice<Cell>,
}

impl Line {
    pub fn first(size: Size, buffer: MutSlice<Cell>) -> Self {
        Self::new(Cursor::empty(), size, buffer)
    }

    pub fn new(cursor: Cursor, size: Size, buffer: MutSlice<Cell>) -> Self {
        Self { cursor, size, buffer }
    }

    pub fn resize(&mut self, size: Size) {
        self.size = size;
    }

    pub fn cursor(&self) -> Cursor {
        self.cursor
    }

    pub fn cursor_mut(&mut self) -> &mut Cursor {
        &mut self.cursor
    }

    pub fn get_cursor_pos(&self) -> Coord {
        self.cursor.pos()
    }

    // TODO: Auslagern
    pub fn get_current_index(&self) -> usize {
        self.get_cursor_pos().to_1d(self.size)
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

    pub fn del_left(&mut self) -> Coord {
        if self.cursor.can_move_left() {
            self.cursor.move_back();
            self.shift_back();
        }

        self.get_cursor_pos()
    }

    pub fn del_right(&mut self) -> Coord {
        if self.cursor.can_move_right() {
            self.shift_back();
            self.cursor.reduce_offset();
        }

        self.get_cursor_pos()
    }

    pub fn shift_back(&mut self) {
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

    pub fn shift_front(&mut self) {
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
