#![allow(dead_code)]

extern crate winapi;
#[macro_use]
extern crate num_derive;
extern crate enumflags;
extern crate num;
extern crate num_traits;
#[macro_use]
extern crate enumflags_derive;

mod basic;
mod buffer;
mod cell;
mod color;
mod console;
mod input;
pub mod terminal;
mod view;
mod viewport;
