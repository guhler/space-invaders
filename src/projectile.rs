use crossterm::{style::Stylize, terminal};

use crate::{
    enemy::Enemy,
    game::{self, GameState},
    player::Player,
    render::RenderBuffer,
    Team,
};

#[derive(Debug, Clone)]
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

    pub fn update(gs: &mut GameState) {
        let mut i = 0;
        while i < gs.projectiles.len() {
            gs.projectiles[i].pos.0 += gs.projectiles[i].vel.0;
            gs.projectiles[i].pos.1 += gs.projectiles[i].vel.1;

            Self::check_hits(gs, i);

            let (w, h) = terminal::size().unwrap();
            if !(0.0..w as f32 - 1.0).contains(&gs.projectiles[i].pos.0)
                || !(0.0..h as f32 - 1.0).contains(&gs.projectiles[i].pos.1)
            {
                gs.projectiles.remove(i);
            } else {
                i += 1;
            }
        }
    }

    fn check_hits(gs: &mut GameState, i: usize) {
        let pos = (
            gs.projectiles[i].pos.0 as usize,
            gs.projectiles[i].pos.1 as usize,
        );
        match gs.projectiles[i].team {
            Team::Enemy => {
                let ppos = (gs.player.pos.0 as usize, gs.player.pos.1 as usize);
                if game::shapes_collide(&["-"], pos, Player::SHIP, ppos) {
                    gs.player.take_damage();
                }
            }
            Team::Player => {
                for j in 0..gs.enemies.len() {
                    let epos = (gs.enemies[j].pos.0 as usize, gs.enemies[j].pos.1 as usize);
                    if game::shapes_collide(&["-"], pos, Enemy::SHAPE, epos) {
                        gs.enemies[j].take_damage();
                    }
                }
            }
        }
    }

    pub fn render(&self, buffer: &mut RenderBuffer) {
        let c = match self.vel.1 as f32 / self.vel.0 as f32 {
            -0.5..=0.5 => '-',
            0.5..=4.0 => '\\',
            -4.0..=-0.5 => '/',
            (4.0..) | (..=-4.0) => '|',
            _ => panic!("Unexpected slope"),
        };

        if self.pos.0 >= 0.0
            && self.pos.0 < buffer.width as f32
            && self.pos.1 >= 0.0
            && self.pos.1 < buffer.height as f32
        {
            buffer[self.pos.0 as usize][self.pos.1 as usize] = c.stylize();
        }
    }
}
