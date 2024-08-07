use std::{
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
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

            let (w, h) = terminal::size().unwrap();
            let mut buf = RenderBuffer::new(w, h);
            self.render(&mut buf);
            thread::spawn(move || buf.render());

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

        loop {
            let (w, h) = terminal::size().unwrap();
            let mut buf = RenderBuffer::new(w, h);
            self.render(&mut buf);

            let game_over_display = TextElement::new(
                "--- Game Over ---".red(),
                (w as u16 / 2, h as u16 / 2),
                TextAlign::Center,
            );
            game_over_display.render(&mut buf);
            buf.render();

            match receiver.try_recv() {
                Ok(Event::Key(KeyEvent {
                    code: KeyCode::Esc, ..
                }))
                | Err(mpsc::TryRecvError::Disconnected) => break,
                _ => (),
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

    fn render(&mut self, buffer: &mut RenderBuffer) {
        let (w, h) = terminal::size().unwrap();

        self.projectiles.iter().for_each(|p| p.render(buffer));

        self.enemies.iter_mut().for_each(|e| e.render(buffer));

        self.player.render(buffer);

        let hp_str = self.player.hp.to_string();
        let hp_display = TextElement::new(
            hp_str.as_str().red(),
            (w as u16 - 1, h as u16 - 1),
            TextAlign::Right,
        );
        hp_display.render(buffer);
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
