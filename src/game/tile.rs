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
    Body,
    Obstacle,
    Grass,
    Tree,
    Stone,
}

impl TileType {
    pub fn sprite_name(&self) -> &'static str {
        match self {
            TileType::Body => "body_background",
            TileType::Obstacle => "body_obstacle",
            TileType::Grass => "grass",
            TileType::Tree => "tree",
            TileType::Stone => "stone",
        }
    }

    pub fn from_tiled_id(id: u32) -> Self {
        match id {
            0 => TileType::Body,
            1 => TileType::Body,
            2 => TileType::Body,
            3 => TileType::Obstacle,
            _ => {
                log::warn!("Unknown tile id: {}", id);
                TileType::Body
            }
        }
    }

    pub fn is_ground(&self) -> bool {
        matches!(self, TileType::Grass | TileType::Body)
    }

    pub fn is_solid(&self) -> bool {
        matches!(self, TileType::Tree | TileType::Stone | TileType::Obstacle)
    }
}
