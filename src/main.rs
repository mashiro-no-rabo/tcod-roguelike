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
        .with(systems::Input, "input_handler", &[])
        .with(systems::DebugPrint, "debug_print", &["input_handler"])
        .with_thread_local(systems::TcodRender::default())
        .build();

    dispatcher.setup(&mut world.res);

    dispatcher.dispatch(&mut world.res);
    world.maintain();
}
