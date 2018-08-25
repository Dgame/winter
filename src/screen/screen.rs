use basic::{Coord, Cursor, Size, Viewport};
use cli::{Cell, Console};
use memory::MutSlice;
use screen::{Buffer, Line};

pub struct Screen {
    viewport: Viewport,
    front: Buffer,
    back: Buffer,
    line: Line,
    top: usize,
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
        let line = Line::first(MutSlice::from_slice(front.mut_slice(0), length));

        Self {
            viewport,
            front,
            back,
            line,
            top: 0,
        }
    }

    pub fn resize(&mut self, size: Size) {
        self.viewport.resize(size);
        //        self.line.resize(size);
        self.front.clear();
    }

    pub fn cursor_pos(&self) -> Coord {
        self.line.cursor().pos()
    }

    pub fn move_right(&mut self) {
        self.line.move_right()
    }

    pub fn move_left(&mut self) {
        self.line.move_left()
    }

    pub fn del_left(&mut self) {
        self.line.del_left();
    }

    pub fn del_right(&mut self) {
        self.line.del_right();
    }

    pub fn write_text<S: Into<String>>(&mut self, s: S) -> Coord {
        if !self.line.cursor().at_end() {
            self.line.shift_front();
        }

        let mut pos = self.cursor_pos();
        for ch in s.into().chars() {
            let cell = Cell::from(ch);
            self.write_cell(pos, cell);
            pos.x += 1;
        }

        self.cursor_pos()
    }

    pub fn write(&mut self, cells: &[Cell]) -> Coord {
        let mut pos = self.cursor_pos();
        for cell in cells {
            self.write_cell(pos, *cell);
            pos.x += 1;
        }

        self.cursor_pos()
    }

    pub fn write_cell(&mut self, pos: Coord, cell: Cell) -> Coord {
        let i = pos.to_1d(self.viewport.size());
        self.front.write(i, cell);
        self.line.cursor_mut().move_front();

        self.cursor_pos()
    }

    pub fn newline(&mut self, gap: usize) -> (Coord, String) {
        let input: String = self.line.get();

        self.top += 1;

        let offset = Coord::new(0, self.top).to_1d(self.viewport.size());
        let length = self.front.length();

        self.line = Line::new(
            Cursor::new(Coord::new(0, self.top), gap),
            MutSlice::from_slice(self.front.mut_slice(offset), length),
        );

        (self.cursor_pos(), input)
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
