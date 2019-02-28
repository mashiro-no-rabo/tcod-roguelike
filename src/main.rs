use tcod::colors;
use tcod::console::*;
use tcod::map::{FovAlgorithm, Map as FovMap};
use tcod::Color;

mod map;
use map::{make_map, Map};

mod object;
use object::{HitPoints, Melee, Object};

pub type Position = (i32, i32);

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const LIMIT_FPS: i32 = 20;

const MAP_WIDTH: i32 = 80;
const MAP_HEIGHT: i32 = 45;

// sizes and coordinates relevant for the GUI
const BAR_WIDTH: i32 = 20;
const PANEL_HEIGHT: i32 = 7;
const PANEL_Y: i32 = SCREEN_HEIGHT - PANEL_HEIGHT;
const MSG_X: i32 = BAR_WIDTH + 2;
const MSG_WIDTH: i32 = SCREEN_WIDTH - BAR_WIDTH - 2;
const MSG_HEIGHT: usize = PANEL_HEIGHT as usize - 1;

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

// player will always be the first object
const PLAYER: usize = 0;

type Messages = Vec<(String, Color)>;

fn main() {
    let mut root = Root::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("MechRogue")
        .init();
    tcod::system::set_fps(LIMIT_FPS);
    let mut con = Offscreen::new(MAP_WIDTH, MAP_HEIGHT);
    let mut panel = Offscreen::new(SCREEN_WIDTH, PANEL_HEIGHT);

    let mut messages: Messages = vec![];
    add_message(&mut messages, "Welcome to MechRogfue!", colors::RED);

    let mut objects: Vec<Object> = Vec::new();
    let (mut map, start_position) = make_map(MAP_HEIGHT as usize, MAP_WIDTH as usize, &mut objects);
    objects.insert(0, create_player(start_position));

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

    // enter Game Loop
    while !root.window_closed() {
        let new_pos = objects[PLAYER].pos();

        let fov_recompute = previous_player_position != new_pos;
        render_all(
            &mut root,
            &mut con,
            &mut panel,
            &messages,
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

        previous_player_position = new_pos;

        let mut exit = false;

        // handle keys and exit game if needed
        loop {
            match handle_keys(&mut root, &mut objects, &map) {
                PlayerAction::TookTurn => break,
                PlayerAction::NoTurn => {}
                PlayerAction::Exit => {
                    exit = true;
                    break;
                }
            }
        }

        if exit {
            break;
        }

        // AI behaviours
        if objects[PLAYER].alive {
            for idx in 0..objects.len() {
                if objects[idx].ai.is_some() {
                    ai_take_turn(idx, &map, &mut objects, &fov_map);
                }
            }
        }
    }
}

fn ai_take_turn(idx: usize, map: &Map, objects: &mut [Object], fov_map: &FovMap) {
    // a basic monster takes its turn. If you can see it, it can see you
    let (monster_x, monster_y) = objects[idx].pos();
    if fov_map.is_in_fov(monster_x, monster_y) {
        if objects[idx].distance_to(&objects[PLAYER]) >= 2.0 {
            // move towards player if far away
            let (player_x, player_y) = objects[PLAYER].pos();
            Object::move_towards(idx, player_x, player_y, map, objects);
        } else if objects[PLAYER].hp.map_or(false, |p| p.alive()) {
            // close enough, attack! (if the player is still alive)
            let (monster, player) = mut_two(idx, PLAYER, objects);
            monster.melee_attack(player);
        }
    }
}

fn create_player((x, y): Position) -> Object {
    use object::DeathCallback;
    let mut player = Object::new(x, y, '@', colors::CYAN, "aquarhead", true);
    player.alive = true;
    player.hp = Some(HitPoints {
        max: 30,
        current: 30,
        on_death: DeathCallback::Player,
    });
    player.melee = Some(Melee {
        attack: 5,
        defense: 2,
    });

    player
}

/// Mutably borrow two *separate* elements from the given slice.
/// Panics when the indexes are equal or out of bounds.
pub fn mut_two<T>(first_index: usize, second_index: usize, items: &mut [T]) -> (&mut T, &mut T) {
    use std::cmp;

    assert!(first_index != second_index);
    let split_at_index = cmp::max(first_index, second_index);
    let (first_slice, second_slice) = items.split_at_mut(split_at_index);
    if first_index < second_index {
        (&mut first_slice[first_index], &mut second_slice[0])
    } else {
        (&mut second_slice[0], &mut first_slice[second_index])
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum PlayerAction {
    TookTurn,
    NoTurn,
    Exit,
}

fn handle_keys(root: &mut Root, objects: &mut Vec<Object>, map: &Map) -> PlayerAction {
    use tcod::input::Key;
    use tcod::input::KeyCode::*;

    use PlayerAction::*;

    let player_alive = objects[PLAYER].alive;
    let key = root.wait_for_keypress(true);

    match (key, player_alive) {
        (Key { code: Escape, .. }, _) => Exit, // exit game

        // movement keys
        (Key { code: Up, .. }, true) => {
            Object::attack_move(PLAYER, 0, -1, map, objects);
            TookTurn
        }
        (Key { code: Down, .. }, true) => {
            Object::attack_move(PLAYER, 0, 1, map, objects);
            TookTurn
        }
        (Key { code: Left, .. }, true) => {
            Object::attack_move(PLAYER, -1, 0, map, objects);
            TookTurn
        }
        (Key { code: Right, .. }, true) => {
            Object::attack_move(PLAYER, 1, 0, map, objects);
            TookTurn
        }

        _ => NoTurn,
    }
}

const FOV_ALGO: FovAlgorithm = FovAlgorithm::Basic;
const FOV_LIGHT_WALLS: bool = true;
const TORCH_RADIUS: i32 = 4;

fn render_all(
    root: &mut Root,
    con: &mut Offscreen,
    panel: &mut Offscreen,
    messages: &Messages,
    objects: &[Object],
    map: &mut Map,
    fov_map: &mut FovMap,
    fov_recompute: bool,
) {
    if fov_recompute {
        // recompute FOV if needed (the player moved or something)
        let player = &objects[PLAYER];
        let (px, py) = player.pos();
        fov_map.compute_fov(px, py, TORCH_RADIUS, FOV_LIGHT_WALLS, FOV_ALGO);

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
    let mut to_draw: Vec<_> = objects
        .iter()
        .filter(|obj| {
            let (x, y) = obj.pos();
            fov_map.is_in_fov(x, y)
        })
        .collect();
    // sort so that non-blocking objects come first
    to_draw.sort_by(|o1, o2| o1.blocks.cmp(&o2.blocks));
    for object in &to_draw {
        object.draw(con);
    }

    // blit the contents of "con" to the root console and present it
    blit(con, (0, 0), (MAP_WIDTH, MAP_HEIGHT), root, (0, 0), 1.0, 1.0);

    // prepare to render the GUI panel
    panel.set_default_background(colors::BLACK);
    panel.clear();

    // show the player's stats
    let hp = objects[PLAYER].hp.map_or(0, |x| x.current);
    let max_hp = objects[PLAYER].hp.map_or(0, |x| x.max);
    render_bar(
        panel,
        1,
        1,
        BAR_WIDTH,
        "HP",
        hp,
        max_hp,
        colors::LIGHT_RED,
        colors::DARKER_RED,
    );

    // print the game messages, one line at a time
    let mut y = MSG_HEIGHT as i32;
    for &(ref msg, color) in messages.iter().rev() {
        let msg_height = panel.get_height_rect(MSG_X, y, MSG_WIDTH, 0, msg);
        y -= msg_height;
        if y < 0 {
            break;
        }
        panel.set_default_foreground(color);
        panel.print_rect(MSG_X, y, MSG_WIDTH, 0, msg);
    }

    // blit the contents of `panel` to the root console
    blit(
        panel,
        (0, 0),
        (SCREEN_WIDTH, PANEL_HEIGHT),
        root,
        (0, PANEL_Y),
        1.0,
        1.0,
    );
}

fn render_bar(
    panel: &mut Offscreen,
    x: i32,
    y: i32,
    total_width: i32,
    name: &str,
    value: i32,
    maximum: i32,
    bar_color: Color,
    back_color: Color,
) {
    // render a bar (HP, experience, etc). First calculate the width of the bar
    let bar_width = (value as f32 / maximum as f32 * total_width as f32) as i32;

    // render the background first
    panel.set_default_background(back_color);
    panel.rect(x, y, total_width, 1, false, BackgroundFlag::Screen);

    // now render the bar on top
    panel.set_default_background(bar_color);
    if bar_width > 0 {
        panel.rect(x, y, bar_width, 1, false, BackgroundFlag::Screen);
    }

    // finally, some centered text with the values
    panel.set_default_foreground(colors::WHITE);
    panel.print_ex(
        x + total_width / 2,
        y,
        BackgroundFlag::None,
        TextAlignment::Center,
        &format!("{}: {}/{}", name, value, maximum),
    );
}

fn add_message<T: Into<String>>(messages: &mut Messages, message: T, color: Color) {
    // if the buffer is full, remove the first message to make room for the new one
    if messages.len() == MSG_HEIGHT {
        messages.remove(0);
    }
    // add the new line as a tuple, with the text and the color
    messages.push((message.into(), color));
}
