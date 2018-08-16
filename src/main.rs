extern crate winter;

use std::env;
use winter::basic::Coord;
use winter::basic::Cursor;
use winter::basic::Offset;
use winter::basic::Size;
use winter::basic::Viewport;
use winter::cell::Cell;
use winter::console::Console;
use winter::input::InputEvent;
use winter::input::Key;
use std::fs::File;
use std::io::Write;

#[derive(Clone)]
struct Line {
    cursor: Cursor,
    buffer: Vec<Cell>,
}

impl Line {
    fn empty() -> Self {
        Self {
            cursor: Cursor::empty(),
            buffer: Vec::new(),
        }
    }

    fn with<S: Into<String>>(s: S) -> Self {
        let s = s.into();

        let mut line = Self::empty();
        line.write(s);

        line
    }

    fn write<S: Into<String>>(&mut self, s: S) {
        for ch in s.into().chars() {
            self.buffer.push(Cell::plain(ch));
            self.cursor.move_right();
        }
    }

    fn del_previous(&mut self) {
        self.cursor.move_left();
    }

    fn del_next(&mut self) {
        self.cursor.move_right();
    }
}

struct Drawing {
    need_update: bool,
    offset: usize,
}

impl Drawing {
    fn new() -> Self {
        Self {
            need_update: false,
            offset: 0
        }
    }

    fn cleared(&mut self) {
        self.need_update = true;
        self.offset = 0;
    }
}

struct Screen {
    viewport: Viewport,
    buffer: Vec<Cell>,
    line: Line,
    offset_y: usize,
    drawing: Drawing,
}

impl Screen {
    fn new(viewport: Viewport) -> Self {
        Self {
            viewport,
            buffer: Vec::new(),
            line: Line::empty(),
            offset_y: 0,
            drawing: Drawing::new()
        }
    }

    fn resize(&mut self, size: Size) {
        self.viewport.size = size;
        self.drawing.cleared();
    }

    fn get_cursor_pos(&self) -> Coord {
        Coord::new(self.line.cursor.index, self.offset_y)
    }

    fn write<S: Into<String>>(&mut self, s: S) -> Coord {
        self.line.write(s);

        self.get_cursor_pos()
    }

    fn writeln<S: Into<String>>(&mut self, s: S) -> Coord {
        self.write(s);
        self.newline();

        self.get_cursor_pos()
    }

    fn newline(&mut self) {
        self.buffer.extend(self.line.buffer.iter());
        self.offset_y += 1;
        self.line = Line::with("~ ");

        self.drawing.need_update = true;
    }

    fn redraw(&mut self, console: &mut Console) {
        for cell in self.buffer.iter().skip(self.drawing.offset) {
            let pos = Coord::index_to_2d(self.drawing.offset, self.viewport.size);
            console.write_cell(pos, *cell);
            self.drawing.offset += 1;
        }

        self.drawing.need_update = false;
    }

    fn render(&mut self, console: &mut Console) {
        if self.drawing.need_update {
            self.redraw(console);
        }

        let mut pos = Coord::new(0, self.offset_y);
        for cell in self.line.buffer.iter() {
            console.write_cell(pos, *cell);
            pos.x += 1;
        }
    }
}

struct ScreenManager {
    screens: Vec<Screen>,
    screen_id: usize,
}

impl ScreenManager {
    fn new(size: Size) -> Self {
        let viewport = Viewport::with(Coord::zero(), size);
        let screen = Screen::new(viewport);
        let mut screens = Vec::with_capacity(4);
        screens.push(screen);

        Self {
            screens,
            screen_id: 0,
        }
    }

    fn screen(&self) -> &Screen {
        &self.screens[self.screen_id]
    }

    fn screen_mut(&mut self) -> &mut Screen {
        &mut self.screens[self.screen_id]
    }

    fn render(&mut self, console: &mut Console) {
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
            _ => { }
        }
    }

    inputs
}

fn main() {
    let mut console = Console::new();
    let mut manager = ScreenManager::new(Size::new(50, 25));
    let path = env::current_dir().unwrap();
    let cursor_pos = manager.screen_mut().writeln(path.to_str().unwrap());
    let mut f = File::create("test.txt").unwrap();
    f.write(format!("Pos: {:?}", cursor_pos).as_bytes());
    console.set_cursor_pos(cursor_pos);
    manager.screen_mut().render(&mut console);

    let mut run = true;
    while run {
        for event in get_input_events(&mut console) {
            if event.is_pressed {
                //                println!("Key {:?} was pressed", event.key);
                match event.key {
                    Key::Escape => run = false,
                    _ => {}
                }
            }
        }
    }
}
