use basic::{CursorMove, Size};
use cli::event::KeyEvent;
use cli::{Console, Event};
use screen::{CursorDel, ScreenManager};

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

    pub fn poll_events(&mut self) -> Vec<Event> {
        self.console.poll_events()
    }

    pub fn write_key(&mut self, key: KeyEvent) {
        let cursor_pos = self.manager.screen_mut().write(key.to_string());
        self.console.set_cursor_pos(cursor_pos);
    }

    pub fn newline(&mut self) {
        let (_, _input) = self.manager.screen_mut().newline(0);
        self.manager.screen_mut().write(self.console.get_dir());
        self.manager.screen_mut().newline(2);
        let cursor_pos = self.manager.screen_mut().write("~ ");
        self.console.set_cursor_pos(cursor_pos);
        //                        println!("Input {}", input);
        //if input == "cd" {
        self.console.set_dir("..");
        //                            println!(" Output {}", console.get_dir());
        //}
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
