use winit::keyboard::KeyCode;

use crate::{
    engine::{
        animation::{Animation, AnimationSet},
        collision::CollisionBox,
        font::Font,
        input::Input,
        renderer::Renderer,
    },
    game::{dialogue::{DialogueData, draw_9slice}, game::TILE_SIZE, object::Object},
};

const ATLAS_TEXTURE: &str = "player";
const FRAME_W: f32 = 16.0;
const FRAME_H: f32 = 32.0;
const ATLAS_W: f32 = 80.0;
const ATLAS_H: f32 = 256.0;

pub const INTERACT_RANGE: f32 = TILE_SIZE * 5.0;

pub struct Player {
    pub x: f32,
    pub y: f32,
    pub speed: f32,
    pub collision_box: CollisionBox,
    idle: bool,
    animation_state: PlayerAnimationState,
    animations: AnimationSet,
}

impl Player {
    pub fn new() -> Self {
        #[rustfmt::skip]
        let animations = AnimationSet::new()
            .add("walk_down",   Animation::new(5, 0.5, 0))
            .add("walk_up",     Animation::new(5, 0.5, 1))
            .add("walk_right",  Animation::new(4, 0.5, 2))
            .add("walk_left",   Animation::new(4, 0.5, 3))
            .add("idle_down",   Animation::new(4, 1.0, 4))
            .add("idle_up",     Animation::new(3, 1.0, 5))
            .add("idle_right",  Animation::new(5, 1.0, 6))
            .add("idle_left",   Animation::new(5, 1.0, 7));

        let collision_box = CollisionBox::new(4.0, 12.0, 8.0, 4.0);

        Self {
            x: 16.0 * TILE_SIZE,
            y: 16.0 * TILE_SIZE,
            speed: 80.0,
            collision_box,
            idle: true,
            animation_state: PlayerAnimationState::Down,
            animations,
        }
    }

    /// Render with a unique batch key so Y-sorting works correctly.
    pub fn render_ordered(
        &self,
        renderer: &mut Renderer,
        order: usize,
        debug: bool,
        show_interact_prompt: bool,
        font: &Font,
    ) {
        let (row, col) = self.animations.current_frame();
        let key = format!("{}_{}", ATLAS_TEXTURE, order);
        renderer.draw_sprite_frame_keyed(
            &key,
            ATLAS_TEXTURE,
            self.x,
            self.y - TILE_SIZE,
            TILE_SIZE,
            TILE_SIZE * 2.0,
            col,
            row,
            FRAME_W,
            FRAME_H,
            ATLAS_W,
            ATLAS_H,
        );

        if show_interact_prompt {
            let prompt = "ENTER";
            let scale = 0.5;
            let prompt_w = font.measure(prompt, scale);
            let prompt_x = self.x + TILE_SIZE / 2.0 - prompt_w / 2.0;
            let prompt_y = self.y - TILE_SIZE * 2.0;
            let indicator_key = format!("indicator_{}", order);
            draw_9slice(
                renderer,
                "interaction_background",
                prompt_x - 4.0,
                prompt_y - 6.0,
                prompt_w + 3.0,
                18.0,
                24.0,
                24.0,
                8.0,
            );
            font.draw_ui(renderer, prompt, prompt_x, prompt_y, scale);
        }

        if debug {
            let cb = &self.collision_box;
            let debug_key = format!("debug_player_{}", order);
            renderer.draw_sprite_keyed(
                &debug_key,
                "debug_red",
                self.x + cb.offset_x,
                self.y + cb.offset_y,
                cb.width,
                cb.height,
            );
        }
    }

    pub fn update(&mut self, input: &Input, dt: f32, objects: &[Object]) -> Option<DialogueData> {
        self.update_position(input, dt);
        self.update_animation();
        self.animations.step(dt);

        // Check for interaction press
        if input.is_just_pressed(KeyCode::Enter) {
            return self.try_interact(objects);
        }

        None
    }

    /// Find the nearest interactable object and return its dialogue, if any.
    fn try_interact(&self, objects: &[Object]) -> Option<DialogueData> {
        let mut nearest_dist = INTERACT_RANGE;
        let mut nearest: Option<&Object> = None;

        for obj in objects {
            let cx = obj.x + obj.w / 2.0;
            let cy = obj.y + obj.h / 2.0;
            let dx = self.x - cx;
            let dy = self.y - cy;
            let dist = (dx * dx + dy * dy).sqrt();

            if dist < nearest_dist && !obj.completed {
                nearest_dist = dist;
                nearest = Some(obj);
            }
        }

        if let Some(obj) = nearest {
            if let Some(data) = &obj.interaction {
                log::info!(
                    "Interacting with {} at ({:.0}, {:.0})",
                    obj.object_type.object_name(),
                    obj.x,
                    obj.y
                );
                return Some(data.clone());
            }
        }

        let tile_x = (self.x / TILE_SIZE) as usize;
        let tile_y = (self.y / TILE_SIZE) as usize;
        log::info!("No interactable nearby, tile ({}, {})", tile_x, tile_y);

        None
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

        let next_x = dx * speed;
        let next_y = dy * speed;

        self.x += next_x;
        self.y += next_y;
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
        log::info!("Interacting with tile ({}, {})", tile_x, tile_y);
    }
}

pub enum PlayerAnimationState {
    Up,
    Down,
    Left,
    Right,
}
