use crossterm::{
    event::{Event, KeyCode, KeyEvent, KeyEventKind},
    style::Stylize,
    terminal,
};

use crate::{game::GameState, projectile::Projectile, render::RenderBuffer, Team};

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
    #[rustfmt::skip]
    const SHIP: &'static [&'static str] = &[
        r"  /\  ", 
        r" /__\ ", 
        r"| __ |", 
        r"|/  \|"
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

    pub fn update(gs: &mut GameState) {
        let pos = gs.player.pos;
        let vel = gs.player.vel;
        let mut i = 10;
        while i >= 0 {
            gs.player.pos.0 = pos.0 + vel.0 * i as f32 / 10.0;
            gs.player.pos.1 = pos.1 + vel.1 * i as f32 / 10.0;
            break;
            // if !Self::collides_with_enemies(gs) {
            // break;
            // }
            i -= 1;
        }

        if i < 10 {
            // gs.player.take_damage();
        }
        if i == 0 {
            println!("Player stuck");
        }

        gs.player.vel.0 += gs.player.acc.0;
        gs.player.vel.0 = gs.player.vel.0.clamp(-Self::MAX_VEL_X, Self::MAX_VEL_X);
        gs.player.acc.0 = match (gs.player.left_arrow_down, gs.player.right_arrow_down) {
            (true, true) | (false, false) => {
                gs.player.vel.0 *= 0.9;
                if gs.player.vel.0.abs() < 0.5 {
                    gs.player.vel.0 = 0.0;
                }
                0.0
            }
            (true, false) => -1.0,
            (false, true) => 1.0,
        };

        gs.player.vel.1 += gs.player.acc.1;
        gs.player.vel.1 = gs.player.vel.1.clamp(-Self::MAX_VEL_Y, Self::MAX_VEL_Y);
        gs.player.acc.1 = match (gs.player.up_arrow_down, gs.player.down_arrow_down) {
            (true, true) | (false, false) => {
                gs.player.vel.1 *= 0.9;
                if gs.player.vel.1.abs() < 0.5 {
                    gs.player.vel.1 = 0.0;
                }
                0.0
            }
            (true, false) => -2.0,
            (false, true) => 1.0,
        };

        Self::check_walls(gs);

        gs.player.reload_time -= 1;
        if gs.player.reload_time == 0 {
            gs.player.reload_time = Self::PROJECTILE_COOLDOWN;
            gs.projectiles.extend(gs.player.spawn_projectiles());
        }
    }

    pub fn render(&mut self, buffer: &mut RenderBuffer) {
        for (y, row) in Self::SHIP.iter().enumerate() {
            for (x, c) in row.chars().enumerate() {
                let pos_x = x + self.pos.0 as usize;
                let pos_y = y + self.pos.1 as usize;
                if pos_x < buffer.width as usize && pos_y < buffer.height as usize {
                    buffer[pos_x][pos_y] = c.stylize();
                }
            }
        }
    }

    pub fn handle_input(&mut self, event: &Event) {
        use KeyCode::*;
        use KeyEventKind::*;
        match event {
            Event::Key(KeyEvent { code, kind, .. }) => match (code, kind) {
                (Left, Press) => self.left_arrow_down = true,
                (Left, Release) => self.left_arrow_down = false,
                (Right, Press) => self.right_arrow_down = true,
                (Right, Release) => self.right_arrow_down = false,
                (Up, Press) => self.up_arrow_down = true,
                (Up, Release) => self.up_arrow_down = false,
                (Down, Press) => self.down_arrow_down = true,
                (Down, Release) => self.down_arrow_down = false,
                _ => (),
            },
            _ => (),
        }
    }

    fn spawn_projectiles(&self) -> Vec<Projectile> {
        vec![
            Projectile::new(self.pos.0 + 3.0, self.pos.1, 0.0, -2.0, Team::Player),
            Projectile::new(self.pos.0 + 3.0, self.pos.1, -2.0, -2.0, Team::Player),
            Projectile::new(self.pos.0 + 3.0, self.pos.1, 2.0, -2.0, Team::Player),
        ]
    }

    fn collides_with_enemies(gs: &mut GameState) -> bool {
        todo!()
    }

    fn check_walls(gs: &mut GameState) {
        println!("{}", gs.player.pos.1);
        let (w, h) = terminal::size().unwrap();
        let (w, h) = (
            w as usize - Self::SHIP[0].len(),
            h as usize - Self::SHIP.len(),
        );
        if !(0.0..=w as f32).contains(&gs.player.pos.0) {
            gs.player.vel.0 = 0.0;
            gs.player.acc.0 = 0.0;
        }
        if !(0.0..=h as f32).contains(&gs.player.pos.1) {
            gs.player.vel.1 = 0.0;
            gs.player.acc.1 = 0.0;
        }
        gs.player.pos.0 = gs.player.pos.0.clamp(0.0, w as f32);
        gs.player.pos.1 = gs.player.pos.1.clamp(0.0, w as f32);
    }

    fn take_damage(&mut self) {
        todo!()
    }
}
