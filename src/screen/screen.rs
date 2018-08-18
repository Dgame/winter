use basic::{Coord, Cursor, Size, Viewport};
use cell::Cell;
use console::Console;
use screen::{Buffer, Line};

pub struct Screen {
    viewport: Viewport,
    front: Buffer,
    back: Buffer,
    line: Line,
    y_offset: usize,
}

//impl Drop for Screen {
//    fn drop(&mut self) {
//        self.buffer.clear();
//    }
//}

impl Screen {
    pub fn new(viewport: Viewport) -> Self {
        let size = viewport.size();

        Self {
            viewport,
            front: Buffer::new(size),
            back: Buffer::new(size),
            line: Line::first(size),
            y_offset: 0,
        }
    }

    pub fn resize(&mut self, size: Size) {
        self.viewport.resize(size);
        self.line.resize(size);
        self.front.clear();
    }

    pub fn line(&self) -> &Line {
        &self.line
    }

    pub fn line_mut(&mut self) -> &mut Line {
        &mut self.line
    }

    pub fn write<S: Into<String>>(&mut self, s: S) -> Coord {
        let mut i = self.line.get_current_index();

        if !self.line.cursor().at_end() {
            self.front.shift_front(i);
        }

        for ch in s.into().chars() {
            self.front.write(i, Cell::plain(ch));
            i += 1;
            self.line.cursor_mut().move_ahead();
        }

        self.line.get_cursor_pos()
    }

    pub fn writeln<S: Into<String>>(&mut self, s: S) -> (Coord, String) {
        self.write(s);
        self.newline()
    }

    pub fn newline(&mut self) -> (Coord, String) {
        let input: String = self.line.get(&self.front);

        self.y_offset += 1;
        self.line = Line::new(self.y_offset, Cursor::new(0, 2), self.viewport.size());
        self.write("~ ");

        (self.line.get_cursor_pos(), input)
    }

    pub fn del_left(&mut self) -> Coord {
        if self.line.cursor().can_move_left() {
            self.line.cursor_mut().move_back();
            let i = self.line.get_current_index();
            self.front.shift_back(i);
        }

        self.line.get_cursor_pos()
    }

    pub fn del_right(&mut self) -> Coord {
        if self.line.cursor().can_move_right() {
            let i = self.line.get_current_index();
            self.front.shift_back(i);
            self.line.cursor_mut().reduce_offset();
        }

        self.line.get_cursor_pos()
    }

    pub fn render(&mut self, console: &mut Console) {
        for (i, cell) in self.front.iter().enumerate() {
            if self.back.need_update(i, *cell) {
                let mut pos = Coord::index_to_2d(i, self.viewport.size());
                pos.x += self.viewport.x();
                pos.y += self.viewport.y();

                console.write_cell(pos, *cell);
                self.back.write(i, *cell);
            }
        }
    }
}
