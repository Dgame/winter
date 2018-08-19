use basic::{Coord, Cursor, Size, Viewport};
use cell::Cell;
use console::Console;
use memory::MutSlice;
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

        let mut front = Buffer::new(size);
        let back = Buffer::new(size);
        let length = front.length();
        let line = Line::first(size, MutSlice::from_slice(front.mut_slice(0), length));

        Self {
            viewport,
            front,
            back,
            line,
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
        if !self.line.cursor().at_end() {
            self.line.shift_front();
        }

        let mut i = self.line.get_current_index();
        for ch in s.into().chars() {
            self.front.write(i, Cell::plain(ch));
            i += 1;
            self.line.cursor_mut().move_ahead();
        }

        self.line.get_cursor_pos()
    }

    pub fn newline(&mut self, gap: usize) -> (Coord, String) {
        let input: String = self.line.get();

        self.y_offset += 1;

        let offset = Coord::new(0, self.y_offset).to_1d(self.viewport.size());
        let length = self.front.length();

        self.line = Line::new(
            self.y_offset,
            Cursor::new(0, gap),
            self.viewport.size(),
            MutSlice::from_slice(self.front.mut_slice(offset), length),
        );

        (self.line.get_cursor_pos(), input)
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
