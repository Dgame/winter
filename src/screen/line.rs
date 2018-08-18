use basic::{Coord, Cursor, Size};
use screen::Buffer;

pub struct Line {
    y_offset: usize,
    cursor: Cursor,
    size: Size,
}

impl Line {
    pub fn first(size: Size) -> Self {
        Self::new(0, Cursor::empty(), size)
    }

    pub fn new(y_offset: usize, cursor: Cursor, size: Size) -> Self {
        Self {
            y_offset,
            cursor,
            size,
        }
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
        Coord::new(self.cursor.index(), self.y_offset)
    }

    pub fn get_current_index(&self) -> usize {
        self.get_cursor_pos().to_1d(self.size)
    }

    pub fn start_index(&self) -> usize {
        let coord = Coord::new(self.cursor.start(), self.y_offset);

        coord.to_1d(self.size)
    }

    pub fn end_index(&self) -> usize {
        let coord = Coord::new(self.cursor.offset(), self.y_offset);

        coord.to_1d(self.size)
    }

    pub fn get(&self, buffer: &Buffer) -> String {
        let index = self.start_index();
        let length = self.end_index() - index;

        buffer
            .iter()
            .skip(index)
            .take(length)
            .map(|cell| cell.ch)
            .collect()
    }
}
