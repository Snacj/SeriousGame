use winit::keyboard::KeyCode;

use crate::{
    animation::{Animation, AnimationSet},
    game::TILE_SIZE,
    input::Input,
    renderer::Renderer,
};

const ATLAS_TEXTURE: &str = "player";
const FRAME_W: f32 = 16.0;
const FRAME_H: f32 = 16.0;
const ATLAS_W: f32 = 64.0;
const ATLAS_H: f32 = 128.0;

pub struct Player {
    pub x: f32,
    pub y: f32,
    pub speed: f32,
    idle: bool,
    animation_state: PlayerAnimationState,
    animations: AnimationSet,
}

impl Player {
    pub fn new() -> Self {
        let animations = AnimationSet::new()
            .add("walk_down",   Animation::new(2, 0.4, 0))
            .add("walk_left",   Animation::new(3, 0.4, 1))
            .add("walk_right",  Animation::new(3, 0.4, 2))
            .add("walk_up",     Animation::new(2, 0.4, 3))
            .add("idle_down",   Animation::new(2, 0.8, 4))
            .add("idle_left",   Animation::new(2, 0.8, 5))
            .add("idle_right",  Animation::new(2, 0.8, 6))
            .add("idle_up",     Animation::new(2, 0.8, 7));
        Self {
            x: 16.0 * TILE_SIZE,
            y: 16.0 * TILE_SIZE,
            speed: 80.0,
            idle: true,
            animation_state: PlayerAnimationState::Down,
            animations,
        }
    }

    pub fn update(&mut self, input: &Input, dt: f32) {
        self.update_position(input, dt);

        self.update_animation();
        self.animations.step(dt);

        if input.just_released(KeyCode::Enter) {
            self.interact();
        }
    }

    pub fn render(&self, renderer: &mut Renderer) {
        let (row, col) = self.animations.current_frame();
        renderer.draw_sprite_frame(
            ATLAS_TEXTURE,
            self.x,
            self.y,
            TILE_SIZE,
            TILE_SIZE,
            col,
            row,
            FRAME_W,
            FRAME_H,
            ATLAS_W,
            ATLAS_H,
        );
    }

    fn update_position(&mut self, input: &Input, dt: f32) {
        let mut speed = self.speed * dt;
        if input.is_held(KeyCode::ShiftLeft) {
            speed *= 1.5;
        }

        let mut dx: f32 = 0.0;
        let mut dy: f32 = 0.0;

        if input.is_held(KeyCode::KeyW) || input.is_held(KeyCode::ArrowUp) {
            dy -= 1.0;
        }
        if input.is_held(KeyCode::KeyS) || input.is_held(KeyCode::ArrowDown) {
            dy += 1.0;
        }
        if input.is_held(KeyCode::KeyA) || input.is_held(KeyCode::ArrowLeft) {
            dx -= 1.0;
        }
        if input.is_held(KeyCode::KeyD) || input.is_held(KeyCode::ArrowRight) {
            dx += 1.0;
        }

        let len = (dx * dx + dy * dy).sqrt();
        if len > 0.0 {
            dx /= len;
            dy /= len;
        }

        if dy < 0.0 {
            self.idle = false;
            self.animation_state = PlayerAnimationState::Up;
        } else if dy > 0.0 {
            self.idle = false;
            self.animation_state = PlayerAnimationState::Down;
        } else if dx < 0.0 {
            self.idle = false;
            self.animation_state = PlayerAnimationState::Left;
        } else if dx > 0.0 {
            self.idle = false;
            self.animation_state = PlayerAnimationState::Right;
        } else {
            self.idle = true;
        }

        self.x += dx * speed;
        self.y += dy * speed;
    }

    fn update_animation(&mut self) {
        if self.idle {
            match self.animation_state {
                PlayerAnimationState::Up => self.animations.play("idle_up"),
                PlayerAnimationState::Down => self.animations.play("idle_down"),
                PlayerAnimationState::Left => self.animations.play("idle_left"),
                PlayerAnimationState::Right => self.animations.play("idle_right"),
            }
        } else {
            match self.animation_state {
                PlayerAnimationState::Up => self.animations.play("walk_up"),
                PlayerAnimationState::Down => self.animations.play("walk_down"),
                PlayerAnimationState::Left => self.animations.play("walk_left"),
                PlayerAnimationState::Right => self.animations.play("walk_right"),
            }
        }
    }

    fn interact(&self) {
        let tile_x = (self.x / TILE_SIZE) as usize;
        let tile_y = (self.y / TILE_SIZE) as usize;
        tracing::info!("Interacting with tile ({}, {})", tile_x, tile_y);
    }
}

pub enum PlayerAnimationState {
    Up,
    Down,
    Left,
    Right,
}
