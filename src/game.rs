use std::time::Duration;

use futures::{future::FutureExt, select, StreamExt};

use futures_timer::Delay;

use crossterm::{
    event::{Event, EventStream, KeyCode, KeyEvent},
    style::Stylize,
    terminal,
};

use crate::{
    enemy::{Enemy, MovementPattern},
    player::Player,
    projectile::Projectile,
    render::{RenderBuffer, TextAlign, TextElement},
};

pub struct GameState {
    // game logic
    pub(crate) running: bool,
    pub(crate) player: Player,
    pub(crate) projectiles: Vec<Projectile>,
    pub(crate) enemies: Vec<Enemy>,
    // animation
}

impl GameState {
    const TICK_LENGTH: Duration = Duration::from_millis(25);

    pub fn new() -> Self {
        let movement_pattern = MovementPattern::BackForth {
            going_left: true,
            down: 0,
        };
        Self {
            running: false,
            player: Player::new(),
            projectiles: vec![],
            enemies: vec![Enemy::new(10, 0, movement_pattern, 100)],
        }
    }

    pub async fn run(mut self) {
        self.running = true;

        let mut update_timer = Delay::new(Duration::from_millis(20)).fuse();
        let mut render_timer = Delay::new(Duration::from_millis(1000)).fuse();
        let mut event_reader = EventStream::new();

        while self.running {
            let mut maybe_event = event_reader.next().fuse();
            select! {
                _ = update_timer => {
                    update_timer = Delay::new(Duration::from_millis(20)).fuse();
                    self.update();
                },
                _ = render_timer => {
                    render_timer = Delay::new(Duration::from_millis(1000 / 30)).fuse();
                    self.render();
                },
                event = maybe_event => if let Some(Ok(e)) = event {
                    self.handle_input(&e);
                },
            }
        }
    }

    fn update(&mut self) {
        Player::update(self);
        if !self.player.is_alive() {
            self.running = false;
        }

        Enemy::update(self);
        Projectile::update(self);
    }

    fn render(&mut self) {
        let (w, h) = terminal::size().unwrap();
        let mut buffer = RenderBuffer::new(w, h);

        self.projectiles.iter().for_each(|p| p.render(&mut buffer));
        self.enemies.iter_mut().for_each(|e| e.render(&mut buffer));
        self.player.render(&mut buffer);

        let hp_str = self.player.hp.to_string();
        let hp_display = TextElement::new(
            // should be part of gamestate?
            hp_str.as_str().red(),
            (w as u16 - 1, h as u16 - 1),
            TextAlign::Right,
        );
        hp_display.render(&mut buffer);

        buffer.render();
    }

    fn handle_input(&mut self, event: &Event) -> bool {
        match event {
            Event::Key(KeyEvent {
                code: KeyCode::Esc, ..
            }) => {
                self.running = false;
                true
            }
            _ => {
                self.player.handle_input(event);
                false
            }
        }
    }
}

pub(crate) fn shapes_collide(
    a: &[&str],
    pos_a: (usize, usize),
    b: &[&str],
    pos_b: (usize, usize),
) -> bool {
    for (ay, row) in a.iter().enumerate() {
        for (ax, ac) in row.chars().enumerate() {
            if ac == ' ' {
                continue;
            }
            for (by, row) in b.iter().enumerate() {
                for (bx, bc) in row.chars().enumerate() {
                    if bc == ' ' {
                        continue;
                    }
                    let pos_ax = ax + pos_a.0;
                    let pos_ay = ay + pos_a.1;
                    let pos_bx = bx + pos_b.0;
                    let pos_by = by + pos_b.1;
                    if pos_ax == pos_bx && pos_ay == pos_by {
                        return true;
                    }
                }
            }
        }
    }
    false
}
