#[derive(Clone, Copy, Debug)]
pub struct Tile {
    pub blocked: bool,
    pub block_sight: bool,
}

impl Tile {
    pub fn empty() -> Self {
        Tile {
            blocked: false,
            block_sight: false,
        }
    }

    pub fn wall() -> Self {
        Tile {
            blocked: true,
            block_sight: true,
        }
    }
}

pub type Map = Vec<Vec<Tile>>;

pub fn make_map(height: usize, width: usize) -> Map {
    let mut map = vec![vec![Tile::empty(); height]; width];
    map[30][22] = Tile::wall();
    map[50][22] = Tile::wall();

    map
}
