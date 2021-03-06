use rand::Rng;
use std::cmp;
use tcod::colors;

use crate::object::{Ai, HitPoints, Melee, Object};
use crate::Position;

#[derive(Clone, Copy, Debug)]
pub struct Tile {
    pub blocked: bool,
    pub block_sight: bool,
    pub explored: bool,
}

impl Tile {
    pub fn empty() -> Self {
        Tile {
            blocked: false,
            block_sight: false,
            explored: false,
        }
    }

    pub fn wall() -> Self {
        Tile {
            blocked: true,
            block_sight: true,
            explored: false,
        }
    }
}

pub type Map = Vec<Vec<Tile>>;

const ROOM_MAX_SIZE: i32 = 10;
const ROOM_MIN_SIZE: i32 = 6;
const MAX_ROOMS: i32 = 30;

pub fn make_map(height: usize, width: usize, objects: &mut Vec<Object>) -> (Map, Position) {
    let mut map = vec![vec![Tile::wall(); height]; width];

    let mut rooms = vec![];

    let mut starting_position = (0, 0);

    for _ in 0..MAX_ROOMS {
        // random width and height
        let w = rand::thread_rng().gen_range(ROOM_MIN_SIZE, ROOM_MAX_SIZE + 1);
        let h = rand::thread_rng().gen_range(ROOM_MIN_SIZE, ROOM_MAX_SIZE + 1);
        // random position without going out of the boundaries of the map
        let x = rand::thread_rng().gen_range(0, width as i32 - w);
        let y = rand::thread_rng().gen_range(0, height as i32 - h);

        let new_room = Rect::new(x, y, w, h);

        // run through the other rooms and see if they intersect with this one
        let failed = rooms
            .iter()
            .any(|other_room| new_room.intersects_with(other_room));

        if !failed {
            // this means there are no intersections, so this room is valid

            // "paint" it to the map's tiles
            create_room(new_room, &mut map);

            // center coordinates of the new room, will be useful later
            let (new_x, new_y) = new_room.center();

            if rooms.is_empty() {
                // this is the first room, where the player starts at
                starting_position = (new_x, new_y);
            } else {
                // all rooms after the first:
                // connect it to the previous room with a tunnel

                // center coordinates of the previous room
                let (prev_x, prev_y) = rooms[rooms.len() - 1].center();

                // place monsters only in non-first rooms
                place_objects(new_room, objects);

                // draw a coin (random bool value -- either true or false)
                if rand::random() {
                    // first move horizontally, then vertically
                    create_h_tunnel(prev_x, new_x, prev_y, &mut map);
                    create_v_tunnel(prev_y, new_y, new_x, &mut map);
                } else {
                    // first move vertically, then horizontally
                    create_v_tunnel(prev_y, new_y, prev_x, &mut map);
                    create_h_tunnel(prev_x, new_x, new_y, &mut map);
                }
            }

            rooms.push(new_room);
        }
    }

    (map, starting_position)
}

#[derive(Clone, Copy, Debug)]
pub struct Rect {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

impl Rect {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        Rect {
            x1: x,
            y1: y,
            x2: x + w,
            y2: y + h,
        }
    }

    pub fn center(&self) -> Position {
        let center_x = (self.x1 + self.x2) / 2;
        let center_y = (self.y1 + self.y2) / 2;
        (center_x, center_y)
    }

    pub fn intersects_with(&self, other: &Rect) -> bool {
        // returns true if this rectangle intersects with another one
        (self.x1 <= other.x2)
            && (self.x2 >= other.x1)
            && (self.y1 <= other.y2)
            && (self.y2 >= other.y1)
    }

    pub fn rand_inside(&self) -> Position {
        let x = rand::thread_rng().gen_range(self.x1 + 1, self.x2);
        let y = rand::thread_rng().gen_range(self.y1 + 1, self.y2);

        (x, y)
    }
}

fn create_room(room: Rect, map: &mut Map) {
    for x in (room.x1 + 1)..room.x2 {
        for y in (room.y1 + 1)..room.y2 {
            map[x as usize][y as usize] = Tile::empty();
        }
    }
}

fn create_h_tunnel(x1: i32, x2: i32, y: i32, map: &mut Map) {
    for x in cmp::min(x1, x2)..=cmp::max(x1, x2) {
        map[x as usize][y as usize] = Tile::empty();
    }
}

fn create_v_tunnel(y1: i32, y2: i32, x: i32, map: &mut Map) {
    for y in cmp::min(y1, y2)..=cmp::max(y1, y2) {
        map[x as usize][y as usize] = Tile::empty();
    }
}

const MAX_ROOM_MONSTERS: i32 = 3;
const MAX_ROOM_ITEMS: i32 = 2;

fn place_objects(room: Rect, objects: &mut Vec<Object>) {
    use crate::object::DeathCallback;
    use crate::object::Item;

    // choose random number of monsters
    let num_monsters = rand::thread_rng().gen_range(0, MAX_ROOM_MONSTERS + 1);

    let mut monsters_pos = vec![];

    for _ in 0..num_monsters {
        // choose random spot for this monster
        let (x, y) = room.rand_inside();

        monsters_pos.push((x, y));

        let mut monster = if rand::random::<f32>() < 0.8 {
            // 80% chance of getting an orc
            // create an orc
            let mut orc = Object::new(x, y, 'o', colors::DESATURATED_GREEN, "orc", true);
            orc.hp = Some(HitPoints {
                max: 10,
                current: 10,
                on_death: DeathCallback::Monster,
            });
            orc.melee = Some(Melee {
                attack: 3,
                defense: 0,
            });
            orc.ai = Some(Ai);
            orc
        } else {
            let mut troll = Object::new(x, y, 'T', colors::DARKER_GREEN, "troll", true);
            troll.hp = Some(HitPoints {
                max: 16,
                current: 16,
                on_death: DeathCallback::Monster,
            });
            troll.melee = Some(Melee {
                attack: 4,
                defense: 1,
            });
            troll.ai = Some(Ai);
            troll
        };

        monster.alive = true;

        objects.push(monster);
    }

    // choose random number of items
    let num_items = rand::thread_rng().gen_range(0, MAX_ROOM_ITEMS + 1);

    for _ in 0..num_items {
        // choose random spot for this item
        let x = rand::thread_rng().gen_range(room.x1 + 1, room.x2);
        let y = rand::thread_rng().gen_range(room.y1 + 1, room.y2);

        // only place it if the tile is not blocked
        if !monsters_pos.contains(&(x, y)) {
            // create a healing potion
            let mut object = Object::new(x, y, '!', colors::VIOLET, "healing potion", false);
            object.item = Some(Item::Heal);
            objects.push(object);
        }
    }
}
