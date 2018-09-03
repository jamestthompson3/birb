use std::io::{Stdout, Write};

static CLEAR_SCREEN: &[u8] = b"\x1b[2J";
static POSITION_CURSOR: &[u8] = b"\x1b[H";

pub struct Screen<'a> {
    stdio: &'a Stdout,
}
// io::stdout().write(&[char]).unwrap();
impl<'a> Screen<'a> {
    pub fn refresh(stdout: &'a mut Stdout) {
        stdout
            .write(CLEAR_SCREEN)
            .expect("Could not write to stdout");
        stdout
            .write(POSITION_CURSOR)
            .expect("Could not position cursor");

        draw_rows(stdout);

        stdout
            .write(POSITION_CURSOR)
            .expect("Could not position cursor");
    }
}

fn draw_rows(stdout: &mut Stdout) {
    for _ in 0..24 {
        stdout.write(b"~\r\n").expect("Could not draw tildes");
    }
}
