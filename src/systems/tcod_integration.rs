use specs::{join::Join, ReadStorage, Resources, System, Write};

use tcod::colors;
use tcod::console::*;
use tcod::input::{self, Event, Key, Mouse};
use tcod::map::{FovAlgorithm, Map as FovMap};
use tcod::Color;

use crate::components::*;
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
    type SystemData = (
        ReadStorage<'a, Position>,
        ReadStorage<'a, MapRenderable>,
        Write<'a, InputMapping>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (pos, mapr, mut im) = data;

        // fetch input, this is done here to avoid being parallel executed by Specs
        Self::map_input(&mut im);

        self.tcod.as_mut().map(|t| {
            t.map.set_default_foreground(colors::WHITE);

            for (pos, mapr) in (&pos, &mapr).join() {
                t.map.put_char(pos.x, pos.y, mapr.rep, BackgroundFlag::None);
            }

            blit(
                &mut t.map,
                (0, 0),
                (SCREEN_WIDTH, SCREEN_HEIGHT),
                &mut t.root,
                (0, 0),
                1.0,
                1.0,
            );

            t.root.flush();

            // cleanup
            for (pos, mapr) in (&pos, &mapr).join() {
                t.map.put_char(pos.x, pos.y, ' ', BackgroundFlag::None);
            }
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

impl TcodIntegration {
    fn map_input(im: &mut InputMapping) {
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
