use specs::{Read, System};

use crate::resources::InputMapping;

pub struct DebugPrint;

impl<'a> System<'a> for DebugPrint {
    type SystemData = Read<'a, InputMapping>;

    fn run(&mut self, im: Self::SystemData) {
        let key = &im.key;

        match key {
            None => {}
            val => println!("{:?}", val),
        }
    }
}
