use crossterm::{event::{Event, KeyCode, KeyEvent, KeyEventKind}, style::Stylize, terminal};

use crate::{projectile::Projectile, render::RenderBuffer, Team};


pub struct Player {
    pos: (f32, f32), 
    vel: (f32, f32), 
    acc: (f32, f32), 
    reload_time: u8, 
    left_arrow_down: bool, 
    right_arrow_down: bool, 
    up_arrow_down: bool, 
    down_arrow_down: bool, 
}

impl Player {
    const SHIP: &'static [&'static str] = &[
        r"  /\  ", 
        r" /__\ ", 
        r"| __ |", 
        r"|/  \|", 
    ];

    const MAX_VEL_X: f32 = 2.0;
    const MAX_VEL_Y: f32 = 1.0;

    const PROJECTILE_COOLDOWN: u8 = 10;

    pub fn new() -> Self {
        Self {
            pos: (0.0, 0.0), 
            vel: (0.0, 0.0), 
            acc: (0.0, 0.0), 
            reload_time: Self::PROJECTILE_COOLDOWN, 
            left_arrow_down: false, 
            right_arrow_down: false, 
            up_arrow_down: false, 
            down_arrow_down: false, 
        }
    }

    pub fn update(&mut self) -> Vec<Projectile> {
        let (w, h) = terminal::size().unwrap();
        self.pos.0 += self.vel.0; 
        self.pos.1 += self.vel.1; 
        self.pos.0 = self.pos.0.clamp(0.0, (w - 6) as f32); 
        self.pos.1 = self.pos.1.clamp(0.0, (h - 4) as f32);

        self.vel.0 += self.acc.0; 
        self.vel.1 += self.acc.1; 
        self.vel.0 = self.vel.0.clamp(-Self::MAX_VEL_X, Self::MAX_VEL_X);
        self.vel.1 = self.vel.1.clamp(-Self::MAX_VEL_Y, Self::MAX_VEL_Y);

        self.acc.0 = match (self.left_arrow_down, self.right_arrow_down) {
            (true, true) | (false, false) => {
                self.vel.0 *= 0.9;
                if self.vel.0.abs() < 0.5 {
                    self.vel.0 = 0.0;
                }
                0.0
            }, 
            (true, false) => -1.0, 
            (false, true) => 1.0, 
        };
        self.acc.1 = match (self.up_arrow_down, self.down_arrow_down) {
            (true, true) | (false, false) => {
                self.vel.1 *= 0.9; 
                if self.vel.1.abs() < 0.5 {
                    self.vel.1 = 0.0;
                }
                0.0
            }, 
            (true, false) => -2.0, 
            (false, true) => 1.0, 
        };

        self.reload_time -= 1; 
        if self.reload_time == 0 {
            self.reload_time = Self::PROJECTILE_COOLDOWN; 
            vec![
                Projectile::new(self.pos.0 + 3.0, self.pos.1, 0.0, -1.0, Team::Player), 
                Projectile::new(self.pos.0 + 3.0, self.pos.1, -1.0, -1.0, Team::Player), 
                Projectile::new(self.pos.0 + 3.0, self.pos.1, 1.0, -1.0, Team::Player), 
            ]
        } else { vec![] }
    }

    pub fn render(&mut self, buffer: &mut RenderBuffer) {
        for (y, row) in Self::SHIP.iter().enumerate() {
            for (x, c) in row.chars().enumerate() {
                let pos_x = x + self.pos.0 as usize; 
                let pos_y = y + self.pos.1 as usize; 
                if pos_x < buffer.width as usize && pos_y < buffer.height as usize {
                    buffer.buf[pos_x + buffer.width as usize * pos_y] = c.stylize();
                }
            }
        }
    }

    pub fn handle_input(&mut self, event: &Event) {
        use KeyEventKind::*;
        use KeyCode::*;
        match event {
            Event::Key(KeyEvent {
                code, 
                kind, 
                ..
            }) => match (code, kind) {
                (Left, Press) => self.left_arrow_down = true, 
                (Left, Release) => self.left_arrow_down = false, 
                (Right, Press) => self.right_arrow_down = true, 
                (Right, Release) => self.right_arrow_down = false, 
                (Up, Press) => self.up_arrow_down = true, 
                (Up, Release) => self.up_arrow_down = false, 
                (Down, Press) => self.down_arrow_down = true, 
                (Down, Release) => self.down_arrow_down = false, 
                _ => (), 
            }
            _ => ()
        }
    }
}