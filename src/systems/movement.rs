use specs::{join::Join, Read, ReadStorage, System, WriteStorage};

use crate::components::{Player, Position};
use crate::{InputMapping, VirtualKey};

#[derive(Default)]
pub struct Movement;

impl<'a> System<'a> for Movement {
    type SystemData = (
        WriteStorage<'a, Position>,
        ReadStorage<'a, Player>,
        Read<'a, InputMapping>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut pos, player, im) = data;

        for (pos, _) in (&mut pos, &player).join() {
            match &im.key {
                Some(VirtualKey::MoveUp) => pos.y -= 1,
                Some(VirtualKey::MoveDown) => pos.y += 1,
                Some(VirtualKey::MoveLeft) => pos.x -= 1,
                Some(VirtualKey::MoveRight) => pos.x += 1,
                _ => {}
            }
        }
    }
}
