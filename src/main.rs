use specs::{Builder as _, DispatcherBuilder, World};

// TODO: review all tcod usage here
use tcod::colors;

mod components;
mod consts;
mod systems;

fn main() {
    let mut world = World::new();

    let mut dispatcher = DispatcherBuilder::new()
        .with(systems::DebugPrint, "debug_print", &[])
        .with_thread_local(systems::TcodIntegration::default())
        .build();

    dispatcher.setup(&mut world.res);

    create_player(&mut world);

    loop {
        dispatcher.dispatch(&mut world.res);
        world.maintain();
    }
}

fn create_player(world: &mut World) {
    use components::*;
    use consts::*;

    world
        .create_entity()
        .with(Position {
            x: SCREEN_WIDTH / 2,
            y: SCREEN_HEIGHT / 2,
        })
        .with(MapRenderable {
            rep: '@',
            color: colors::CYAN,
        })
        .build();
}

#[derive(Debug, Default)]
pub struct InputMapping {
    key: Option<VirtualKey>,
    mouse: Option<(i32, i32)>,
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
