use std::io::{stdout, Write};
use std::ops::{Index, IndexMut};
use std::slice::SliceIndex;

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

    pub fn get<I>(&self, index: I) -> Option<&<I as SliceIndex<[Vec<StyledContent<char>>]>>::Output>
    where
        I: SliceIndex<[Vec<StyledContent<char>>]>,
    {
        self.buf.get(index)
    }

    pub fn get_mut<I>(
        &mut self,
        index: I,
    ) -> Option<&mut <I as SliceIndex<[Vec<StyledContent<char>>]>>::Output>
    where
        I: SliceIndex<[Vec<StyledContent<char>>]>,
    {
        self.buf.get_mut(index)
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

pub enum TextAlign {
    Left,
    Center,
    Right,
}

pub struct TextElement<'a> {
    content: StyledContent<&'a str>,
    pos: (u16, u16),
    align: TextAlign,
}

impl<'a> TextElement<'a> {
    pub fn new(content: StyledContent<&'a str>, pos: (u16, u16), align: TextAlign) -> Self {
        Self {
            content,
            pos,
            align,
        }
    }

    pub fn render(&self, buffer: &mut RenderBuffer) {
        let pos = match self.align {
            TextAlign::Left => self.pos,
            TextAlign::Center => (
                self.pos.0 - self.content.content().len().div_ceil(2) as u16,
                self.pos.1,
            ),
            TextAlign::Right => (self.pos.0 - self.content.content().len() as u16, self.pos.1),
        };
        for (i, c) in self.content.content().chars().enumerate() {
            if let Some(b) = buffer
                .get_mut(pos.0 as usize + i)
                .map(|col| col.get_mut(pos.1 as usize))
                .flatten()
            {
                *b = self.content.style().apply(c);
            }
        }
    }
}
