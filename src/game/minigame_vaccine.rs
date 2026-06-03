use crate::engine::animation::Animation;
use crate::engine::font::Font;
use crate::engine::input::Input;
use crate::engine::renderer::Renderer;
use crate::game::minigame::{Minigame, MinigameResult};

use winit::keyboard::KeyCode;

const TILE: f32 = 16.0;

const VIEW_W: f32 = 320.0;
const VIEW_H: f32 = 180.0;

const CIRCLE_X: f32 = VIEW_W / 2.0;
const CIRCLE_Y: f32 = VIEW_H / 2.0 - 10.0;
const CIRCLE_RADIUS: f32 = 28.0;
const CIRCLE_DISPLAY_SIZE: f32 = CIRCLE_RADIUS * 2.0;

const PIN_LENGTH: f32 = 26.0;
const PIN_WIDTH: f32 = 3.0;
const PIN_HEAD_RADIUS: f32 = 5.0;
const PIN_INSET: f32 = 10.0;
const PIN_SHOOT_SPEED: f32 = 260.0;
const PIN_START_Y: f32 = VIEW_H - 20.0;
const PIN_QUEUE_X: f32 = VIEW_W / 2.0;
const PIN_QUEUE_SPACING: f32 = 12.0;

const PIN_COLLISION_ANGLE: f32 = 0.11;

const BASE_ROTATION_SPEED: f32 = 1.5;
const SPEED_INCREMENT: f32 = 0.20;

const PINS_PER_LEVEL: usize = 6;
const MAX_LEVEL: u32 = 4;
const PRE_PLACED_PER_LEVEL: usize = 2;

const TIME_LIMIT: f32 = 90.0;

struct AttachedPin {
    angle: f32,
    anim: Animation,
}

struct FlyingPin {
    x: f32,
    y: f32,
    active: bool,
}

pub struct AaMinigame {
    attached_pins: Vec<AttachedPin>,
    flying_pin: Option<FlyingPin>,
    pins_remaining: usize,
    rotation_angle: f32,
    rotation_speed: f32,
    rotation_direction: f32,
    level: u32,
    score: u32,
    timer: f32,
    finished: MinigameFinish,
    font: Font,
    level_transition_timer: f32,
}

#[derive(PartialEq)]
enum MinigameFinish {
    Running,
    Won,
    Lost,
}

impl AaMinigame {
    pub fn new() -> Self {
        let mut game = Self {
            attached_pins: Vec::new(),
            flying_pin: None,
            pins_remaining: PINS_PER_LEVEL,
            rotation_angle: 0.0,
            rotation_speed: BASE_ROTATION_SPEED,
            rotation_direction: 1.0,
            level: 1,
            score: 0,
            timer: 0.0,
            finished: MinigameFinish::Running,
            font: Font::new("font"),
            level_transition_timer: 0.0,
        };
        game.place_initial_pins();
        game
    }

    fn place_initial_pins(&mut self) {
        self.attached_pins.clear();
        let count = PRE_PLACED_PER_LEVEL + (self.level as usize).saturating_sub(1);
        let step = std::f32::consts::TAU / count as f32;
        for i in 0..count {
            self.attached_pins.push(AttachedPin {
                angle: step * i as f32,
                anim: Animation::new(1, 0.1, 0).one_shot(),
            });
        }
    }

    fn start_level(&mut self, level: u32) {
        self.level = level;
        self.pins_remaining = PINS_PER_LEVEL + (level as usize - 1) * 2;
        self.rotation_speed = BASE_ROTATION_SPEED + SPEED_INCREMENT * (level as f32 - 1.0) * 1.5;
        self.rotation_direction = if level % 2 == 0 { -1.0 } else { 1.0 };
        self.flying_pin = None;
        self.level_transition_timer = 0.0;
        self.place_initial_pins();
    }

    fn attached_pin_tip(angle: f32, rotation: f32) -> (f32, f32) {
        let world_angle = angle + rotation;
        let tip_dist = CIRCLE_RADIUS - PIN_INSET + PIN_LENGTH;
        (
            CIRCLE_X + world_angle.cos() * tip_dist,
            CIRCLE_Y + world_angle.sin() * tip_dist,
        )
    }

    fn would_collide(&self, new_angle: f32) -> bool {
        for pin in &self.attached_pins {
            let mut diff = (new_angle - pin.angle).rem_euclid(std::f32::consts::TAU);
            if diff > std::f32::consts::PI {
                diff = std::f32::consts::TAU - diff;
            }
            if diff < PIN_COLLISION_ANGLE {
                return true;
            }
        }
        false
    }

    fn update_rotation(&mut self, dt: f32) {
        self.rotation_angle += self.rotation_speed * self.rotation_direction * dt;
        self.rotation_angle = self.rotation_angle.rem_euclid(std::f32::consts::TAU);
    }

    fn update_shooting(&mut self, input: &Input) {
        if self.flying_pin.is_some() || self.pins_remaining == 0 || self.level_transition_timer > 0.0 {
            return;
        }
        if input.is_just_pressed(KeyCode::Space) {
            self.flying_pin = Some(FlyingPin {
                x: PIN_QUEUE_X,
                y: PIN_START_Y,
                active: true,
            });
        }
    }

    fn update_flying_pin(&mut self, dt: f32) {
        let hit_circle;
        let pin_x;
        let pin_y;

        if let Some(ref mut pin) = self.flying_pin {
            if !pin.active {
                return;
            }
            pin.y -= PIN_SHOOT_SPEED * dt;
            pin_x = pin.x;
            pin_y = pin.y;

            let dx = pin_x - CIRCLE_X;
            let dy = pin_y - CIRCLE_Y;
            let dist = (dx * dx + dy * dy).sqrt();
            hit_circle = dist <= CIRCLE_RADIUS - PIN_INSET + PIN_LENGTH;
        } else {
            return;
        }

        if hit_circle {
            let dx = pin_x - CIRCLE_X;
            let dy = pin_y - CIRCLE_Y;
            let world_angle = dy.atan2(dx);
            let local_angle = (world_angle - self.rotation_angle).rem_euclid(std::f32::consts::TAU);

            if self.would_collide(local_angle) {
                self.finished = MinigameFinish::Lost;
                self.flying_pin = None;
                return;
            }

            self.attached_pins.push(AttachedPin {
                angle: local_angle,
                anim: Animation::new(4, 0.3, 0).one_shot(),
            });
            self.pins_remaining -= 1;
            self.score += 10 * self.level;
            self.flying_pin = None;
        }
    }

    fn update_level_progress(&mut self) {
        if self.pins_remaining == 0 && self.flying_pin.is_none() && self.level_transition_timer <= 0.0 {
            if self.level >= MAX_LEVEL {
                self.finished = MinigameFinish::Won;
            } else {
                self.level_transition_timer = 1.2;
            }
        }
    }

    fn update_level_transition(&mut self, dt: f32) {
        if self.level_transition_timer > 0.0 {
            self.level_transition_timer -= dt;
            if self.level_transition_timer <= 0.0 {
                self.start_level(self.level + 1);
            }
        }
    }
    fn render_circle(&self, renderer: &mut Renderer, cam_x: f32, cam_y: f32) {
        renderer.draw_sprite(
            "circle_target",
            cam_x + CIRCLE_X - CIRCLE_RADIUS,
            cam_y + CIRCLE_Y - CIRCLE_RADIUS,
            CIRCLE_DISPLAY_SIZE,
            CIRCLE_DISPLAY_SIZE,
        );
    }

    fn render_attached_pins(&self, renderer: &mut Renderer, cam_x: f32, cam_y: f32) {
        for (i, pin) in self.attached_pins.iter().enumerate() {
            let world_angle = pin.angle + self.rotation_angle;
            let mid_dist = CIRCLE_RADIUS - PIN_INSET + PIN_LENGTH / 2.0;
            let mid_x = CIRCLE_X + world_angle.cos() * mid_dist;
            let mid_y = CIRCLE_Y + world_angle.sin() * mid_dist;

            let outer_dist = CIRCLE_RADIUS - PIN_INSET + PIN_LENGTH;
            let outer_x = CIRCLE_X + world_angle.cos() * outer_dist;
            let outer_y = CIRCLE_Y + world_angle.sin() * outer_dist;

            let key_shaft = format!("apin_shaft_{}", i);
            renderer.draw_sprite_rotated_keyed(
                &key_shaft,
                "white",
                cam_x + mid_x,
                cam_y + mid_y,
                PIN_WIDTH,
                PIN_LENGTH,
                world_angle - std::f32::consts::FRAC_PI_2,
            );

            let key_head = format!("apin_head_{}", i);
            renderer.draw_sprite_rotated_keyed(
                &key_head,
                "pin_head",
                cam_x + outer_x,
                cam_y + outer_y,
                PIN_HEAD_RADIUS * 2.0,
                PIN_HEAD_RADIUS * 2.0,
                world_angle - std::f32::consts::FRAC_PI_2,
            );
        }
    }

    fn render_flying_pin(&self, renderer: &mut Renderer, cam_x: f32, cam_y: f32) {
        if let Some(ref pin) = self.flying_pin {
            renderer.draw_sprite_keyed(
                "flying_shaft",
                "white",
                cam_x + pin.x - PIN_WIDTH / 2.0,
                cam_y + pin.y,
                PIN_WIDTH,
                PIN_LENGTH,
            );
            renderer.draw_sprite_keyed(
                "flying_head",
                "pin_head",
                cam_x + pin.x - PIN_HEAD_RADIUS,
                cam_y + pin.y - PIN_HEAD_RADIUS,
                PIN_HEAD_RADIUS * 2.0,
                PIN_HEAD_RADIUS * 2.0,
            );
        }
    }

    fn render_queue(&self, renderer: &mut Renderer, cam_x: f32, cam_y: f32) {
        let count = self.pins_remaining.min(8);
        for i in 0..count {
            let y = PIN_START_Y + 10.0 + i as f32 * PIN_QUEUE_SPACING;
            let alpha_key = format!("queue_pin_{}", i);
            renderer.draw_sprite_keyed(
                &alpha_key,
                "pin_head",
                cam_x + PIN_QUEUE_X - PIN_HEAD_RADIUS,
                cam_y + y - PIN_HEAD_RADIUS,
                PIN_HEAD_RADIUS * 2.0,
                PIN_HEAD_RADIUS * 2.0,
            );
        }
    }

    fn render_hud(&self, renderer: &mut Renderer, cam_x: f32, cam_y: f32) {
        let time_left = (TIME_LIMIT - self.timer).max(0.0) as u32;
        let score_text = format!("SCORE {}", self.score);
        let timer_text = format!("TIME {}", time_left);
        let level_text = format!("LV {}", self.level);

        self.font.draw_keyed(renderer, &score_text, cam_x + 4.0, cam_y + 4.0, 0.6, "hud_score");

        let timer_w = self.font.measure(&timer_text, 0.6);
        self.font.draw_keyed(renderer, &timer_text, cam_x + VIEW_W - timer_w - 4.0, cam_y + 4.0, 0.6, "hud_timer");

        let level_w = self.font.measure(&level_text, 0.6);
        self.font.draw_keyed(renderer, &level_text, cam_x + VIEW_W / 2.0 - level_w / 2.0, cam_y + 4.0, 0.6, "hud_level");
    }
}

impl Minigame for AaMinigame {
    fn update(&mut self, input: &Input, dt: f32) -> MinigameResult {
        if self.finished != MinigameFinish::Running {
            if input.is_just_pressed(KeyCode::Enter) || input.is_just_pressed(KeyCode::Space) {
                return match self.finished {
                    MinigameFinish::Won  => MinigameResult::Won { score: self.score },
                    MinigameFinish::Lost => MinigameResult::Lost,
                    MinigameFinish::Running => unreachable!(),
                };
            }
            return MinigameResult::Running;
        }

        self.timer += dt;
        self.update_rotation(dt);
        self.update_shooting(input);
        self.update_flying_pin(dt);
        self.update_level_progress();
        self.update_level_transition(dt);

        if self.timer >= TIME_LIMIT {
            self.finished = MinigameFinish::Lost;
        }

        MinigameResult::Running
    }

    fn render(&self, renderer: &mut Renderer) {
        let cam_x = renderer.camera.position.x;
        let cam_y = renderer.camera.position.y;
        let cam_w = renderer.camera.logical_width;
        let cam_h = renderer.camera.logical_height;

        renderer.draw_sprite("body_background", cam_x, cam_y, cam_w, cam_h);

        self.render_circle(renderer, cam_x, cam_y);
        self.render_attached_pins(renderer, cam_x, cam_y);
        self.render_flying_pin(renderer, cam_x, cam_y);
        self.render_queue(renderer, cam_x, cam_y);
        self.render_hud(renderer, cam_x, cam_y);

        if self.level_transition_timer > 0.0 {
            let cx = cam_x + VIEW_W / 2.0;
            let cy = cam_y + VIEW_H / 2.0;
            let text = format!("LEVEL {}!", self.level);
            let tw = self.font.measure(&text, 1.0);
            self.font.draw_keyed(renderer, &text, cx - tw / 2.0, cy - 6.0, 1.0, "level_announce");
        }

        let cx = cam_x + VIEW_W / 2.0;
        let cy = cam_y + VIEW_H / 2.0;

        match self.finished {
            MinigameFinish::Won => {
                renderer.draw_sprite("ui_panel", cam_x + VIEW_W * 0.2, cy - 20.0, VIEW_W * 0.6, 40.0);
                self.font.draw_keyed(renderer, "YOU WIN",
                    cx - self.font.measure("YOU WIN", 1.0) / 2.0, cy - 14.0, 1.0, "result_title");
                let s = format!("SCORE {}", self.score);
                self.font.draw_keyed(renderer, &s,
                    cx - self.font.measure(&s, 0.6) / 2.0, cy + 2.0, 0.6, "result_score");
                self.font.draw_keyed(renderer, "PRESS ENTER",
                    cx - self.font.measure("PRESS ENTER", 0.5) / 2.0, cy + 14.0, 0.5, "result_hint");
            }
            MinigameFinish::Lost => {
                renderer.draw_sprite("ui_panel", cam_x + VIEW_W * 0.2, cy - 20.0, VIEW_W * 0.6, 40.0);
                self.font.draw_keyed(renderer, "YOU LOST",
                    cx - self.font.measure("YOU LOST", 1.0) / 2.0, cy - 14.0, 1.0, "result_title");
                self.font.draw_keyed(renderer, "PRESS ENTER",
                    cx - self.font.measure("PRESS ENTER", 0.5) / 2.0, cy + 14.0, 0.5, "result_hint");
            }
            MinigameFinish::Running => {}
        }
    }

    fn on_resize(&mut self, _w: f32, _h: f32) {}
}
