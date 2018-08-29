use basic::{CursorMove, Size};
use cli::event::KeyEvent;
use cli::{Console, Display, Event};
use screen::{CursorDel, ScreenManager};
use std::io;
use std::process::{Command, Output};

pub struct Terminal {
    console: Console,
    manager: ScreenManager,
}

impl Terminal {
    pub fn new(size: Size) -> Self {
        let mut console = Console::new();
        let mut manager = ScreenManager::new(size);
        manager.screen_mut().write(console.get_dir());
        manager.screen_mut().newline(2);
        let cursor_pos = manager.screen_mut().write("~ ");
        console.set_cursor_pos(cursor_pos);

        Self { console, manager }
    }

    fn clear(&mut self) {
        self.manager.screen_mut().clear();
    }

    pub fn poll_events(&mut self) -> Vec<Event> {
        self.console.poll_events()
    }

    pub fn write_key(&mut self, key: KeyEvent) {
        let cursor_pos = self.manager.screen_mut().write(key.to_string());
        self.console.set_cursor_pos(cursor_pos);
    }

    pub fn newline(&mut self) {
        let (_, input) = self.manager.screen_mut().newline(0);
        let token: Vec<&str> = input.split_whitespace().collect();
        match token.len() {
            0 => {}
            1 => match token[0] {
                "cls" => self.clear(),
                _ => {
                    let output = Command::new(token[0]).output();
                    self.process(output);
                }
            },
            _ => match token[0] {
                "cd" => self.console.set_dir(&token[1..].join(" ")),
                _ => {
                    let output = Command::new(token[0]).args(&token[1..]).output();
                    self.process(output);
                }
            },
        }
        self.manager.screen_mut().newline(0);
        self.manager.screen_mut().write(self.console.get_dir());
        self.manager.screen_mut().newline(2);
        let cursor_pos = self.manager.screen_mut().write("~ ");
        self.console.set_cursor_pos(cursor_pos);
    }

    fn process(&mut self, output: io::Result<Output>) {
        match output {
            Ok(output) => {
                if output.status.success() {
                    self.manager
                        .screen_mut()
                        .write(String::from_utf8_lossy(&output.stdout).trim());
                } else {
                    self.manager
                        .screen_mut()
                        .write(String::from_utf8_lossy(&output.stderr).trim());
                }
            }
            Err(e) => {
                self.manager.screen_mut().write(Display::error(e));
            }
        }
        //        let cursor = self.manager.screen().line().cursor();
        //        let mut pos = Coord::index_to_2d(cursor.index(), 25);
        //        pos.y += 2;
        //        self.console.set_cursor_pos(pos);
        //        println!("{:?}", pos);
    }

    pub fn render(&mut self) {
        self.manager.render(&mut self.console);
    }
}

impl CursorMove for Terminal {
    fn move_left(&mut self) {
        self.manager.screen_mut().move_left();
        let cursor_pos = self.manager.screen().cursor_pos();
        self.console.set_cursor_pos(cursor_pos);
    }

    fn move_right(&mut self) {
        self.manager.screen_mut().move_right();
        let cursor_pos = self.manager.screen().cursor_pos();
        self.console.set_cursor_pos(cursor_pos);
    }
}

impl CursorDel for Terminal {
    fn del_left(&mut self) {
        self.manager.screen_mut().del_left();
        let cursor_pos = self.manager.screen().cursor_pos();
        self.console.set_cursor_pos(cursor_pos);
    }

    fn del_right(&mut self) {
        self.manager.screen_mut().del_right();
        let cursor_pos = self.manager.screen().cursor_pos();
        self.console.set_cursor_pos(cursor_pos);
    }
}
