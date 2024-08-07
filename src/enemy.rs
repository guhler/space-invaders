use crossterm::{style::Stylize, terminal};

use crate::{game::GameState, render::RenderBuffer};

#[derive(Debug)]
pub enum MovementPattern {
    BackForth { going_left: bool, down: u8 }, 
}

impl MovementPattern {
    fn update(e: &mut Enemy) {
        match e.movement_pattern {
            Self::BackForth { ref mut going_left, ref mut down } => {
                if *down != 0 {
                    *down -= 1;
                    e.pos.1 += 1;
                } else if *going_left {
                    e.pos.0 -= 1;
                    if e.pos.0 == 0 {
                        *going_left = false;
                        *down = Enemy::SHAPE.len() as u8;
                    }
                } else {
                    let (w, _) = terminal::size().unwrap();
                    let w = w - Enemy::SHAPE[0].len() as u16;
                    e.pos.0 += 1;
                    if e.pos.0 == w {
                        *going_left = true;
                        *down = Enemy::SHAPE.len() as u8;
                    }
                }
            }
        }
    }
}

pub struct Enemy {
    // game logic
    pub pos: (u16, u16),
    pub hp: u16,
    pub movement_pattern: MovementPattern, 

    // animation
    pub hurt: bool, 
}

impl Enemy {
    #[rustfmt::skip]
    pub const SHAPE: &'static [&'static str] = &[ 
        " ▄▄▄▄▄ ", 
        "█     █", 
        "█▄█ █▄█", 
        " ▀▀ ▀▀ ", 
    ];

    pub fn new(x: u16, y: u16, movement_pattern: MovementPattern, hp: u16) -> Self {
        Self {
            pos: (x, y),
            hp,
            movement_pattern, 
            hurt: false, 
        }
    }

    pub fn update(gs: &mut GameState) {
        let mut i = 0;
        while i < gs.enemies.len() {
            MovementPattern::update(&mut gs.enemies[i]);
            if gs.enemies[i].hp == 0 {
                gs.enemies.remove(i);
            } else {
                i += 1;
            }
        }
    }

    pub fn render(&mut self, buffer: &mut RenderBuffer) {
        for (y, row) in Self::SHAPE.iter().enumerate() {
            for (x, c) in row.chars().enumerate() {
                let pos_x = self.pos.0 as usize + x; 
                let pos_y = self.pos.1 as usize + y; 
                if pos_x < buffer.width as usize && pos_y < buffer.height as usize {
                    buffer[pos_x][pos_y] = if self.hurt { c.red() } else { c.stylize() };
                }
            }
        }
        self.hurt = false; 
    }

    pub fn take_damage(&mut self) {
        if self.hp > 0 {
            self.hp -= 1;
        }
        self.hurt = true;
    }
}
