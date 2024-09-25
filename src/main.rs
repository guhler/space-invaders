use std::io::stdout;

use crossterm::{
    cursor,
    event::{self, KeyboardEnhancementFlags},
    execute, terminal,
};
use space_invaders::GameState;

#[tokio::main]
async fn main() {
    if !terminal::supports_keyboard_enhancement().unwrap() {
        panic!("Keyboard enhancement not supported");
    }

    let mut game = GameState::new();

    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        quit();
        hook(info);
    }));

    execute!(
        stdout(),
        cursor::Hide,
        terminal::EnterAlternateScreen,
        event::PushKeyboardEnhancementFlags(KeyboardEnhancementFlags::REPORT_EVENT_TYPES),
    )
    .unwrap();

    terminal::enable_raw_mode().unwrap();

    game.run().await;

    quit();
}

fn quit() {
    let _ = terminal::disable_raw_mode();
    let _ = execute!(
        stdout(),
        cursor::Show,
        terminal::LeaveAlternateScreen,
        event::PopKeyboardEnhancementFlags,
    );
}
