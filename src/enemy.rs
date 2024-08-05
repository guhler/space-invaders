use crossterm::style::Stylize;

use crate::{game::GameState, render::RenderBuffer};

pub struct Enemy {
    pub pos: (f32, f32),
    pub vel: (f32, f32),
    pub hp: u16,
    pub alive: bool,
}

impl Enemy {
    #[rustfmt::skip]
    pub const SHAPE: &'static [&'static str] = &[ 
        " ▄▄▄▄▄ ", 
        "█     █", 
        "█▄█ █▄█", 
        " ▀▀ ▀▀ ", 
    ];

    pub fn new(x: f32, y: f32, hp: u16) -> Self {
        Self {
            pos: (x, y),
            vel: (0.0, 0.0),
            hp,
            alive: true,
        }
    }

    pub fn update(gs: &mut GameState) {
        let mut i = 0;
        while i < gs.enemies.len() {
            if gs.enemies[i].hp == 0 {
                gs.enemies[i].alive = false;
            }
            gs.enemies[i].pos.0 += gs.enemies[i].vel.0;
            gs.enemies[i].pos.1 += gs.enemies[i].vel.1;
            i += 1;
        }
    }

    pub fn render(&self, buffer: &mut RenderBuffer) {
        for (y, row) in Self::SHAPE.iter().enumerate() {
            for (x, c) in row.chars().enumerate() {
                let pos_x = self.pos.0 as usize + x; 
                let pos_y = self.pos.1 as usize + y; 
                if pos_x < buffer.width as usize && pos_y < buffer.height as usize {
                    buffer[pos_x][pos_y] = c.stylize(); 
                }
            }
        }
    }
}
