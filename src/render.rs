use std::io::{stdout, Write};

use crossterm::{
    cursor, execute,
    style::{StyledContent, Stylize},
    terminal,
};

pub struct RenderBuffer {
    buf: Vec<StyledContent<char>>,
    width: u16,
    height: u16,
}

impl RenderBuffer {
    pub fn new(width: u16, height: u16) -> Self {
        let buf = vec![' '.stylize(); (width * height) as usize];
        Self { buf, width, height }
    }

    pub fn width(&self) -> u16 {
        self.width
    }

    pub fn height(&self) -> u16 {
        self.height
    }

    pub fn get(&self, x: u16, y: u16) -> Option<&StyledContent<char>> {
        self.buf.get((x + y * self.width) as usize)
    }

    pub fn get_mut(&mut self, x: u16, y: u16) -> Option<&mut StyledContent<char>> {
        self.buf.get_mut((x + y * self.width) as usize)
    }

    pub fn render(self) {
        assert_eq!((self.width * self.height) as usize, self.buf.len());
        execute!(stdout(), cursor::MoveTo(0, 0)).unwrap();
        let mut str_buf = String::with_capacity((self.width * self.height) as usize);
        for c in self.buf {
            str_buf += &format!("{}", c);
        }
        stdout().write_all(str_buf.as_bytes()).unwrap();
    }
}

impl Default for RenderBuffer {
    fn default() -> Self {
        let (w, h) = terminal::size().unwrap();
        Self::new(w, h)
    }
}
