use basic::{Position, Size};
use buffer::Buffer;
use cell::Cell;
use console::Console;
use viewport::Viewport;

pub struct View {
    buffer: Buffer,
    viewport: Viewport,
    write_pos: Position,
    line_offset: usize,
    needs_update: bool,
}

impl View {
    pub fn new(pos: Position, size: Size) -> Self {
        let mut buffer = Buffer::new(size);
        buffer.init();

        Self {
            buffer,
            viewport: Viewport::new(pos, size),
            write_pos: Position::zero(),
            line_offset: 0,
            needs_update: true,
        }
    }

    pub fn get_size(&self) -> Size {
        self.viewport.get_size()
    }

    pub fn get_position(&self) -> Position {
        self.viewport.get_position()
    }

    pub fn get_write_pos(&self) -> Position {
        self.write_pos
    }

    pub fn set_write_pos(&mut self, pos: Position) {
        self.write_pos = pos;
    }

    pub fn get_line_offset(&self) -> usize {
        self.line_offset
    }

    pub fn newline(&mut self) -> Position {
        self.line_offset = 0;

        self.write_pos.x = 0;
        self.write_pos.y += 1;

        self.write_pos
    }

    pub fn write<T: Into<String>>(&mut self, arg: T) -> Position {
        for ch in arg.into().chars() {
            let offset = self.write_pos.to_1d(self.get_size());

            self.buffer.front[offset] = Cell::plain(ch);
            self.write_pos.x += 1;
        }

        self.line_offset = self.write_pos.x;

        self.write_pos
    }

    pub fn remove_lhs(&mut self) -> Position {
        if self.write_pos.x > 0 {
            if self.write_pos.x == self.line_offset {
                self.line_offset -= 1;
                self.buffer.front[self.line_offset] = Cell::default();
                self.write_pos.x -= 1;
            } else {
                let mut pos = self.write_pos;
                pos.x -= 1;

                let line: String = self
                    .buffer
                    .front
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| *i != pos.x)
                    .map(|(_, cell)| cell.ch)
                    .collect();

                self.buffer.clear_front();
                self.write_pos.x = 0;

                self.write(line);
                self.write_pos = pos;
            }
        }

        self.write_pos
    }

    pub fn clear_back_buffer(&mut self) {
        self.buffer.clear_back();
    }

    pub fn display(&mut self, console: &mut Console) {
        for (i, cell) in self.buffer.front.iter().enumerate() {
            if self.needs_update || *cell != self.buffer.back[i] {
                let mut pos = Position::index_to_2d(i, self.viewport.get_size());

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
