extern crate winter;

use std::env;
use std::process::Command;
use winter::basic::Size;
use winter::console::Console;
use winter::input::InputEvent;
use winter::input::Key;
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
    let path = env::current_dir().unwrap();
    let (cursor_pos, _) = manager.screen_mut().writeln(path.to_str().unwrap());
    console.set_cursor_pos(cursor_pos);

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

                        let args: Vec<&str> = input.split_whitespace().collect();
                        println!("{:?}", args);

                        //                        let (cursor_pos, _) = match Command::new("cd").args([".."].iter()).output() {
                        //                            Ok(output) => {
                        //                                manager.screen_mut().writeln(String::from_utf8(output.stdout).unwrap())
                        //                            },
                        //                            Err(e) => manager.screen_mut().writeln(e.to_string()),
                        //                        };
                        console.set_cursor_pos(cursor_pos);
                    }
                    Key::Back => {
                        let cursor_pos = manager.screen_mut().line_mut().del_left();
                        console.set_cursor_pos(cursor_pos);
                    }
                    Key::Delete => {
                        let cursor_pos = manager.screen_mut().line_mut().del_right();
                        console.set_cursor_pos(cursor_pos);
                    }
                    Key::Left => {
                        manager.screen_mut().line_mut().cursor_mut().move_left();
                        let cursor_pos = manager.screen_mut().line().get_cursor_pos();
                        console.set_cursor_pos(cursor_pos);
                    }
                    Key::Right => {
                        manager.screen_mut().line_mut().cursor_mut().move_right();
                        let cursor_pos = manager.screen_mut().line().get_cursor_pos();
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
