use crate::game::game::TILE_SIZE;

#[derive(Copy, Clone)]
pub struct Tile {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub tile_type: TileType,
}

impl Tile {
    pub fn new(x: f32, y: f32, tile_type: TileType) -> Self {
        Self {
            x,
            y,
            w: TILE_SIZE,
            h: TILE_SIZE,
            tile_type,
        }
    }

    pub fn sprite_name(&self) -> &'static str {
        self.tile_type.sprite_name()
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Grass,
    Water,
    Stone,
}

impl TileType {
    pub fn sprite_name(&self) -> &'static str {
        match self {
            TileType::Grass => "grass",
            TileType::Water => "water",
            TileType::Stone => "stone",
        }
    }
}
