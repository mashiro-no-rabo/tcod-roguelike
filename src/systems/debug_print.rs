use specs::{System, Read};

use crate::InputMapping;

pub struct DebugPrint;

impl<'a> System<'a> for DebugPrint {
    type SystemData = Read<'a, InputMapping>;

    fn run(&mut self, im: Self::SystemData) {
        println!("{:?}", *im);
    }
}
