use specs::{System, Write};

use tcod::input::{self, Event, Key, Mouse};

use crate::{InputMapping, VirtualKey};

#[derive(Default)]
pub struct Input;

impl<'a> System<'a> for Input {
    type SystemData = Write<'a, InputMapping>;

    fn run(&mut self, mut im: Self::SystemData) {
        // fetch input, this is done here to avoid being parallel executed by Specs
        use tcod::input::KeyCode::*;

        match input::check_for_event(input::MOUSE | input::KEY_PRESS) {
            Some((_, Event::Mouse(m))) => {
                *im = InputMapping {
                    key: None,
                    mouse: Some((m.cx as i32, m.cy as i32)),
                };
            }
            Some((_, Event::Key(k))) => {
                let vkey = match k {
                    Key { code: Up, .. } | Key { code: NumPad8, .. } => VirtualKey::MoveUp,
                    Key { code: Down, .. } | Key { code: NumPad2, .. } => VirtualKey::MoveDown,
                    Key { code: Left, .. } | Key { code: NumPad4, .. } => VirtualKey::MoveLeft,
                    Key { code: Right, .. } | Key { code: NumPad6, .. } => VirtualKey::MoveRight,
                    Key { code: Escape, .. } => VirtualKey::Exit,
                    _ => VirtualKey::NoAction,
                };

                *im = InputMapping {
                    key: Some(vkey),
                    mouse: None,
                };
            }
            _ => {
                *im = Default::default();
            }
        }
    }
}
