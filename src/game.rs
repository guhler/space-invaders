use std::{
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    terminal,
};

use crate::{enemy::Enemy, player::Player, projectile::Projectile, render::RenderBuffer};

pub struct GameState {
    pub(crate) running: bool,
    pub(crate) player: Player,
    pub(crate) projectiles: Vec<Projectile>,
    pub(crate) enemies: Vec<Enemy>,
}

impl GameState {
    const TICK_LENGTH: Duration = Duration::from_millis(25);

    pub fn new() -> Self {
        Self {
            running: false,
            player: Player::new(),
            projectiles: vec![],
            enemies: vec![Enemy::new(10.0, 0.0, 7)],
        }
    }

    pub fn run(mut self) {
        self.running = true;

        let (sender, receiver) = mpsc::channel::<event::Event>();
        let _input_thread = std::thread::spawn(move || loop {
            match event::read() {
                Ok(event) => {
                    if sender.send(event).is_err() {
                        break;
                    }
                }
                Err(e) => eprintln!("{e}"),
            }
        });

        while self.running {
            let start = Instant::now();

            self.update();
            self.render();

            while self.running && Instant::now() - start < Self::TICK_LENGTH {
                match receiver.try_recv() {
                    Ok(event) => {
                        if !self.handle_input(&event) {
                            self.player.handle_input(&event);
                        }
                    }
                    Err(mpsc::TryRecvError::Disconnected) => panic!("Input channel disconnected"),
                    _ => (),
                }
            }
        }
    }

    fn update(&mut self) {
        Player::update(self);

        Enemy::update(self);

        Projectile::update(self);
    }

    fn render(&mut self) {
        let (w, h) = terminal::size().unwrap();
        let mut buffer = RenderBuffer::new(w, h);

        self.projectiles.iter().for_each(|p| p.render(&mut buffer));

        self.enemies.iter_mut().for_each(|e| e.render(&mut buffer));

        self.player.render(&mut buffer);

        thread::spawn(move || {
            buffer.render();
        });
    }

    fn handle_input(&mut self, event: &Event) -> bool {
        match event {
            Event::Key(KeyEvent {
                code: KeyCode::Esc, ..
            }) => {
                self.running = false;
                true
            }
            _ => false,
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
