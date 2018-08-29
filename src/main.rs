#![feature(range_contains)]
extern crate termios;
mod input;

use input::Input;
use std::io;

fn handle_exit(c: u8) -> bool {
    if c == b'q' {
        // || (0..32).contains(&c)
        return true;
    }

    false
}

fn main() {
    let stdio = io::stdin();
    let mut input = Input::new(0, &stdio);

    input.enable_rawmode();
    loop {
        let char = input.read_key();
        println!("{}\r\n", char);
        if handle_exit(char) {
            break;
        }
    }

    input.disable_rawmode();
}
