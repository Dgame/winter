use basic::Size;
use cli::event::KeyEvent;
use cli::Console;
use cli::Event;
use screen::ScreenManager;

pub struct Terminal {
    console: Console,
    manager: ScreenManager,
}

impl Terminal {
    pub fn new(size: Size) -> Self {
        let mut console = Console::new();
        let mut manager = ScreenManager::new(size);
        manager.screen_mut().write_text(console.get_dir());
        manager.screen_mut().newline(2);
        let cursor_pos = manager.screen_mut().write_text("~ ");
        console.set_cursor_pos(cursor_pos);

        Self { console, manager }
    }

    pub fn poll_events(&mut self) -> Vec<Event> {
        self.console.poll_events()
    }

    pub fn write_key(&mut self, key: KeyEvent) {
        let cursor_pos = self.manager.screen_mut().write_text(key.to_string());
        self.console.set_cursor_pos(cursor_pos);
    }

    pub fn newline(&mut self) {
        let (_, _input) = self.manager.screen_mut().newline(0);
        self.manager.screen_mut().write_text(self.console.get_dir());
        self.manager.screen_mut().newline(2);
        let cursor_pos = self.manager.screen_mut().write_text("~ ");
        self.console.set_cursor_pos(cursor_pos);
        //                        println!("Input {}", input);
        //if input == "cd" {
        self.console.set_dir("..");
        //                            println!(" Output {}", console.get_dir());
        //}
    }

    pub fn del_right(&mut self) {
        self.manager.screen_mut().del_right();
        let cursor_pos = self.manager.screen().cursor_pos();
        self.console.set_cursor_pos(cursor_pos);
    }

    pub fn del_left(&mut self) {
        self.manager.screen_mut().del_left();
        let cursor_pos = self.manager.screen().cursor_pos();
        self.console.set_cursor_pos(cursor_pos);
    }

    pub fn move_left(&mut self) {
        self.manager.screen_mut().move_left();
        let cursor_pos = self.manager.screen().cursor_pos();
        self.console.set_cursor_pos(cursor_pos);
    }

    pub fn move_right(&mut self) {
        self.manager.screen_mut().move_right();
        let cursor_pos = self.manager.screen().cursor_pos();
        self.console.set_cursor_pos(cursor_pos);
    }

    pub fn render(&mut self) {
        self.manager.render(&mut self.console);
    }
}
