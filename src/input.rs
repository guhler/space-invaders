use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};

pub struct InputState {
    pressed: Vec<KeyCode>,
}

impl InputState {
    pub fn new() -> Self {
        Self {
            pressed: Vec::new(),
        }
    }

    pub fn accept(&mut self, event: &Event) -> bool {
        match event {
            Event::Key(KeyEvent { code, kind, .. }) => match kind {
                KeyEventKind::Release => {
                    debug_assert!(self.pressed.contains(code));
                    for i in (0..self.pressed.len()).rev() {
                        if &self.pressed[i] == code {
                            self.pressed.remove(i);
                            #[cfg(not(debug_assertions))]
                            break;
                        }
                    }
                    true
                }
                _ => {
                    if !self.pressed.contains(code) {
                        self.pressed.push(*code);
                        true
                    } else {
                        false
                    }
                }
            },
            _ => false,
        }
    }

    pub fn is_pressed(&self, code: &KeyCode) -> bool {
        self.pressed.contains(&code)
    }
}
