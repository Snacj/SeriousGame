use serde::Deserialize;
use crate::game::tile::{Tile, TileType};
use crate::game::game::{TILE_SIZE, MAP_SIZE};

#[derive(Deserialize)]
pub struct TiledMap {
    pub width: usize,
    pub height: usize,
    pub layers: Vec<TiledLayer>,
}

#[derive(Deserialize)]
pub struct TiledLayer {
    pub name: String,
    #[serde(rename = "type")]
    pub layer_type: String,
    pub data: Option<Vec<u32>>,
}

pub fn load_map(bytes: &[u8]) -> [[Tile; MAP_SIZE]; MAP_SIZE] {
    let tiled: TiledMap = serde_json::from_slice(bytes)
        .expect("Failed to parse Tiled JSON map");

    let layer = tiled
        .layers
        .iter()
        .find(|l| l.layer_type == "tilelayer")
        .expect("No tile layer found in map");

    let data = layer.data.as_ref().expect("Tile layer has no data");

    let mut map = [[Tile::new(0.0, 0.0, TileType::Grass); MAP_SIZE]; MAP_SIZE];

    for (y, row) in map.iter_mut().enumerate() {
        for (x, tile) in row.iter_mut().enumerate() {
            let idx = y * tiled.width + x;

            let tile_id = if idx < data.len() { data[idx] } else { 0 };

            let tile_type = TileType::from_tiled_id(tile_id);
            *tile = Tile::new(x as f32 * TILE_SIZE, y as f32 * TILE_SIZE, tile_type);
        }
    }

    map
}
