#![allow(dead_code)]

extern crate winapi;

use terminal::Terminal;

mod basic;
mod buffer;
mod cell;
mod color;
mod console;
mod view;
mod viewport;
mod terminal;

fn main() {
    let mut terminal = Terminal::default();
//    terminal.write_line("Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo ");
//    terminal.write_line("dolores et ea rebum.");
    terminal.display();

    let stdin = std::io::stdin();
    let mut input = String::new();
    input.clear();
    stdin.read_line(&mut input).unwrap();

    terminal.vsplit();
//    terminal.write_line("Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet.");

    terminal.display();

    input.clear();
    stdin.read_line(&mut input).unwrap();
}
