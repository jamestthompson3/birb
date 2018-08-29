use std::io::prelude::*;
use std::io::Stdin;
use std::os::unix::io::RawFd;
use termios::*;

pub struct Input<'a> {
    fd: RawFd,
    original_termios: Termios,
    stdio: &'a Stdin,
}

impl<'a> Input<'a> {
    pub fn new(fd: RawFd, stdio: &'a Stdin) -> Input {
        let termios = Termios::from_fd(fd).unwrap();
        Input {
            fd,
            stdio,
            original_termios: termios,
        }
    }

    pub fn enable_rawmode(&mut self) {
        let mut termios = self.original_termios;

        termios.c_oflag &= !(OPOST);
        termios.c_iflag &= !(BRKINT | ICRNL | INPCK | ISTRIP | IXON); // turn off ctrl-s, ctrl-m, ctrl-q, read them as bytes
        termios.c_lflag &= !(ECHO | ICANON | ISIG | IEXTEN);
        termios.c_cflag |= CS8;
        //termios.c_cc[VMIN] = 0; // min number of bytes needed before reading
        termios.c_cc[VTIME] = 1; // max amount of time to wait before reading
        tcsetattr(self.fd, TCSAFLUSH, &termios).expect("Failed to enter raw mode");
        tcflush(self.fd, TCIOFLUSH).expect("Failed to flush termios in raw mode");
    }

    pub fn disable_rawmode(&self) {
        tcsetattr(self.fd, TCSAFLUSH, &self.original_termios)
            .expect("Failed to restore terminal state");
    }

    pub fn read_key(&self) -> u8 {
        match self.stdio.lock().bytes().next() {
            Some(character) => character.unwrap(),
            None => 0,
        }
    }
}
