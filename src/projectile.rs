use crossterm::{style::Stylize, terminal};

use crate::{player::Player, render::RenderBuffer, Team};


pub struct Projectile {
    pos: (f32, f32), 
    vel: (f32, f32), 
    pub team: Team, 
}

impl Projectile {
    pub fn new(x: f32, y: f32, vel_x: f32, vel_y: f32, team: Team) -> Self {
        Self {
            pos: (x, y), 
            vel: (vel_x, vel_y), 
            team, 
        }
    }

    pub fn update_player(&mut self) -> bool {
        assert_eq!(self.team, Team::Player);
        self.pos.0 += self.vel.0; 
        self.pos.1 += self.vel.1; 

        let (w, h) = terminal::size().unwrap();
        !(0.0..w as f32 - 1.0).contains(&self.pos.0) ||
        !(0.0..h as f32 - 1.0).contains(&self.pos.1)
    }

    pub fn update_enemy(&mut self, _player: &mut Player) -> bool {
        assert_eq!(self.team, Team::Enemy);
        todo!()
    }

    pub fn render(&self, buffer: &mut RenderBuffer) {
        let c = match self.vel.1 as f32 / self.vel.0 as f32 {
            -0.5..=0.5 => '-', 
            0.5..=4.0 => '\\', 
            -4.0..=-0.5 => '/', 
            (4.0..) | (..=-4.0) => '|', 
            _ => panic!("Unexpected slope"), 
        };

        if self.pos.0 >= 0.0 && self.pos.0 < buffer.width as f32 &&
            self.pos.1 >= 0.0 && self.pos.1 < buffer.height as f32 {
            buffer.buf[self.pos.0 as usize + self.pos.1 as usize * buffer.width as usize] = c.stylize();
        }
    }
}