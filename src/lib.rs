pub mod terminal;
pub mod game;
mod player;
mod projectile;
mod render; 


#[derive(Debug, Eq, PartialEq)]
enum Team {
    Player, 
    Enemy, 
}