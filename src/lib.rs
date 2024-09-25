mod input;
mod player;
mod projectile;
mod render;

use crossterm::event::{self, Event, KeyCode, KeyEvent};
use futures::{future::FutureExt, select, StreamExt};
use input::InputState;
use player::Player;
use projectile::Projectile;
use render::RenderBuffer;
use std::time::Duration;

pub struct GameState {
    player: Player,
    projectiles: Vec<Projectile>,
    input: InputState,
}

impl GameState {
    pub const FRAME_RATE: u16 = 30;
    pub fn new() -> Self {
        Self {
            player: Player::new((0.0, 0.0)),
            projectiles: vec![],
            input: InputState::new(),
        }
    }

    pub async fn run(&mut self) {
        let mut input_reader = event::EventStream::new();
        let mut render_delay = futures_timer::Delay::new(Duration::from_secs(1)).fuse();
        let mut update_delay = futures_timer::Delay::new(Duration::from_millis(50)).fuse();

        loop {
            let mut event = input_reader.next().fuse();

            select! {
                _ = render_delay => {
                    render_delay = futures_timer::Delay::new(Duration::from_millis((1000 / Self::FRAME_RATE) as u64)).fuse();
                    self.render();
                },
                _ = update_delay => {
                    update_delay = futures_timer::Delay::new(Duration::from_millis(50)).fuse();
                    self.update();
                },
                e = event => {
                    match e {
                        Some(Ok(Event::Key(KeyEvent {
                            code: KeyCode::Esc,
                            ..
                        }))) => {
                            break;
                        }
                        Some(Ok(e)) => { self.input.accept(&e); },
                        _ => (),
                    }
                }
            }
        }
    }

    fn update(&mut self) {
        Player::update(self);
        Projectile::update(self);
    }

    fn render(&self) {
        let mut buf = RenderBuffer::default();

        self.player.render(&mut buf);
        for p in self.projectiles.iter() {
            p.render(&mut buf);
        }
        buf.render();
    }
}
