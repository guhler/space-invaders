use std::io::{stdout, Write};

use crossterm::{cursor, queue, style::{Print, StyledContent, Stylize}};


pub struct RenderBuffer {
    pub buf: Vec<StyledContent<char>>, 
    pub width: u16, 
    pub height: u16, 
}

impl RenderBuffer {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            buf: vec![' '.stylize(); (width * height) as usize], 
            width,  
            height, 
        }
    }

    pub fn render(self) {
        queue!(stdout(), cursor::MoveTo(0, 0)).unwrap();
        for elem in self.buf {
            queue!(stdout(), Print(elem)).unwrap();
        }
        stdout().flush().unwrap();
    }
}