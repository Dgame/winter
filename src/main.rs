extern crate winter;

use winter::basic::Size;
use winter::cli::{Console, InputEvent, Key};
use winter::screen::ScreenManager;

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
    manager.screen_mut().write(console.get_dir());
    manager.screen_mut().newline(2);
    let cursor_pos = manager.screen_mut().write("~ ");
    console.set_cursor_pos(cursor_pos);

    let mut run = true;
    while run {
        for event in get_input_events(&mut console) {
            if event.is_pressed {
                //                println!("Key {:?} was pressed", event.key);
                match event.key {
                    Key::Escape => run = false,
                    Key::Return => {
                        let (_, input) = manager.screen_mut().newline(0);
                        manager.screen_mut().write(console.get_dir());
                        manager.screen_mut().newline(2);
                        let cursor_pos = manager.screen_mut().write("~ ");
                        console.set_cursor_pos(cursor_pos);
                        //                        println!("Input {}", input);
                        //if input == "cd" {
                        console.set_dir("..");
                        //                            println!(" Output {}", console.get_dir());
                        //}
                    }
                    Key::Back => {
                        manager.screen_mut().line_mut().del_left();
                        let cursor_pos = manager.screen().line().cursor().pos();
                        console.set_cursor_pos(cursor_pos);
                    }
                    Key::Delete => {
                        manager.screen_mut().line_mut().del_right();
                        let cursor_pos = manager.screen().line().cursor().pos();
                        console.set_cursor_pos(cursor_pos);
                    }
                    Key::Left => {
                        manager.screen_mut().line_mut().cursor_mut().move_left();
                        let cursor_pos = manager.screen().line().cursor().pos();
                        console.set_cursor_pos(cursor_pos);
                    }
                    Key::Right => {
                        manager.screen_mut().line_mut().cursor_mut().move_right();
                        let cursor_pos = manager.screen().line().cursor().pos();;
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
