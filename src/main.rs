use specs::{DispatcherBuilder, World};

mod consts;
mod systems;

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
