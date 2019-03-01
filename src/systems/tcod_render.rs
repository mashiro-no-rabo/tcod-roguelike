use specs::{prelude::Resources, Read, System};

use tcod::colors;
use tcod::console::*;
use tcod::input::{self, Event, Key, Mouse};
use tcod::map::{FovAlgorithm, Map as FovMap};
use tcod::Color;

use crate::consts::*;
use crate::InputMapping;

#[derive(Default)]
pub struct TcodRender {
    tcod: Option<Tcod>,
}

struct Tcod {
    root: Root,
    map: Offscreen,
    panel: Offscreen,
    fov: FovMap,
}

impl<'a> System<'a> for TcodRender {
    type SystemData = Read<'a, InputMapping>;

    fn run(&mut self, _im: Self::SystemData) {
        self.tcod.as_mut().map(|t| t.root.flush());
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
