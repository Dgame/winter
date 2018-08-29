use basic::{Coord, Cursor, CursorMove, Size, Viewport};
use cli::{Console, Display, Write};
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
        let line = Line::first(MutSlice::from(front.as_mut()));

        Self {
            viewport,
            front,
            back,
            line,
            top: 0,
        }
    }

    pub fn clear(&mut self) {
        self.front.clear();
        self.top = 0;
        self.line = Line::first(MutSlice::from(self.front.as_mut()));
    }

    pub fn resize(&mut self, size: Size) {
        self.viewport.resize(size);
        //        self.line.resize(size);
        self.front.clear();
    }

    pub fn top(&self) -> usize {
        self.top
    }

    pub fn viewport(&self) -> &Viewport {
        &self.viewport
    }

    pub fn line(&self) -> &Line {
        &self.line
    }

    pub fn cursor_pos(&self) -> Coord {
        let mut pos = Coord::index_to_2d(self.line.cursor().index(), self.viewport.width());
        pos.y += self.top;

        pos
    }

    pub fn write<T: Into<Display>>(&mut self, text: T) -> Coord {
        self.line.write_text(text);

        self.cursor_pos()
    }

    pub fn newline(&mut self, gap: usize) -> (Coord, String) {
        let input: String = self.line.to_string();
        let pos = Coord::index_to_2d(self.line.cursor().index(), self.viewport.width());
        self.top += pos.y + 1;

        let offset = Coord::new(0, self.top).to_1d(self.viewport.width());

        self.line = Line::new(
            Cursor::new(0, gap),
            MutSlice::from(&mut self.front.as_mut()[offset..]),
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
