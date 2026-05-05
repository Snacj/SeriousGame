use winit::keyboard::KeyCode;

use crate::engine::camera::Camera;
use crate::engine::collision::{CollisionBox, CollisionWorld, Rect};
use crate::engine::font::Font;
use crate::engine::input::Input;
use crate::engine::map;
use crate::engine::renderer::Renderer;
use crate::game::GameState;
use crate::game::object::{Object, ObjectType};
use crate::game::player::Player;
use crate::game::tile::{Tile, TileType};

pub const TILE_SIZE: f32 = 16.0;
pub const MAP_SIZE: usize = 32;

pub struct MyGame {
    pub camera: Camera,
    font: Font,
    objects: Vec<Object>,
    player: Player,
    map: [[Tile; MAP_SIZE]; MAP_SIZE],
    collision_world: CollisionWorld,
    debug: bool,
    fps: u32,
    frame_count: u32,
    fps_timer: f32,
}

impl MyGame {
    pub fn new(screen_width: f32, screen_height: f32) -> Self {
        let mut camera = Camera::new(180.0);
        camera.update_aspect_ratio(screen_width, screen_height);

        let collision_box = CollisionBox::new(8.0, 64.0, 80.0, 64.0);
        let mut objects: Vec<Object> = Vec::new();

        objects.push(Object::new(
            4.0 * TILE_SIZE,
            4.0 * TILE_SIZE,
            96.0,
            128.0,
            ObjectType::House,
            collision_box,
        ));
        objects.push(Object::new(
            16.0 * TILE_SIZE,
            16.0 * TILE_SIZE,
            96.0,
            128.0,
            ObjectType::House,
            collision_box,
        ));
        objects.push(Object::new(
            24.0 * TILE_SIZE,
            24.0 * TILE_SIZE,
            96.0,
            128.0,
            ObjectType::House,
            collision_box,
        ));

        let map = map::load_map(include_bytes!("../../map/test_map.json"));
        let mut collision_world = CollisionWorld::new();

        for row in &map {
            for tile in row {
                if tile.tile_type.is_solid() {
                    collision_world.add(Rect::new(tile.x, tile.y, tile.w, tile.h));
                }
            }
        }
        for object in &objects {
            let rect = object.collision_box.world_rect(object.x, object.y);
            collision_world.add(rect);
        }

        let player = Player::new();

        Self {
            camera,
            font: Font::new("font"),
            objects,
            player,
            map,
            collision_world,
            debug: false,
            fps: 0,
            frame_count: 0,
            fps_timer: 0.0,
        }
    }

    /// Normal gameplay update. Returns Some(state) to transition away.
    pub fn update(&mut self, input: &Input, dt: f32) -> Option<GameState> {
        self.player.update(input, dt);

        let (nx, ny) = self.collision_world.resolve_player(
            &self.player.collision_box,
            self.player.x,
            self.player.y,
        );
        self.player.x = nx;
        self.player.y = ny;

        if input.is_just_released(KeyCode::F9) {
            self.debug = !self.debug;
        }

        // Pause on Escape and move self into the Paused variant
        if input.is_just_pressed(KeyCode::Space) {
            let game = std::mem::replace(self, MyGame::new(0.0, 0.0));
            return Some(GameState::Paused(game));
        }

        // FPS counter
        self.frame_count += 1;
        self.fps_timer += dt;
        if self.fps_timer >= 1.0 {
            self.fps = self.frame_count;
            self.frame_count = 0;
            self.fps_timer -= 1.0;
        }

        None
    }

    /// Called while paused. Escape resumes, everything else stays paused.
    pub fn update_paused(&mut self, input: &Input) -> Option<GameState> {
        if input.is_just_pressed(KeyCode::Space) {
            let game = std::mem::replace(self, MyGame::new(0.0, 0.0));
            return Some(GameState::Playing(game));
        }
        None
    }

    pub fn render(&self, renderer: &mut Renderer) {
        renderer.camera.position.x =
            self.player.x + TILE_SIZE / 2.0 - self.camera.logical_width / 2.0;
        renderer.camera.position.y =
            self.player.y + TILE_SIZE / 2.0 - self.camera.logical_height / 2.0;

        // Ground always behind everything
        for row in &self.map {
            for tile in row {
                if tile.tile_type.is_ground() {
                    renderer.draw_sprite(tile.sprite_name(), tile.x, tile.y, tile.w, tile.h);
                }
            }
        }

        // Y-sorted draw list
        let mut calls: Vec<(f32, DrawCall)> = Vec::new();

        for (y, row) in self.map.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if tile.tile_type.is_solid() {
                    calls.push((tile.y + tile.h, DrawCall::Tile(y, x)));
                }
            }
        }
        for (i, object) in self.objects.iter().enumerate() {
            calls.push((object.feet_y(), DrawCall::Object(i)));
        }
        calls.push((self.player.y + TILE_SIZE * 2.0, DrawCall::Player));
        calls.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        for (order, (_, call)) in calls.iter().enumerate() {
            match call {
                DrawCall::Tile(row, col) => {
                    let tile = &self.map[*row][*col];
                    let key = format!("{}_{}", tile.sprite_name(), order);
                    renderer.draw_sprite_keyed(
                        &key,
                        tile.sprite_name(),
                        tile.x,
                        tile.y,
                        tile.w,
                        tile.h,
                    );
                }
                DrawCall::Object(i) => {
                    self.objects[*i].render_ordered(renderer, order, self.debug);
                }
                DrawCall::Player => {
                    self.player.render_ordered(renderer, order, self.debug);
                }
            }
        }

        if self.debug {
            self.draw_fps(renderer);
        }
    }

    pub fn on_resize(&mut self, width: f32, height: f32) {
        self.camera.update_aspect_ratio(width, height);
    }

    fn draw_fps(&self, renderer: &mut Renderer) {
        let text = format!("{} FPS", self.fps);
        let scale = 0.5;
        let padding = 2.0;
        let text_w = self.font.measure(&text, scale);
        let x = renderer.camera.position.x + renderer.camera.logical_width - text_w - padding;
        let y = renderer.camera.position.y + padding;
        self.font.draw(renderer, &text, x, y, scale);
    }
}

#[derive(Debug)]
enum DrawCall {
    Tile(usize, usize),
    Object(usize),
    Player,
}
