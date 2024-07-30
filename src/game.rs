use std::{sync::mpsc, thread, time::{Duration, Instant}};

use crossterm::{event::{self, Event, KeyCode, KeyEvent}, terminal};

use crate::{player::Player, projectile::Projectile, render::RenderBuffer, Team};



pub struct GameState {
    running: bool, 
    player: Player, 
    projectiles: Vec<Projectile>, 
}

impl GameState {
    const TICK_LENGTH: Duration = Duration::from_millis(25);

    pub fn new() -> Self {
        Self {
            running: false, 
            player: Player::new(), 
            projectiles: vec![], 
        }
    }

    pub fn run(mut self) {
        self.running = true; 

        let (sender, receiver) = mpsc::channel::<event::Event>();
        let _input_thread = std::thread::spawn(move || {
            loop {
                match event::read() {
                    Ok(event) => if sender.send(event).is_err() { break }, 
                    Err(e) => eprintln!("{e}"), 
                }
            }
        });


        while self.running {
            let start = Instant::now();

            self.update();
            self.render();

            while self.running && Instant::now() - start < Self::TICK_LENGTH {
                match receiver.try_recv() {
                    Ok(event) => if !self.handle_input(&event) {
                        self.player.handle_input(&event);
                    }, 
                    Err(mpsc::TryRecvError::Disconnected) => panic!("Input channel disconnected"), 
                    _ => (), 
                }
            }

        }
    }

    fn update(&mut self) {
        let projs = self.player.update(); 
        self.projectiles.extend(projs);
        let mut i = 0; 
        while i < self.projectiles.len() {
            let proj = &mut self.projectiles[i];
            let remove = match proj.team {
                Team::Player => proj.update_player(), 
                Team::Enemy => proj.update_enemy(&mut self.player), 
            };
            if remove {
                self.projectiles.remove(i);
            } else {
                i += 1;
            }
        }
    }

    fn render(&mut self) {
        let (w, h) = terminal::size().unwrap();
        let mut buffer = RenderBuffer::new(w, h);
        self.player.render(&mut buffer);
        for proj in self.projectiles.iter() {
            proj.render(&mut buffer);
        }
        thread::spawn(move || {
            buffer.render();
        });
    }


    fn handle_input(&mut self, event: &Event) -> bool {
        match event {
            Event::Key(KeyEvent {
                code: KeyCode::Esc, 
                ..
            }) => {
                self.running = false;
                true
            }, 
            _ => false, 
        }
    }
}