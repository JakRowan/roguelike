use tcod::{colors::Color, map::FovAlgorithm};

mod geometry;
mod map;
mod tile;

pub const MAP_WIDTH: usize = 80;
pub const MAP_HEIGHT: usize = 45;

pub const COLOR_DARK_WALL: Color = Color { r: 0, g: 0, b: 100 };
pub const COLOR_LIGHT_WALL: Color = Color {
    r: 130,
    g: 110,
    b: 50,
};
pub const COLOR_DARK_GROUND: Color = Color {
    r: 50,
    g: 50,
    b: 150,
};
pub const COLOR_LIGHT_GROUND: Color = Color {
    r: 200,
    g: 180,
    b: 50,
};

pub const FOV_ALGO: FovAlgorithm = FovAlgorithm::Basic;
pub const FOV_LIGTH_WALLS: bool = true;
pub const TORCH_RADIUS: usize = 10;

const ROOM_MAX_SIZE: usize = 10;
const ROOM_MIN_SIZE: usize = 6;
const MAX_ROOMS: usize = 30;

pub use geometry::{Point, Rect};
pub use map::Map;
pub use tile::Tile;
