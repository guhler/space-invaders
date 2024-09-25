mod input;
mod player;
mod projectile;
mod render;

use crossterm::event::{self, Event, KeyCode, KeyEvent};
use futures::{future::FutureExt, lock::Mutex, select, StreamExt};
use input::InputState;
use player::Player;
use projectile::Projectile;
use render::RenderBuffer;
use std::{future, sync::Arc, time::Duration};

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

    pub async fn run(self) {
        let s = Arc::new(Mutex::new(self));
        let mut input_reader = event::EventStream::new();
        let render = async {
            loop {
                futures_timer::Delay::new(Duration::from_millis((1000 / Self::FRAME_RATE) as u64))
                    .await;
                s.lock().await.render().await;
            }
        }
        .fuse();
        tokio::pin!(render);
        let update = async {
            loop {
                futures_timer::Delay::new(Duration::from_millis(50)).await;
                s.lock().await.update();
            }
        }
        .fuse();
        tokio::pin!(update);

        loop {
            let mut event = input_reader.next().fuse();

            select! {
                _ = render => {},
                _ = update => {},
                e = event => {
                    match e {
                        Some(Ok(Event::Key(KeyEvent {
                            code: KeyCode::Esc,
                            ..
                        }))) => {
                            break;
                        }
                        Some(Ok(e)) => { s.lock().await.input.accept(&e); },
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

    fn render(&self) -> impl future::Future {
        let mut buf = RenderBuffer::default();

        self.player.render(&mut buf);
        for p in self.projectiles.iter() {
            p.render(&mut buf);
        }
        buf.render()
    }
}
