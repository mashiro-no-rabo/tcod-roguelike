extern crate tcod;

use tcod::colors;
use tcod::console::*;
use tcod::map::{FovAlgorithm, Map as FovMap};
use tcod::Color;

mod map;
use map::{make_map, Map};

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const LIMIT_FPS: i32 = 20;

const MAP_WIDTH: i32 = 80;
const MAP_HEIGHT: i32 = 45;

const COLOR_DARK_WALL: Color = Color { r: 0, g: 0, b: 100 };
const COLOR_LIGHT_WALL: Color = Color {
    r: 130,
    g: 110,
    b: 50,
};
const COLOR_DARK_GROUND: Color = Color {
    r: 50,
    g: 50,
    b: 150,
};
const COLOR_LIGHT_GROUND: Color = Color {
    r: 200,
    g: 180,
    b: 50,
};

#[derive(Debug)]
struct Object {
    x: i32,
    y: i32,
    rep: char,
    color: Color,
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
}

fn main() {
    let mut root = Root::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("MechRogue")
        .init();

    tcod::system::set_fps(LIMIT_FPS);

    let mut con = Offscreen::new(MAP_WIDTH, MAP_HEIGHT);

    let (mut map, (x, y)) = make_map(MAP_HEIGHT as usize, MAP_WIDTH as usize);

    let player = Object::new(x, y, '@', colors::CYAN);
    let mut objects = [player];

    let mut fov_map = FovMap::new(MAP_WIDTH, MAP_HEIGHT);
    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            fov_map.set(
                x,
                y,
                !map[x as usize][y as usize].block_sight,
                !map[x as usize][y as usize].blocked,
            );
        }
    }

    let mut previous_player_position = (-1, -1);

    while !root.window_closed() {
        let fov_recompute = previous_player_position != (objects[0].x, objects[0].y);
        render_all(
            &mut root,
            &mut con,
            &objects,
            &mut map,
            &mut fov_map,
            fov_recompute,
        );
        root.flush();

        // erase all objects at their old locations, before they move
        for object in &objects {
            object.clear(&mut con)
        }

        // handle keys and exit game if needed
        let player = &mut objects[0];
        previous_player_position = (player.x, player.y);
        let exit = handle_keys(&mut root, player, &map);
        if exit {
            break;
        }
    }
}

fn handle_keys(root: &mut Root, player: &mut Object, map: &Map) -> bool {
    use tcod::input::Key;
    use tcod::input::KeyCode::*;

    let key = root.wait_for_keypress(true);
    match key {
        Key { code: Escape, .. } => return true, // exit game

        // movement keys
        Key { code: Up, .. } => player.move_by(0, -1, map),
        Key { code: Down, .. } => player.move_by(0, 1, map),
        Key { code: Left, .. } => player.move_by(-1, 0, map),
        Key { code: Right, .. } => player.move_by(1, 0, map),

        _ => {}
    }

    false
}

const FOV_ALGO: FovAlgorithm = FovAlgorithm::Basic;
const FOV_LIGHT_WALLS: bool = true;
const TORCH_RADIUS: i32 = 4;

fn render_all(
    root: &mut Root,
    con: &mut Offscreen,
    objects: &[Object],
    map: &mut Map,
    fov_map: &mut FovMap,
    fov_recompute: bool,
) {
    if fov_recompute {
        // recompute FOV if needed (the player moved or something)
        let player = &objects[0];
        fov_map.compute_fov(player.x, player.y, TORCH_RADIUS, FOV_LIGHT_WALLS, FOV_ALGO);

        // go through all tiles, and set their background color
        for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTH {
                let visible = fov_map.is_in_fov(x, y);

                let wall = map[x as usize][y as usize].block_sight;

                let color = match (visible, wall) {
                    // outside of field of view:
                    (false, true) => COLOR_DARK_WALL,
                    (false, false) => COLOR_DARK_GROUND,
                    // inside fov:
                    (true, true) => COLOR_LIGHT_WALL,
                    (true, false) => COLOR_LIGHT_GROUND,
                };

                let explored = &mut map[x as usize][y as usize].explored;
                if visible {
                    // since it's visible, explore it
                    *explored = true;
                }

                if *explored {
                    // show explored tiles only (any visible tile is explored already)
                    con.set_char_background(x, y, color, BackgroundFlag::Set);
                }
            }
        }
    }

    // render objects
    for object in objects {
        if fov_map.is_in_fov(object.x, object.y) {
            object.draw(con);
        }
    }

    // blit the contents of "con" to the root console and present it
    blit(con, (0, 0), (MAP_WIDTH, MAP_HEIGHT), root, (0, 0), 1.0, 1.0);
}
