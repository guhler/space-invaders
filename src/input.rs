use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState};

pub struct InputState {
    pub left_arrow: bool,
    pub up_arrow: bool,
    pub right_arrow: bool,
    pub down_arrow: bool,
}

impl InputState {
    pub fn new() -> Self {
        Self {
            left_arrow: false,
            up_arrow: false,
            right_arrow: false,
            down_arrow: false,
        }
    }

    pub fn accept(&mut self, event: &Event) -> bool {
        match event {
            Event::Key(KeyEvent {
                code: KeyCode::Up,
                kind,
                ..
            }) => {
                self.up_arrow = *kind == KeyEventKind::Press;
                true
            }
            Event::Key(KeyEvent {
                code: KeyCode::Right,
                kind,
                ..
            }) => {
                self.right_arrow = *kind == KeyEventKind::Press;
                true
            }
            Event::Key(KeyEvent {
                code: KeyCode::Down,
                kind,
                ..
            }) => {
                self.down_arrow = *kind == KeyEventKind::Press;
                true
            }
            Event::Key(KeyEvent {
                code: KeyCode::Left,
                kind,
                ..
            }) => {
                self.left_arrow = *kind == KeyEventKind::Press;
                true
            }
            _ => false,
        }
    }
}
