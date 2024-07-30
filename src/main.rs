use std::io::stdout;

use crossterm::{self, style, execute};
use space_invaders::terminal;


fn main() {
    terminal::enter();
    let _ = execute!(stdout(), 
        style::SetBackgroundColor(style::Color::Rgb { r: 0, g: 0, b: 15 })
    ); 


    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        terminal::exit();
        hook(info);
    }));

    let game = space_invaders::game::GameState::new();
    game.run();

    terminal::exit();
}
