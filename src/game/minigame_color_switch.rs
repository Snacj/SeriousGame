use std::f32::consts::TAU;

use crate::engine::font::Font;
use crate::engine::input::Input;
use crate::engine::renderer::Renderer;
use crate::game::minigame::{Minigame, MinigameResult};

use winit::keyboard::KeyCode;

const VIEW_W: f32 = 320.0;
const VIEW_H: f32 = 180.0;

const BALL_X: f32 = VIEW_W / 2.0;
const BALL_RADIUS: f32 = 5.0;
const GRAVITY: f32 = 280.0;
const JUMP_VELOCITY: f32 = -130.0;
const MAX_FALL_SPEED: f32 = 200.0;
const START_Y: f32 = 2000.0;

const RING_RADIUS: f32 = 36.0;
const RING_THICKNESS: f32 = 8.0;
const SEGMENTS_PER_QUADRANT: usize = 6;
const TOTAL_SEGMENTS: usize = SEGMENTS_PER_QUADRANT * 4;
const SEGMENT_ARC: f32 = TAU / TOTAL_SEGMENTS as f32;
const SEGMENT_WIDTH: f32 = 14.0;

const OBSTACLE_SPACING: f32 = 140.0;
const NUM_OBSTACLES: usize = 6;
const SCORE_PER_OBSTACLE: u32 = 15;

const TIME_LIMIT: f32 = 75.0;

const COLORS: [&str; 4] = ["color_red", "color_blue", "color_yellow", "color_green"];

fn pseudo_random(seed: u32) -> u32 {
    seed.wrapping_mul(1103515245).wrapping_add(12345) >> 16
}

fn shuffle_colors(seed: u32) -> [usize; 4] {
    let mut c = [0usize, 1, 2, 3];
    let mut s = seed;
    for i in (1..4).rev() {
        s = s.wrapping_mul(1103515245).wrapping_add(12345);
        let j = (s >> 16) as usize % (i + 1);
        c.swap(i, j);
    }
    c
}

struct RingObstacle {
    y: f32,
    rotation: f32,
    rotation_speed: f32,
    colors: [usize; 4],
    passed: bool,
}

struct ColorPickup {
    y: f32,
    color: usize,
    collected: bool,
}

pub struct ColorSwitchMinigame {
    ball_y: f32,
    ball_vy: f32,
    ball_color: usize,
    obstacles: Vec<RingObstacle>,
    pickups: Vec<ColorPickup>,
    scroll_y: f32,
    score: u32,
    timer: f32,
    finished: MinigameFinish,
    font: Font,
    started: bool,
}

#[derive(PartialEq)]
enum MinigameFinish {
    Running,
    Won,
    Lost,
}

impl ColorSwitchMinigame {
    pub fn new() -> Self {
        let mut obstacles = Vec::with_capacity(NUM_OBSTACLES);
        let mut pickups = Vec::with_capacity(NUM_OBSTACLES);
        let mut seed: u32 = 42;

        for i in 0..NUM_OBSTACLES {
            let obs_y = START_Y - OBSTACLE_SPACING * (i as f32 + 1.0);
            seed = pseudo_random(seed);
            let colors = shuffle_colors(seed);
            let base_speed = 0.9 + i as f32 * 0.08;
            let direction = if i % 2 == 0 { 1.0 } else { -1.0 };

            obstacles.push(RingObstacle {
                y: obs_y,
                rotation: 0.0,
                rotation_speed: base_speed * direction,
                colors,
                passed: false,
            });

            seed = pseudo_random(seed);
            let pickup_color = (seed as usize) % 4;
            pickups.push(ColorPickup {
                y: obs_y + OBSTACLE_SPACING * 0.5,
                color: pickup_color,
                collected: false,
            });
        }

        Self {
            ball_y: START_Y,
            ball_vy: 0.0,
            ball_color: 0,
            obstacles,
            pickups,
            scroll_y: START_Y - VIEW_H * 0.7,
            score: 0,
            timer: 0.0,
            finished: MinigameFinish::Running,
            font: Font::new("font"),
            started: false,
        }
    }

    fn update_ball(&mut self, input: &Input, dt: f32) {
        if input.is_just_pressed(KeyCode::Space) {
            self.ball_vy = JUMP_VELOCITY;
            self.started = true;
        }

        if !self.started {
            return;
        }

        self.ball_vy += GRAVITY * dt;
        self.ball_vy = self.ball_vy.min(MAX_FALL_SPEED);
        self.ball_y += self.ball_vy * dt;

        let target_scroll = self.ball_y - VIEW_H * 0.7;
        if target_scroll < self.scroll_y {
            self.scroll_y = target_scroll;
        }
    }

    fn update_obstacles(&mut self, dt: f32) {
        for obs in &mut self.obstacles {
            obs.rotation += obs.rotation_speed * dt;
            obs.rotation = obs.rotation.rem_euclid(std::f32::consts::TAU);
        }
    }

    fn check_collisions(&mut self) {
        let inner_r = RING_RADIUS - RING_THICKNESS / 2.0;
        let outer_r = RING_RADIUS + RING_THICKNESS / 2.0;

        for obs in &mut self.obstacles {
            let dy = self.ball_y - obs.y;
            let dist = dy.abs();

            if dist > inner_r - BALL_RADIUS && dist < outer_r + BALL_RADIUS {
                let world_angle = if dy > 0.0 {
                    std::f32::consts::FRAC_PI_2
                } else {
                    std::f32::consts::FRAC_PI_2 * 3.0
                };

                let local_angle = (world_angle - obs.rotation).rem_euclid(std::f32::consts::TAU);
                let quadrant = (local_angle / (std::f32::consts::TAU / 4.0)) as usize % 4;

                if obs.colors[quadrant] != self.ball_color {
                    self.finished = MinigameFinish::Lost;
                    return;
                }
            }

            if !obs.passed && self.ball_y < obs.y {
                obs.passed = true;
                self.score += SCORE_PER_OBSTACLE;
            }
        }
    }

    fn check_pickups(&mut self) {
        for pickup in &mut self.pickups {
            if pickup.collected {
                continue;
            }
            let dy = (self.ball_y - pickup.y).abs();
            if dy < BALL_RADIUS + 6.0 {
                pickup.collected = true;
                self.ball_color = pickup.color;
            }
        }
    }

    fn check_death(&mut self) {
        if self.started && self.ball_y > self.scroll_y + VIEW_H + BALL_RADIUS * 2.0 {
            self.finished = MinigameFinish::Lost;
        }
    }

    fn check_win(&mut self) {
        if self.obstacles.iter().all(|o| o.passed) {
            self.finished = MinigameFinish::Won;
        }
    }

    fn screen_y(&self, world_y: f32) -> f32 {
        world_y - self.scroll_y
    }

    fn render_ring(
        &self,
        renderer: &mut Renderer,
        cam_x: f32,
        cam_y: f32,
        obs_index: usize,
        obs: &RingObstacle,
    ) {
        let sy = self.screen_y(obs.y);
        if sy < -RING_RADIUS * 2.0 || sy > VIEW_H + RING_RADIUS * 2.0 {
            return;
        }

        for q in 0..4 {
            let color = COLORS[obs.colors[q]];
            for s in 0..SEGMENTS_PER_QUADRANT {
                let seg_index = q * SEGMENTS_PER_QUADRANT + s;
                let angle = seg_index as f32 * SEGMENT_ARC + obs.rotation;

                let sx = BALL_X + angle.cos() * RING_RADIUS;
                let seg_y = sy + angle.sin() * RING_RADIUS;

                let key = format!("ring_{}_{}", obs_index, seg_index);
                renderer.draw_sprite_rotated_keyed(
                    &key,
                    color,
                    cam_x + sx,
                    cam_y + seg_y,
                    SEGMENT_WIDTH,
                    RING_THICKNESS,
                    angle,
                );
            }
        }
    }

    fn render_ball(&self, renderer: &mut Renderer, cam_x: f32, cam_y: f32) {
        let sy = self.screen_y(self.ball_y);
        let color = COLORS[self.ball_color];
        renderer.draw_sprite_keyed(
            "ball",
            color,
            cam_x + BALL_X - BALL_RADIUS,
            cam_y + sy - BALL_RADIUS,
            BALL_RADIUS * 2.0,
            BALL_RADIUS * 2.0,
        );
    }

    fn render_pickups(&self, renderer: &mut Renderer, cam_x: f32, cam_y: f32) {
        let size = 5.0;
        for (i, pickup) in self.pickups.iter().enumerate() {
            if pickup.collected {
                continue;
            }
            let sy = self.screen_y(pickup.y);
            if sy < -20.0 || sy > VIEW_H + 20.0 {
                continue;
            }

            for (ci, &c) in COLORS.iter().enumerate() {
                let ox = if ci % 2 == 0 { -size / 2.0 } else { size / 2.0 };
                let oy = if ci < 2 { -size / 2.0 } else { size / 2.0 };
                let ckey = format!("pickup_{}_{}", i, ci);
                renderer.draw_sprite_keyed(
                    &ckey,
                    c,
                    cam_x + BALL_X + ox - size / 4.0,
                    cam_y + sy + oy - size / 4.0,
                    size / 2.0,
                    size / 2.0,
                );
            }
        }
    }

    fn render_hud(&self, renderer: &mut Renderer, cam_x: f32, cam_y: f32) {
        let time_left = (TIME_LIMIT - self.timer).max(0.0) as u32;
        let score_text = format!("PUNKTE {}", self.score);
        let timer_text = format!("ZEIT {}", time_left);
        let passed = self.obstacles.iter().filter(|o| o.passed).count();
        let progress_text = format!("{}/{}", passed, NUM_OBSTACLES);

        self.font
            .draw_keyed(renderer, &score_text, cam_x + 4.0, cam_y + 4.0, 0.6, "hud_score");

        let timer_w = self.font.measure(&timer_text, 0.6);
        self.font.draw_keyed(
            renderer,
            &timer_text,
            cam_x + VIEW_W - timer_w - 4.0,
            cam_y + 4.0,
            0.6,
            "hud_timer",
        );

        let progress_w = self.font.measure(&progress_text, 0.6);
        self.font.draw_keyed(
            renderer,
            &progress_text,
            cam_x + VIEW_W / 2.0 - progress_w / 2.0,
            cam_y + 4.0,
            0.6,
            "hud_progress",
        );
    }
}

impl Minigame for ColorSwitchMinigame {
    fn update(&mut self, input: &Input, dt: f32) -> MinigameResult {
        if self.finished != MinigameFinish::Running {
            if input.is_just_pressed(KeyCode::Enter) || input.is_just_pressed(KeyCode::Space) {
                return match self.finished {
                    MinigameFinish::Won => MinigameResult::Won { score: self.score },
                    MinigameFinish::Lost => MinigameResult::Lost,
                    MinigameFinish::Running => unreachable!(),
                };
            }
            return MinigameResult::Running;
        }

        self.timer += dt;
        self.update_ball(input, dt);
        self.update_obstacles(dt);
        self.check_pickups();
        self.check_collisions();
        self.check_death();
        self.check_win();

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

        for (i, obs) in self.obstacles.iter().enumerate() {
            self.render_ring(renderer, cam_x, cam_y, i, obs);
        }

        self.render_pickups(renderer, cam_x, cam_y);
        self.render_ball(renderer, cam_x, cam_y);
        self.render_hud(renderer, cam_x, cam_y);

        let cx = cam_x + VIEW_W / 2.0;
        let cy = cam_y + VIEW_H / 2.0;

        if !self.started {
            self.font.draw_keyed(
                renderer,
                "LEERTASTE ZUM STARTEN",
                cx - self.font.measure("LEERTASTE ZUM STARTEN", 0.6) / 2.0,
                cy,
                0.6,
                "start_hint",
            );
        }

        match self.finished {
            MinigameFinish::Won => {
                renderer.draw_sprite(
                    "ui_panel",
                    cam_x + VIEW_W * 0.2,
                    cy - 20.0,
                    VIEW_W * 0.6,
                    40.0,
                );
                self.font.draw_keyed(
                    renderer,
                    "GEWONNEN",
                    cx - self.font.measure("GEWONNEN", 1.0) / 2.0,
                    cy - 14.0,
                    1.0,
                    "result_title",
                );
                let s = format!("PUNKTE {}", self.score);
                self.font.draw_keyed(
                    renderer,
                    &s,
                    cx - self.font.measure(&s, 0.6) / 2.0,
                    cy + 2.0,
                    0.6,
                    "result_score",
                );
                self.font.draw_keyed(
                    renderer,
                    "ENTER DRUECKEN",
                    cx - self.font.measure("ENTER DRUECKEN", 0.5) / 2.0,
                    cy + 14.0,
                    0.5,
                    "result_hint",
                );
            }
            MinigameFinish::Lost => {
                renderer.draw_sprite(
                    "ui_panel",
                    cam_x + VIEW_W * 0.2,
                    cy - 20.0,
                    VIEW_W * 0.6,
                    40.0,
                );
                self.font.draw_keyed(
                    renderer,
                    "VERLOREN",
                    cx - self.font.measure("VERLOREN", 1.0) / 2.0,
                    cy - 14.0,
                    1.0,
                    "result_title",
                );
                self.font.draw_keyed(
                    renderer,
                    "ENTER DRUECKEN",
                    cx - self.font.measure("ENTER DRUECKEN", 0.5) / 2.0,
                    cy + 14.0,
                    0.5,
                    "result_hint",
                );
            }
            MinigameFinish::Running => {}
        }
    }

    fn on_resize(&mut self, _w: f32, _h: f32) {}
}
