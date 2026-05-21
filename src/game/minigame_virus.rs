use crate::engine::font::Font;
use crate::engine::input::Input;
use crate::engine::renderer::Renderer;
use crate::game::minigame::{Minigame, MinigameResult};

use winit::keyboard::KeyCode;

const TILE: f32 = 16.0;

// World size
const VIEW_W: f32 = 320.0;
const VIEW_H: f32 = 180.0;

// Player
const PLAYER_SPEED: f32 = 120.0;
const PLAYER_Y: f32 = VIEW_H - TILE * 2.0;
const SHOOT_COOLDOWN: f32 = 0.4;

// Bullets
const BULLET_SPEED: f32 = 200.0;
const BULLET_W: f32 = 3.0;
const BULLET_H: f32 = 8.0;

// Virus grid
const COLS: usize = 9;
const ROWS: usize = 3;
const VIRUS_W: f32 = TILE;
const VIRUS_H: f32 = TILE;
const VIRUS_SPACING_X: f32 = 26.0;
const VIRUS_SPACING_Y: f32 = 22.0;
const GRID_TOP: f32 = 24.0;
const GRID_LEFT: f32 = 20.0;
const VIRUS_MOVE_SPEED: f32 = 40.0; // horizontal speed
const VIRUS_DROP: f32 = 12.0; // how far they drop when hitting an edge

// Game timer
const TIME_LIMIT: f32 = 60.0;

struct Bullet {
    x: f32,
    y: f32,
    active: bool,
}

struct Virus {
    x: f32,
    y: f32,
    alive: bool,
}

pub struct VirusMinigame {
    // Player
    player_x: f32,
    shoot_timer: f32,

    // Bullets
    bullets: Vec<Bullet>,

    // Virus grid
    viruses: Vec<Virus>,
    grid_dx: f32,       // current horizontal direction (+1 or -1)
    grid_offset_x: f32, // accumulated horizontal movement
    grid_offset_y: f32, // accumulated vertical drop

    // State
    timer: f32,
    score: u32,
    finished: MinigameFinish,

    font: Font,
}

#[derive(PartialEq)]
enum MinigameFinish {
    Running,
    Won,
    Lost,
}

impl VirusMinigame {
    pub fn new() -> Self {
        let mut viruses = Vec::with_capacity(COLS * ROWS);
        for row in 0..ROWS {
            for col in 0..COLS {
                viruses.push(Virus {
                    x: GRID_LEFT + col as f32 * VIRUS_SPACING_X,
                    y: GRID_TOP + row as f32 * VIRUS_SPACING_Y,
                    alive: true,
                });
            }
        }

        Self {
            player_x: VIEW_W / 2.0 - TILE / 2.0,
            shoot_timer: 0.0,
            bullets: Vec::new(),
            viruses,
            grid_dx: 1.0,
            grid_offset_x: 0.0,
            grid_offset_y: 0.0,
            timer: 0.0,
            score: 0,
            finished: MinigameFinish::Running,
            font: Font::new("font"),
        }
    }

    fn alive_count(&self) -> usize {
        self.viruses.iter().filter(|v| v.alive).count()
    }

    fn rightmost_x(&self) -> f32 {
        self.viruses
            .iter()
            .filter(|v| v.alive)
            .map(|v| v.x + self.grid_offset_x + VIRUS_W)
            .fold(f32::NEG_INFINITY, f32::max)
    }

    fn leftmost_x(&self) -> f32 {
        self.viruses
            .iter()
            .filter(|v| v.alive)
            .map(|v| v.x + self.grid_offset_x)
            .fold(f32::INFINITY, f32::min)
    }

    fn lowest_y(&self) -> f32 {
        self.viruses
            .iter()
            .filter(|v| v.alive)
            .map(|v| v.y + self.grid_offset_y + VIRUS_H)
            .fold(f32::NEG_INFINITY, f32::max)
    }

    fn update_player(&mut self, input: &Input, dt: f32) {
        if input.is_held(KeyCode::KeyA) || input.is_held(KeyCode::ArrowLeft) {
            self.player_x -= PLAYER_SPEED * dt;
        }
        if input.is_held(KeyCode::KeyD) || input.is_held(KeyCode::ArrowRight) {
            self.player_x += PLAYER_SPEED * dt;
        }
        self.player_x = self.player_x.clamp(0.0, VIEW_W - TILE);

        self.shoot_timer -= dt;
        if input.is_held(KeyCode::Space) && self.shoot_timer <= 0.0 {
            self.bullets.push(Bullet {
                x: self.player_x + TILE / 2.0 - BULLET_W / 2.0,
                y: PLAYER_Y,
                active: true,
            });
            self.shoot_timer = SHOOT_COOLDOWN;
        }
    }

    fn update_bullets(&mut self, dt: f32) {
        for bullet in &mut self.bullets {
            if bullet.active {
                bullet.y -= BULLET_SPEED * dt;
                if bullet.y < 0.0 {
                    bullet.active = false;
                }
            }
        }

        // Bullet vs virus collision
        for bullet in &mut self.bullets {
            if !bullet.active {
                continue;
            }
            for virus in &mut self.viruses {
                if !virus.alive {
                    continue;
                }
                let vx = virus.x + self.grid_offset_x;
                let vy = virus.y + self.grid_offset_y;
                if bullet.x < vx + VIRUS_W
                    && bullet.x + BULLET_W > vx
                    && bullet.y < vy + VIRUS_H
                    && bullet.y + BULLET_H > vy
                {
                    bullet.active = false;
                    virus.alive = false;
                    self.score += 10;
                }
            }
        }

        self.bullets.retain(|b| b.active);
    }

    fn update_viruses(&mut self, dt: f32) {
        // Move horizontally
        self.grid_offset_x += self.grid_dx * VIRUS_MOVE_SPEED * dt;

        // Hit right edge -> drop + reverse
        if self.rightmost_x() >= VIEW_W {
            self.grid_dx = -1.0;
            self.grid_offset_y += VIRUS_DROP;
        }
        // Hit left edge -> drop + reverse
        if self.leftmost_x() <= 0.0 {
            self.grid_dx = 1.0;
            self.grid_offset_y += VIRUS_DROP;
        }
    }
}

impl Minigame for VirusMinigame {
    fn update(&mut self, input: &Input, dt: f32) -> MinigameResult {
        if self.finished != MinigameFinish::Running {
            // Wait for Enter/Space on results
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

        self.update_player(input, dt);
        self.update_bullets(dt);
        self.update_viruses(dt);

        // Win: all viruses dead
        if self.alive_count() == 0 {
            self.finished = MinigameFinish::Won;
        }

        // Lose: time up OR viruses reach the player
        if self.timer >= TIME_LIMIT || self.lowest_y() >= PLAYER_Y {
            self.finished = MinigameFinish::Lost;
        }

        MinigameResult::Running
    }

    fn render(&self, renderer: &mut Renderer) {
        let cam_x = renderer.camera.position.x;
        let cam_y = renderer.camera.position.y;

        // Background
        renderer.draw_sprite("body_background", cam_x, cam_y, VIEW_W, VIEW_H);

        // Viruses
        for (i, virus) in self.viruses.iter().enumerate() {
            if !virus.alive {
                continue;
            }
            let vx = cam_x + virus.x + self.grid_offset_x;
            let vy = cam_y + virus.y + self.grid_offset_y;
            let key = format!("virus_{}", i);
            renderer.draw_sprite_keyed(&key, "virus", vx, vy, VIRUS_W, VIRUS_H);
        }

        // Bullets
        for (i, bullet) in self.bullets.iter().enumerate() {
            let key = format!("bullet_{}", i);
            renderer.draw_sprite_keyed(
                &key,
                "debug_red",
                cam_x + bullet.x,
                cam_y + bullet.y,
                BULLET_W,
                BULLET_H,
            );
        }

        // Player
        renderer.draw_sprite(
            "white_bloodcell",
            cam_x + self.player_x,
            cam_y + PLAYER_Y,
            TILE,
            TILE,
        );

        // HUD: timer and score
        let time_left = (TIME_LIMIT - self.timer).max(0.0) as u32;
        let score_text = format!("SCORE {}", self.score);
        let timer_text = format!("TIME {}", time_left);

        self.font.draw_keyed(
            renderer,
            &score_text,
            cam_x + 4.0,
            cam_y + 4.0,
            0.6,
            "hud_score",
        );

        let timer_w = self.font.measure(&timer_text, 0.6);
        self.font.draw_keyed(
            renderer,
            &timer_text,
            cam_x + VIEW_W - timer_w - 4.0,
            cam_y + 4.0,
            0.6,
            "hud_timer",
        );

        // Win / lose overlay
        let cx = cam_x + VIEW_W / 2.0;
        let cy = cam_y + VIEW_H / 2.0;
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
                    "YOU WIN",
                    cx - self.font.measure("YOU WIN", 1.0) / 2.0,
                    cy - 14.0,
                    1.0,
                    "result_title",
                );
                let s = format!("SCORE {}", self.score);
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
                    "PRESS ENTER",
                    cx - self.font.measure("PRESS ENTER", 0.5) / 2.0,
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
                    "YOU LOST",
                    cx - self.font.measure("YOU LOST", 1.0) / 2.0,
                    cy - 14.0,
                    1.0,
                    "result_title",
                );
                self.font.draw_keyed(
                    renderer,
                    "PRESS ENTER",
                    cx - self.font.measure("PRESS ENTER", 0.5) / 2.0,
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
