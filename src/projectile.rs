use crossterm::{style::Stylize, terminal};

use crate::{render::RenderBuffer, GameState};

pub struct Projectile {
    pos: (f32, f32),
    vel: (f32, f32),
}

impl Projectile {
    pub fn new(pos: (f32, f32), vel: (f32, f32)) -> Self {
        Self { pos, vel }
    }

    pub fn update(gs: &mut GameState) {
        let mut i = 0;
        while i < gs.projectiles.len() {
            gs.projectiles[i].pos.0 += gs.projectiles[i].vel.0;
            gs.projectiles[i].pos.1 += gs.projectiles[i].vel.1;

            let (w, h) = terminal::size().unwrap();
            if gs.projectiles[i].pos.0 < 0.0
                || gs.projectiles[i].pos.0 >= w as f32
                || gs.projectiles[i].pos.1 < 0.0
                || gs.projectiles[i].pos.1 >= h as f32
            {
                gs.projectiles.remove(i);
            } else {
                i += 1;
            }
        }
    }

    pub fn render(&self, buf: &mut RenderBuffer) {
        *buf.get_mut(self.pos.0 as u16, self.pos.1 as u16).unwrap() = match self.vel.1 / self.vel.0
        {
            -4.0..=-0.5 => '\\',
            -0.5..=0.5 => '-',
            0.5..4.0 => '/',
            _ => '|',
        }
        .stylize();
    }
}
