use tcod::console::*;
use tcod::Color;

use crate::map::Map;

#[derive(Debug)]
pub struct Object {
    x: i32,
    y: i32,
    pub rep: char,
    pub color: Color,
    pub name: String,
    pub blocks: bool,
    pub alive: bool,
}

impl Object {
    pub fn new(x: i32, y: i32, rep: char, color: Color, name: &str, blocks: bool) -> Self {
        Object {
            x,
            y,
            rep,
            color,
            name: name.into(),
            blocks,
            alive: false,
        }
    }

    pub fn try_move(idx: usize, dx: i32, dy: i32, map: &Map, objects: &mut [Object]) {
        if let Some(obj) = objects.get(idx) {
            let (x, y) = obj.pos();
            if !is_blocked(x + dx, y + dy, map, objects) {
                objects[idx].set_pos(x + dx, y + dy);
            }
        }
    }

    /// set the color and then draw the character that represents this object at its position
    pub fn draw(&self, con: &mut Console) {
        con.set_default_foreground(self.color);
        con.put_char(self.x, self.y, self.rep, BackgroundFlag::None);
    }

    /// Erase the character that represents this object
    pub fn clear(&self, con: &mut Console) {
        con.put_char(self.x, self.y, ' ', BackgroundFlag::None);
    }

    pub fn pos(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn set_pos(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }
}

fn is_blocked(x: i32, y: i32, map: &Map, objects: &[Object]) -> bool {
    // first test the map tile
    if map[x as usize][y as usize].blocked {
        return true;
    }
    // now check for any blocking objects
    objects
        .iter()
        .any(|object| object.blocks && object.pos() == (x, y))
}
