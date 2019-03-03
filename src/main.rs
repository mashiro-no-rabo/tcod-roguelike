use specs::{DispatcherBuilder, World};

mod consts;
mod systems;

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

    let mut dispatcher = DispatcherBuilder::new()
        .with(systems::DebugPrint, "debug_print", &[])
        .with_thread_local(systems::TcodIntegration::default())
        .build();

    dispatcher.setup(&mut world.res);

    loop {
        dispatcher.dispatch(&mut world.res);
        world.maintain();
    }
}
