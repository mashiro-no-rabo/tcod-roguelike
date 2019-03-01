use specs::{System, Write};
use tcod::input::{self, Event, Key};

use crate::{InputMapping, VirtualKey};

pub struct Input;

impl<'a> System<'a> for Input {
    type SystemData = Write<'a, InputMapping>;

    fn run(&mut self, mut im: Self::SystemData) {
        match input::check_for_event(input::MOUSE | input::KEY_PRESS) {
            Some((_, Event::Mouse(m))) => {
                *im = InputMapping {
                    key: None,
                    mouse: Some((m.cx as i32, m.cy as i32)),
                };
            }
            Some((_, Event::Key(k))) => {
                *im = InputMapping {
                    key: Some(VirtualKey::NoAction),
                    mouse: None,
                }
            }
            _ => {
                *im = Default::default();
            }
        }
    }
}
