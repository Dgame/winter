use basic::{size_t, Position, Size};
use buffer::Buffer;
use cell::Cell;
use console::Console;
use viewport::Viewport;

pub struct View {
    buffer: Buffer,
    viewport: Viewport,
    offset: usize,
    needs_update: bool,
}

fn index_to_2d(index: size_t, size: Size) -> Position {
    let x = index % size.width;
    let y = index / size.width;

    Position::new(x, y)
}

impl View {
    pub fn new(pos: Position, size: Size) -> Self {
        let mut buffer = Buffer::new(size);
        buffer.init();

        Self {
            buffer,
            viewport: Viewport::new(pos, size),
            offset: 0,
            needs_update: true,
        }
    }

    pub fn seek(&mut self, offset: usize) {
        self.offset = offset;
    }

    pub fn get_size(&self) -> Size {
        self.viewport.get_size()
    }

    pub fn get_position(&self) -> Position {
        self.viewport.get_position()
    }

    pub fn write<T: Into<String>>(&mut self, arg: T) {
        for ch in arg.into().chars() {
            self.buffer.front[self.offset] = Cell::plain(ch);
            self.offset += 1;
        }
    }

    pub fn clear_front_buffer(&mut self) {
        self.buffer.clear_front();
        self.seek(0);
    }

    pub fn clear_back_buffer(&mut self) {
        self.buffer.clear_back();
    }

    pub fn display(&mut self, console: &mut Console) {
        for (i, cell) in self.buffer.front.iter().enumerate() {
            if self.needs_update || *cell != self.buffer.back[i] {
                let mut pos = index_to_2d(i as size_t, self.viewport.get_size());

                let cell = if pos.x == 0 && cell.is_empty() {
                    Cell::border()
                } else {
                    *cell
                };

                pos.x += self.viewport.x();
                pos.y += self.viewport.y();

                console.write_cell(pos, cell);
                self.buffer.back[i] = cell;
            }
        }

        self.needs_update = false;
    }

    pub fn fill_with(&mut self, ch: char) {
        for cell in self.buffer.front.iter_mut() {
            cell.ch = ch;
        }
    }

    pub fn resize(&mut self, new_size: Size) {
        self.viewport.set_size(new_size);
        self.needs_update = true;
    }
}
