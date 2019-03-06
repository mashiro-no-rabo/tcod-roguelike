use specs::{Read, System, Write};

use crate::resources::{InputMapping, PlayerExit, VirtualKey};

#[derive(Default)]
pub struct Exit;

impl<'a> System<'a> for Exit {
    type SystemData = (Read<'a, InputMapping>, Write<'a, PlayerExit>);

    fn run(&mut self, data: Self::SystemData) {
        let (im, mut pe) = data;

        if let Some(VirtualKey::Exit) = &im.key {
            pe.0 = true;
        }
    }
}
