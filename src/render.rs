use std::io::{stdout, Write};
use std::ops::{Index, IndexMut};

use crossterm::{
    cursor, queue,
    style::{Print, StyledContent, Stylize},
};

pub struct RenderBuffer {
    buf: Vec<Vec<StyledContent<char>>>,
    pub width: u16,
    pub height: u16,
}

impl RenderBuffer {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            buf: vec![vec![' '.stylize(); height as usize]; width as usize],
            width,
            height,
        }
    }

    pub fn render(self) {
        queue!(stdout(), cursor::MoveTo(0, 0)).unwrap();
        for y in 0..self.height as usize {
            for x in 0..self.width as usize {
                queue!(stdout(), Print(self[x][y])).unwrap();
            }
        }
        stdout().flush().unwrap();
    }
}

impl<I: Into<usize>> Index<I> for RenderBuffer {
    type Output = Vec<StyledContent<char>>;
    fn index(&self, index: I) -> &Self::Output {
        &self.buf[index.into()]
    }
}

impl<I: Into<usize>> IndexMut<I> for RenderBuffer {
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        &mut self.buf[index.into()]
    }
}
