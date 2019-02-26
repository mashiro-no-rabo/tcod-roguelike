use tcod::console::*;
use tcod::Color;

use crate::map::Map;

#[derive(Debug)]
pub struct Object {
    x: i32,
    y: i32,
    pub rep: char,
    pub color: Color,
}

impl Object {
    pub fn new(x: i32, y: i32, rep: char, color: Color) -> Self {
        Object { x, y, rep, color }
    }

    pub fn move_by(&mut self, dx: i32, dy: i32, map: &Map) {
        // can't move onto blocked tile
        if !map[(self.x + dx) as usize][(self.y + dy) as usize].blocked {
            self.x += dx;
            self.y += dy;
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
