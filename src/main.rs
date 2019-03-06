use specs::{Builder as _, DispatcherBuilder, World};

// TODO: review all tcod usage here
use tcod::colors;

mod components;
mod consts;
mod resources;
mod systems;

use resources::PlayerExit;

fn main() {
    let mut world = World::new();

    world.add_resource(PlayerExit(false));

    let mut dispatcher = DispatcherBuilder::new()
        .with(systems::DebugPrint, "debug_print", &[])
        .with(systems::Movement, "movement", &[])
        .with(systems::Exit, "exit_game", &[])
        .with_thread_local(systems::TcodIntegration::default())
        .with_thread_local(systems::Input)
        .build();

    dispatcher.setup(&mut world.res);

    create_player(&mut world);

    loop {
        dispatcher.dispatch(&mut world.res);
        world.maintain();

        if world.read_resource::<PlayerExit>().0 {
            break;
        }
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
        .with(Player {})
        .build();
}
