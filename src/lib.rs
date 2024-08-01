use std::ops::{Add, Div, Shr};

mod enemy;
pub mod game;
mod player;
mod projectile;
mod render;
pub mod terminal;

#[derive(Debug, Eq, PartialEq, Clone)]
enum Team {
    Player,
    Enemy,
}
