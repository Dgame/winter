use basic::{Coord, Cursor, CursorMove, Size, Viewport};
use cli::{Console, Text, Write};
use memory::MutSlice;
use screen::{Buffer, CursorDel, Line};

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

    pub fn write<T: Into<Text>>(&mut self, text: T) -> Coord {
        self.line.write_text(text);

        self.cursor_pos()
    }

    pub fn newline(&mut self, gap: usize) -> (Coord, String) {
        let input: String = self.line.get();

        self.top += 1;

        let offset = Coord::new(0, self.top).to_1d(self.viewport.width());
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
                let mut pos = Coord::index_to_2d(i, self.viewport.width());
                pos.x += self.viewport.x();
                pos.y += self.viewport.y();

                console.write_cell(pos, *cell);
                self.back.write(i, *cell);
            }
        }
    }
}

impl CursorMove for Screen {
    fn move_left(&mut self) {
        self.line.move_left()
    }

    fn move_right(&mut self) {
        self.line.move_right()
    }
}

impl CursorDel for Screen {
    fn del_left(&mut self) {
        self.line.del_left();
    }

    fn del_right(&mut self) {
        self.line.del_right();
    }
}
