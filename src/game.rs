use winit::keyboard::KeyCode;

use crate::engine::Engine;
use crate::input::Input;
use crate::renderer::Renderer;

pub trait Game: Sized {
    fn init(engine: &Engine, renderer: &mut Renderer) -> Self;
    fn update(&mut self, input: &Input, dt: f32);
    fn render(&self, renderer: &mut Renderer);
}

const TILE_SIZE: f32 = 16.0;
const MAP_SIZE: u32 = 32;

pub struct MyGame {
    player_x: f32,
    player_y: f32,
}

impl Game for MyGame {
    fn init(engine: &Engine, renderer: &mut Renderer) -> Self {
        renderer.load_texture(
            &engine.device,
            &engine.queue,
            "grass",
            include_bytes!("../assets/grass.png"),
        );

        renderer.load_texture(
            &engine.device,
            &engine.queue,
            "player",
            include_bytes!("../assets/player.png"),
        );

        Self {
            player_x: 32.0,
            player_y: 32.0,
        }
    }

    fn update(&mut self, input: &Input, dt: f32) {
        let mut speed = 80.0 * dt;

        if input.is_held(KeyCode::ShiftLeft) {
            speed *= 1.5;
        }
        if input.is_held(KeyCode::KeyW) || input.is_held(KeyCode::ArrowUp) {
            self.player_y -= speed;
        }
        if input.is_held(KeyCode::KeyS) || input.is_held(KeyCode::ArrowDown) {
            self.player_y += speed;
        }
        if input.is_held(KeyCode::KeyA) || input.is_held(KeyCode::ArrowLeft) {
            self.player_x -= speed;
        }
        if input.is_held(KeyCode::KeyD) || input.is_held(KeyCode::ArrowRight) {
            self.player_x += speed;
        }
    }

    fn render(&self, renderer: &mut Renderer) {
        // Center camera on the player
        renderer.camera.position.x = self.player_x + TILE_SIZE / 2.0 - renderer.camera.logical_width / 2.0;
        renderer.camera.position.y = self.player_y + TILE_SIZE / 2.0 - renderer.camera.logical_height / 2.0;

        // Draw tiles
        for y in 0..MAP_SIZE {
            for x in 0..MAP_SIZE {
                renderer.draw_sprite(
                    "grass",
                    x as f32 * TILE_SIZE,
                    y as f32 * TILE_SIZE,
                    TILE_SIZE,
                    TILE_SIZE,
                );
            }
        }

        // Draw the player
        renderer.draw_sprite("player", self.player_x, self.player_y, TILE_SIZE, TILE_SIZE);
    }
}
