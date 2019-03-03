use specs::{prelude::Resources, Write, System};

use tcod::colors;
use tcod::console::*;
use tcod::input::{self, Event, Key, Mouse};
use tcod::map::{FovAlgorithm, Map as FovMap};
use tcod::Color;

use crate::consts::*;
use crate::{InputMapping, VirtualKey};

#[derive(Default)]
pub struct TcodIntegration {
    tcod: Option<Tcod>,
}

struct Tcod {
    root: Root,
    map: Offscreen,
    panel: Offscreen,
    fov: FovMap,
}

impl<'a> System<'a> for TcodIntegration {
    type SystemData = Write<'a, InputMapping>;

    fn run(&mut self, mut im: Self::SystemData) {
        // fetch input, this is done here to avoid being parallel executed by Specs
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

        self.tcod.as_mut().map(|t| {
            t.root.set_default_foreground(colors::WHITE);
            t.root.flush();
        });
    }

    fn setup(&mut self, res: &mut Resources) {
        use specs::prelude::SystemData;
        Self::SystemData::setup(res);

        let root = Root::initializer()
            .font("arial10x10.png", FontLayout::Tcod)
            .font_type(FontType::Greyscale)
            .size(SCREEN_WIDTH, SCREEN_HEIGHT)
            .title("MechRogue")
            .init();

        let map = Offscreen::new(MAP_WIDTH, MAP_HEIGHT);
        let panel = Offscreen::new(SCREEN_WIDTH, PANEL_HEIGHT);
        let fov = FovMap::new(MAP_WIDTH, MAP_HEIGHT);

        self.tcod = Some(Tcod {
            root,
            map,
            fov,
            panel,
        });

        tcod::system::set_fps(LIMIT_FPS);
    }
}
