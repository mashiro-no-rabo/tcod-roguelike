use specs::{storage::BTreeStorage, Component, VecStorage};

// TODO: review all tcod usage here
use tcod::Color;

#[derive(Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Component for Position {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
pub struct MapRenderable {
    pub rep: char,
    pub color: Color,
}

impl Component for MapRenderable {
    type Storage = BTreeStorage<Self>;
}
