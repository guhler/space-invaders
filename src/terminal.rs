use std::io::{stdout, Write};
use crossterm::{cursor, queue, terminal};

pub fn enter() {
    terminal::enable_raw_mode().unwrap();
    queue!(stdout(), 
        terminal::EnterAlternateScreen, 
        cursor::Hide, 
    ).unwrap();
    stdout().flush().unwrap();
}

pub fn exit() {
    queue!(stdout(), terminal::LeaveAlternateScreen, cursor::Show).unwrap();
    stdout().flush().unwrap();
    terminal::disable_raw_mode().unwrap();
}