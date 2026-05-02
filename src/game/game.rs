use crate::engine::camera::Camera;
use crate::engine::engine::Engine;
use crate::engine::input::Input;
use crate::engine::renderer::Renderer;
use crate::engine::texture::load_sprites;
use crate::game::object::{Object, ObjectType};
use crate::game::player::Player;
use crate::game::tile::{Tile, TileType};

pub trait Game: Sized {
    fn init(engine: &Engine, renderer: &mut Renderer) -> Self;
    fn update(&mut self, input: &Input, dt: f32);
    fn render(&self, renderer: &mut Renderer);
    fn on_resize(&mut self, width: f32, height: f32);
}

pub const TILE_SIZE: f32 = 16.0;
pub const MAP_SIZE: usize = 32;

pub struct MyGame {
    pub camera: Camera,
    player: Player,
    map: [[Tile; MAP_SIZE]; MAP_SIZE],
}

impl MyGame {
    fn init_map() -> [[Tile; MAP_SIZE]; MAP_SIZE] {
        let mut map = [[Tile::new(0.0, 0.0, TileType::Grass); MAP_SIZE]; MAP_SIZE];

        for (y, row) in map.iter_mut().enumerate() {
            for (x, tile) in row.iter_mut().enumerate() {
                *tile = Tile::new(x as f32 * TILE_SIZE, y as f32 * TILE_SIZE, TileType::Grass);
            }
        }

        map
    }

    pub fn get_tile(&self, x: usize, y: usize) -> &Tile {
        &self.map[y][x]
    }
}

impl Game for MyGame {
    fn init(engine: &Engine, renderer: &mut Renderer) -> Self {
        load_sprites(engine, renderer);

        let (sw, sh) = engine.screen_size();
        let mut camera = Camera::new(180.0);
        camera.update_aspect_ratio(sw, sh);

        Self {
            camera,
            player: Player::new(),
            map: MyGame::init_map(),
        }
    }

    fn update(&mut self, input: &Input, dt: f32) {
        self.player.update(input, dt);
    }

    fn render(&self, renderer: &mut Renderer) {
        // Center camera on the player
        renderer.camera.position.x =
            self.player.x + TILE_SIZE / 2.0 - self.camera.logical_width / 2.0;
        renderer.camera.position.y =
            self.player.y + TILE_SIZE / 2.0 - self.camera.logical_height / 2.0;

        // Draw tiles
        for row in &self.map {
            for tile in row {
                renderer.draw_sprite(tile.sprite_name(), tile.x, tile.y, tile.w, tile.h);
            }
        }

        // Draw player
        self.player.render(renderer);
    }

    fn on_resize(&mut self, width: f32, height: f32) {
        self.camera.update_aspect_ratio(width, height);
    }
}
