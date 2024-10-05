use crossterm::{event::KeyCode, style::Stylize, terminal};

use crate::{projectile::Projectile, render::RenderBuffer, GameState};

pub struct Player {
    pos: (f32, f32),
    vel: (f32, f32),
    reload_time: u16,
    shape: &'static [&'static str],
}

impl Player {
    const RELOAD_TIME: u16 = 4;

    pub fn new(pos: (f32, f32)) -> Self {
        Self {
            pos,
            vel: (0.0, 0.0),
            reload_time: Self::RELOAD_TIME,
            shape: shapes::SHAPE_1,
        }
    }

    pub fn update(gs: &mut GameState) {
        let (w, h) = terminal::size().unwrap();
        gs.player.pos.0 += gs.player.vel.0;
        gs.player.pos.1 += gs.player.vel.1;
        gs.player.pos.0 = gs
            .player
            .pos
            .0
            .clamp(0.0, w as f32 - gs.player.shape[0].len() as f32);
        gs.player.pos.1 = gs
            .player
            .pos
            .1
            .clamp(0.0, h as f32 - gs.player.shape.len() as f32);

        gs.player.vel.0 = match (
            gs.input.is_pressed(&KeyCode::Left),
            gs.input.is_pressed(&KeyCode::Right),
        ) {
            (true, true) | (false, false) => gs.player.vel.0 * 0.8,
            (true, false) => gs.player.vel.0 - 1.0,
            (false, true) => gs.player.vel.0 + 1.0,
        };
        gs.player.vel.1 = match (
            gs.input.is_pressed(&KeyCode::Up),
            gs.input.is_pressed(&KeyCode::Down),
        ) {
            (true, true) | (false, false) => gs.player.vel.1 * 0.8,
            (true, false) => gs.player.vel.1 - 1.0,
            (false, true) => gs.player.vel.1 + 1.0,
        };

        gs.player.reload_time -= 1;
        if gs.player.reload_time == 0 {
            gs.player.reload_time = Self::RELOAD_TIME;
            Self::shoot(gs);
        }
    }

    pub fn render(&self, buf: &mut RenderBuffer) {
        let pos_x = self.pos.0 as u16;
        let pos_y = self.pos.1 as u16;
        for (i, row) in self.shape.iter().enumerate() {
            for (j, c) in row.chars().enumerate() {
                if pos_x + j as u16 >= buf.width() || pos_y + i as u16 >= buf.height() {
                    continue;
                }
                *buf.get_mut(pos_x + j as u16, pos_y + i as u16).unwrap() = c.white();
            }
        }
    }

    fn shoot(gs: &mut GameState) {
        let middle = (
            gs.player.pos.0 + gs.player.shape[0].len() as f32 / 2.0,
            gs.player.pos.1,
        );
        gs.projectiles.push(Projectile::new(middle, (0.0, -1.0)));
    }
}

mod shapes {

    #[rustfmt::skip]
    pub const SHAPE_1: &'static [&'static str] = &[
        r"  /\  ", 
        r" /__\ ", 
        r"| __ |", 
        r"|/  \|"
    ];
}
