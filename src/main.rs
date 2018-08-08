extern crate winter;

use winter::input::Key;
use winter::terminal::Terminal;

fn main() {
    let mut terminal = Terminal::default();
    //    terminal.write_line("Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo ");
    //    terminal.write_line("dolores et ea rebum.");
    let mut run = true;
    while run {
        for event in terminal.get_input_events() {
            if event.is_pressed {
                //                println!("Key {:?} was pressed", event.key);
                match event.key {
                    Key::Escape => run = false,
                    Key::Return => terminal.write("\n"),
                    _ => {
                        let s = event.key.to_string(event.control);
                        if !s.is_empty() {
                            terminal.write(s);
                        }
                    }
                }
            }
        }

        terminal.display();
    }

    //    let stdin = std::io::stdin();
    //    let mut input = String::new();
    //    input.clear();
    //    terminal.get_input();
    //
    //    stdin.read_line(&mut input).unwrap();
    //
    //    terminal.vsplit();
    //    //    terminal.write_line("Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet.");
    //
    //    terminal.display();
    //
    //    input.clear();
    //    stdin.read_line(&mut input).unwrap();
}
