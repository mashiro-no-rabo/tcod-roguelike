use tcod::colors;
use tcod::console::*;
use tcod::Color;

use crate::map::Map;
use crate::mut_two;
use crate::Position;
use crate::PLAYER;

#[derive(Debug)]
pub struct Object {
    x: i32,
    y: i32,
    pub rep: char,
    pub color: Color,
    pub name: String,
    pub blocks: bool,
    pub alive: bool,
    pub hp: Option<HitPoints>,
    pub melee: Option<Melee>,
    pub ai: Option<Ai>,
    pub item: Option<Item>,
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
            hp: None,
            melee: None,
            ai: None,
            item: None,
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

    pub fn attack_move(idx: usize, dx: i32, dy: i32, map: &Map, objects: &mut [Object]) {
        if let Some(obj) = objects.get(idx) {
            let (x, y) = obj.pos();
            let (tx, ty) = (x + dx, y + dy);

            let target_idx = objects
                .iter()
                .position(|obj| obj.pos() == (tx, ty) && obj.melee.is_some());

            match target_idx {
                Some(tidx) => {
                    let (player, target) = mut_two(PLAYER, tidx, objects);
                    player.melee_attack(target);
                }
                None => Self::try_move(idx, dx, dy, map, objects),
            }
        }
    }

    pub fn move_towards(
        idx: usize,
        target_x: i32,
        target_y: i32,
        map: &Map,
        objects: &mut [Object],
    ) {
        // vector from this object to the target, and distance
        let dx = target_x - objects[idx].x;
        let dy = target_y - objects[idx].y;
        let distance = ((dx.pow(2) + dy.pow(2)) as f32).sqrt();

        // normalize it to length 1 (preserving direction), then round it and
        // convert to integer so the movement is restricted to the map grid
        let dx = (dx as f32 / distance).round() as i32;
        let dy = (dy as f32 / distance).round() as i32;
        Self::try_move(idx, dx, dy, map, objects);
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

    pub fn pos(&self) -> Position {
        (self.x, self.y)
    }

    pub fn set_pos(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    /// return the distance to another object
    pub fn distance_to(&self, other: &Object) -> f32 {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        ((dx.pow(2) + dy.pow(2)) as f32).sqrt()
    }

    pub fn take_damage(&mut self, damage: i32) {
        if let Some(hp) = self.hp.as_mut() {
            if damage > 0 {
                hp.current -= damage;
            }
        }

        if let Some(hp) = self.hp {
            if !hp.alive() {
                self.alive = false;
                hp.on_death.callback(self);
            }
        }
    }

    pub fn melee_attack(&mut self, target: &mut Object) {
        // a simple formula for attack damage
        let damage = self.melee.map_or(0, |m| m.attack) - target.melee.map_or(0, |m| m.defense);
        if damage > 0 {
            // make the target take some damage
            println!(
                "{} attacks {} for {} hit points.",
                self.name, target.name, damage
            );
            target.take_damage(damage);
        } else {
            println!(
                "{} attacks {} but it has no effect!",
                self.name, target.name
            );
        }
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

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HitPoints {
    pub max: i32,
    pub current: i32,
    pub on_death: DeathCallback,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DeathCallback {
    Player,
    Monster,
}

impl DeathCallback {
    pub fn callback(self, object: &mut Object) {
        use DeathCallback::*;

        let callback: fn(&mut Object) = match self {
            Player => player_death,
            Monster => monster_death,
        };

        callback(object);
    }
}

fn player_death(player: &mut Object) {
    // the game ended!
    println!("You died!");

    // for added effect, transform the player into a corpse!
    player.rep = '%';
    player.color = colors::DARK_RED;
}

fn monster_death(monster: &mut Object) {
    // transform it into a nasty corpse! it doesn't block, can't be
    // attacked and doesn't move
    println!("{} is dead!", monster.name);
    monster.rep = '%';
    monster.color = colors::DARK_RED;
    monster.blocks = false;
    monster.hp = None;
    monster.melee = None;
    monster.ai = None;
    monster.name = format!("remains of {}", monster.name);
}

impl HitPoints {
    pub fn alive(&self) -> bool {
        self.current > 0
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Melee {
    pub attack: i32,
    pub defense: i32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ai;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Item {
    Heal,
}
