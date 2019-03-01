use specs::{
    Builder, Component, DispatcherBuilder, ReadStorage, System, VecStorage, World, WriteStorage,
};

use tcod::colors;
use tcod::console::*;
use tcod::input::{self, Event, Key, Mouse};
use tcod::map::{FovAlgorithm, Map as FovMap};
use tcod::Color;

mod systems;

pub struct Tcod {
    root: Root,
    map: Offscreen,
    panel: Offscreen,
    fov: FovMap,
}

impl Tcod {
    fn new() -> Self {
        let root = Root::initializer()
            .font("arial10x10.png", FontLayout::Tcod)
            .font_type(FontType::Greyscale)
            .size(SCREEN_WIDTH, SCREEN_HEIGHT)
            .title("MechRogue")
            .init();
        tcod::system::set_fps(LIMIT_FPS);
        let map = Offscreen::new(MAP_WIDTH, MAP_HEIGHT);
        let panel = Offscreen::new(SCREEN_WIDTH, PANEL_HEIGHT);
        let fov = FovMap::new(MAP_WIDTH, MAP_HEIGHT);

        Tcod {
            root,
            map,
            panel,
            fov,
        }
    }
}

pub type Position = (i32, i32);

#[derive(Debug, Default)]
pub struct InputMapping {
    key: Option<VirtualKey>,
    mouse: Option<Position>,
}

#[derive(Debug)]
pub enum VirtualKey {
    NoAction,
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    Exit,
    PickItem,
    DropItem,
}

fn main() {
    let mut world = World::new();
    // world.add_resource(Tcod);

    let mut dispatcher = DispatcherBuilder::new()
        .with(systems::Input, "input_handler", &[])
        .with(systems::DebugPrint, "debug_print", &["input_handler"])
        .build();

    dispatcher.setup(&mut world.res);

    let mut tcod = Tcod::new();
    // enter Game Loop
    while !tcod.root.window_closed() {
        dispatcher.dispatch(&mut world.res);
        world.maintain();

        tcod.root.flush();
    }
}

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const LIMIT_FPS: i32 = 20;

const MAP_WIDTH: i32 = 80;
const MAP_HEIGHT: i32 = 45;

// sizes and coordinates relevant for the GUI
const BAR_WIDTH: i32 = 20;
const PANEL_HEIGHT: i32 = 7;
const PANEL_Y: i32 = SCREEN_HEIGHT - PANEL_HEIGHT;
const MSG_X: i32 = BAR_WIDTH + 2;
const MSG_WIDTH: i32 = SCREEN_WIDTH - BAR_WIDTH - 2;
const MSG_HEIGHT: usize = PANEL_HEIGHT as usize - 1;

const COLOR_DARK_WALL: Color = Color { r: 0, g: 0, b: 100 };
const COLOR_LIGHT_WALL: Color = Color {
    r: 130,
    g: 110,
    b: 50,
};
const COLOR_DARK_GROUND: Color = Color {
    r: 50,
    g: 50,
    b: 150,
};
const COLOR_LIGHT_GROUND: Color = Color {
    r: 200,
    g: 180,
    b: 50,
};
