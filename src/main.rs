extern crate winter;

use winter::basic::{CursorMove, Size};
use winter::cli::{Event, Key};
use winter::screen::CursorDel;
use winter::terminal::Terminal;

fn main() {
    let mut terminal = Terminal::new(Size::new(25, 50));

    let mut run = true;
    while run {
        for event in terminal.poll_events() {
            match event {
                Event::Key(key) => {
                    if key.is_pressed {
                        //                println!("Key {:?} was pressed", event.key);
                        match key.code {
                            Key::Escape => run = false,
                            Key::Return => terminal.newline(),
                            Key::Back => terminal.del_left(),
                            Key::Delete => terminal.del_right(),
                            Key::Left => terminal.move_left(),
                            Key::Right => terminal.move_right(),
                            _ => terminal.write_key(key),
                        }
                    }
                }
                _ => {}
            }
        }

        terminal.render();
    }
}
