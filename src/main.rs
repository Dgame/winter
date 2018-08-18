extern crate winter;

use std::env;
use std::fs::File;
use std::io::Write;
use winter::basic::Coord;
use winter::basic::Cursor;
use winter::basic::Size;
use winter::basic::Viewport;
use winter::buffer::Buffer;
use winter::cell::Cell;
use winter::console::Console;
use winter::input::InputEvent;
use winter::input::Key;

pub struct Screen {
    viewport: Viewport,
    front: Buffer,
    back: Buffer,
    cursor: Cursor,
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
            cursor: Cursor::empty(),
            y_offset: 0,
        }
    }

    pub fn resize(&mut self, size: Size) {
        self.viewport.resize(size);
        self.front.clear();
    }

    pub fn get_cursor_pos(&self) -> Coord {
        Coord::new(self.cursor.index(), self.y_offset)
    }

    fn get_buffer_index(&self) -> usize {
        self.get_cursor_pos().to_1d(self.viewport.size())
    }

    pub fn write<S: Into<String>>(&mut self, s: S) -> Coord {
        let mut i = self.get_buffer_index();

        if !self.cursor.at_end() {
            self.front.shift_front(i);
        }

        for ch in s.into().chars() {
            self.front.write(i, Cell::plain(ch));
            i += 1;
            self.cursor.move_ahead();
        }

        self.get_cursor_pos()
    }

    pub fn writeln<S: Into<String>>(&mut self, s: S) -> (Coord, String) {
        self.write(s);
        self.newline()
    }

    pub fn newline(&mut self) -> (Coord, String) {
        let coord = Coord::new(self.cursor.start(), self.y_offset);
        let offset = coord.to_1d(self.viewport.size());

        let input: String = self
            .front
            .iter()
            .skip(offset)
            .take_while(|cell| !cell.is_empty())
            .map(|cell| cell.ch)
            .collect();

        self.y_offset += 1;
        self.cursor = Cursor::new(0, 2);
        self.write("~ ");

        (self.get_cursor_pos(), input)
    }

    pub fn del_left(&mut self) -> Coord {
        if self.cursor.can_move_left() {
            self.cursor.move_back();
            let i = self.get_buffer_index();
            self.front.shift_back(i);
        }

        self.get_cursor_pos()
    }

    pub fn del_right(&mut self) -> Coord {
        if self.cursor.can_move_right() {
            let i = self.get_buffer_index();
            self.front.shift_back(i);
            self.cursor.reduce_offset();
        }

        self.get_cursor_pos()
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

pub struct ScreenManager {
    screens: Vec<Screen>,
    screen_id: usize,
}

impl ScreenManager {
    pub fn new(size: Size) -> Self {
        let viewport = Viewport::with(Coord::zero(), size);
        let screen = Screen::new(viewport);
        let mut screens = Vec::with_capacity(4);
        screens.push(screen);

        Self {
            screens,
            screen_id: 0,
        }
    }

    pub fn screen(&self) -> &Screen {
        &self.screens[self.screen_id]
    }

    pub fn screen_mut(&mut self) -> &mut Screen {
        &mut self.screens[self.screen_id]
    }

    pub fn switch_screen(&mut self, screen_id: usize) {
        self.screen_id = screen_id;
    }

    pub fn render(&mut self, console: &mut Console) {
        for screen in self.screens.iter_mut() {
            screen.render(console);
        }
    }
}

fn get_input_events(console: &mut Console) -> Vec<InputEvent> {
    let mut inputs = Vec::new();
    for input in console.get_input() {
        match input.EventType {
            KEY_EVENT => inputs.push(InputEvent::from(&input)),
        }
    }

    inputs
}

fn main() {
    let mut console = Console::new();
    let mut manager = ScreenManager::new(Size::new(50, 25));
    let path = env::current_dir().unwrap();
    let (cursor_pos, _) = manager.screen_mut().writeln(path.to_str().unwrap());
    console.set_cursor_pos(cursor_pos);

    let mut file = File::create("buf.txt").unwrap();

    let mut run = true;
    while run {
        for event in get_input_events(&mut console) {
            if event.is_pressed {
                //                println!("Key {:?} was pressed", event.key);
                match event.key {
                    Key::Escape => run = false,
                    Key::Return => {
                        let (cursor_pos, input) = manager.screen_mut().newline();
                        console.set_cursor_pos(cursor_pos);
                        file.write(input.as_bytes());
                    }
                    Key::Back => {
                        let cursor_pos = manager.screen_mut().del_left();
                        console.set_cursor_pos(cursor_pos);
                    }
                    Key::Delete => {
                        let cursor_pos = manager.screen_mut().del_right();
                        console.set_cursor_pos(cursor_pos);
                    }
                    Key::Left => {
                        manager.screen_mut().cursor.move_left();
                        let cursor_pos = manager.screen_mut().get_cursor_pos();
                        console.set_cursor_pos(cursor_pos);
                    }
                    Key::Right => {
                        manager.screen_mut().cursor.move_right();
                        let cursor_pos = manager.screen_mut().get_cursor_pos();
                        console.set_cursor_pos(cursor_pos);
                    }
                    _ => {
                        let cursor_pos = manager
                            .screen_mut()
                            .write(event.key.to_string(event.control));
                        console.set_cursor_pos(cursor_pos);
                    }
                }
            }
        }

        manager.screen_mut().render(&mut console);
    }
}
