extern crate termios;
mod input;
mod screen;

use input::Input;
use screen::Screen;
use std::io;

fn ctrl_key(c: u8) -> u8 {
    c & 0x1f
}

fn handle_exit(c: u8) -> bool {
    if c == ctrl_key(b'q') {
        return true;
    }

    false
}

fn main() {
    let stdio = io::stdin();
    let mut input = Input::new(0, &stdio);

    let mut output = io::stdout();
    input.enable_rawmode();
    loop {
        Screen::refresh(&mut output);
        let char = input.read_key();
        //io::stdout().write(&[char]).unwrap();
        if handle_exit(char) {
            break;
        }
    }

    input.disable_rawmode();

    Screen::refresh(&mut output);
}
