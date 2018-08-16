extern crate winter;

use std::env;
use std::fs::File;
use std::io::Write;
use winter::basic::Coord;
use winter::basic::Cursor;
use winter::basic::Size;
use winter::basic::Viewport;
use winter::cell::Cell;
use winter::console::Console;
use winter::input::InputEvent;
use winter::input::Key;

pub struct DoubleBuffer {
    pub front: Vec<Cell>,
    pub back: Vec<Cell>,
}

impl DoubleBuffer {
    pub fn new(size: Size) -> Self {
        let cap = size.width * size.height;

        let mut front = Vec::with_capacity(cap);
        let mut back = Vec::with_capacity(cap);

        while front.len() < front.capacity() {
            front.push(Cell::default());
            back.push(Cell::default());
        }

        Self { front, back }
    }

    pub fn clear(&mut self) {
        self.clear_front();
        self.clear_back();
    }

    pub fn clear_front(&mut self) {
        for cell in &mut self.front.iter_mut() {
            *cell = Cell::default();
        }
    }

    pub fn clear_back(&mut self) {
        for cell in &mut self.back.iter_mut() {
            *cell = Cell::default();
        }
    }
}

pub struct Screen {
    viewport: Viewport,
    buffer: DoubleBuffer,
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
        let buffer = DoubleBuffer::new(viewport.size);

        Self {
            viewport,
            buffer,
            cursor: Cursor::empty(),
            y_offset: 0,
        }
    }

    pub fn resize(&mut self, size: Size) {
        self.viewport.size = size;
        self.buffer.clear_front();
    }

    pub fn get_cursor_pos(&self) -> Coord {
        Coord::new(self.cursor.index(), self.y_offset)
    }

    pub fn write<S: Into<String>>(&mut self, s: S) -> Coord {
        let mut pos = self.get_cursor_pos();
        for ch in s.into().chars() {
            let i = pos.to_1d(self.viewport.size);
            self.buffer.front[i] = Cell::plain(ch);
            pos.x += 1;
            self.cursor.do_move();
        }

        self.get_cursor_pos()
    }

    pub fn writeln<S: Into<String>>(&mut self, s: S) -> Coord {
        self.write(s);
        self.newline();

        self.get_cursor_pos()
    }

    pub fn newline(&mut self) -> Coord {
        self.y_offset += 1;
        self.cursor = Cursor::new(0, 2);
        self.write("~ ");

        self.get_cursor_pos()
    }

    pub fn del_left(&mut self) -> Coord {
        if self.cursor.can_move_left() {
            let mut pos = self.get_cursor_pos();
            pos.x -= 1;

            self.buffer.front[pos.to_1d(self.viewport.size)] = Cell::default();
            self.cursor.move_left();
            self.cursor.reduce_offset();
        }

        self.get_cursor_pos()
    }

    pub fn del_right(&mut self) -> Coord {
        if self.cursor.can_move_right() {
            let mut pos = self.get_cursor_pos();
            for _ in 0 .. self.cursor.diff() {
                let i = pos.to_1d(self.viewport.size);
                self.buffer.front[i] = self.buffer.front[i + 1];
                pos.x += 1;
            }

            self.cursor.reduce_offset();
        }

        self.get_cursor_pos()
    }

    fn redraw(&mut self, console: &mut Console) {
        for (i, cell) in self.buffer.front.iter().enumerate() {
            if *cell != self.buffer.back[i] {
                let mut pos = Coord::index_to_2d(i, self.viewport.size);
                pos.x += self.viewport.x();
                pos.y += self.viewport.y();

                console.write_cell(pos, *cell);
                self.buffer.back[i] = *cell;
            }
        }
    }

    pub fn render(&mut self, console: &mut Console) {
        self.redraw(console);
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
            _ => {}
        }
    }

    inputs
}

fn main() {
    let mut console = Console::new();
    let mut manager = ScreenManager::new(Size::new(50, 25));
    let path = env::current_dir().unwrap();
    let cursor_pos = manager.screen_mut().writeln(path.to_str().unwrap());
    console.set_cursor_pos(cursor_pos);

    let mut run = true;
    while run {
        for event in get_input_events(&mut console) {
            if event.is_pressed {
                //                println!("Key {:?} was pressed", event.key);
                match event.key {
                    Key::Escape => run = false,
                    Key::Return => {
                        let cursor_pos = manager.screen_mut().newline();
                        console.set_cursor_pos(cursor_pos);
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
