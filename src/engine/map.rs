use serde::Deserialize;
use crate::game::tile::{Tile, TileType};
use crate::game::game::{TILE_SIZE, MAP_WIDTH, MAP_HEIGHT};

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
    /// Custom properties set in Tiled's layer properties panel
    #[serde(default)]
    pub properties: Vec<TiledProperty>,
}

#[derive(Deserialize)]
pub struct TiledProperty {
    pub name: String,
    /// "bool", "int", "string", "float" etc.
    #[serde(rename = "type")]
    pub property_type: String,
    pub value: serde_json::Value,
}

impl TiledLayer {
    pub fn bool_prop(&self, name: &str) -> bool {
        self.properties
            .iter()
            .find(|p| p.name == name)
            .and_then(|p| p.value.as_bool())
            .unwrap_or(false)
    }
}

/// Load a two-layer Tiled map.
///
/// Layer custom properties (set in Tiled):
///   is_ground: bool, tiles on this layer are drawn first, behind everything
///   is_solid:  bool, tiles on this layer block the player
///
/// Layer 1 (ground):  is_ground = true,  is_solid = false  -> Body tiles
/// Layer 2 (solid):   is_ground = true,  is_solid = true   -> Obstacle tiles
///
/// Tiles from later layers overwrite earlier layers at the same position,
/// so solid tiles sit on top of ground tiles correctly.
pub fn load_map(bytes: &[u8]) -> [[Tile; MAP_WIDTH]; MAP_HEIGHT] {
    let tiled: TiledMap = serde_json::from_slice(bytes)
        .expect("Failed to parse Tiled JSON map");

    let mut map = [[Tile::new(0.0, 0.0, TileType::Body); MAP_WIDTH]; MAP_HEIGHT];

    for layer in &tiled.layers {
        // Skip anything that isn't a tile layer (object layers, image layers etc.)
        if layer.layer_type != "tilelayer" {
            continue;
        }

        let data = match &layer.data {
            Some(d) => d,
            None => continue,
        };

        let is_ground = layer.bool_prop("is_ground");
        let is_solid  = layer.bool_prop("is_solid");

        for (y, row) in map.iter_mut().enumerate() {
            for (x, tile) in row.iter_mut().enumerate() {
                let idx = y * tiled.width + x;
                let tile_id = if idx < data.len() { data[idx] } else { 0 };

                if tile_id == 0 {
                    continue;
                }

                let tile_type = TileType::from_layer(tile_id, is_ground, is_solid);
                *tile = Tile::new(x as f32 * TILE_SIZE, y as f32 * TILE_SIZE, tile_type);
            }
        }
    }

    map
}
