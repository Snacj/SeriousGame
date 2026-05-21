use winit::keyboard::KeyCode;

use crate::engine::texture::load_sprites;
use crate::engine::{engine::Engine, input::Input, renderer::Renderer};
use crate::game::dialogue::{DialogueBox, DialogueData, MinigameTrigger};
use crate::game::game::MyGame;
use crate::game::main_menu::MainMenu;
use crate::game::minigame::{Minigame, MinigameOutcome, MinigameResult, PlaceholderMinigame};

pub mod dialogue;
pub mod game;
pub mod main_menu;
pub mod minigame;
pub mod minigame_virus;
pub mod object;
pub mod player;
pub mod tile;

pub struct GameContext {
    pub state: GameState,
    pub health: f32,
}

impl GameContext {
    pub fn init(engine: &Engine, renderer: &mut Renderer) -> Self {
        Self {
            state: GameState::init(engine, renderer),
            health: 0.3,
        }
    }

    pub fn update(&mut self, input: &Input, dt: f32) {
        if let Some((next, delta)) = self.state.update(input, dt) {
            self.state = next;
            self.health = (self.health + delta).clamp(0.0, 1.0);
        }

        if input.is_just_pressed(KeyCode::F12) {
            self.health = 1.0;
        }

        if self.health >= 1.0 && !matches!(self.state, GameState::Won) {
            self.state = GameState::Won;
        }
    }

    pub fn render(&self, renderer: &mut Renderer) {
        self.state.render(renderer);
        if !self.state.is_main_menu() && !matches!(self.state, GameState::Won) {
            Self::render_health_bar(renderer, self.health);
        }
    }

    pub fn on_resize(&mut self, w: f32, h: f32) {
        self.state.on_resize(w, h);
    }

    fn render_health_bar(renderer: &mut Renderer, health: f32) {
        let cam_x = renderer.camera.position.x;
        let cam_y = renderer.camera.position.y;
        let bar_x = cam_x + 6.0;
        let bar_y = cam_y + 12.0;
        let bar_w = 80.0;
        let bar_h = 5.0;
        renderer.draw_sprite_ui("debug_red", bar_x, bar_y, bar_w, bar_h);
        renderer.draw_sprite_ui("health_green", bar_x, bar_y, bar_w * health, bar_h);
    }
}

pub enum GameState {
    MainMenu(MainMenu),
    Playing(MyGame),
    Paused(MyGame),

    Dialogue {
        game: MyGame,
        data: DialogueData,
        dialogue_box: DialogueBox,
    },

    /// A minigame is running. Overworld suspended.
    Minigame {
        game: MyGame,
        minigame: Box<dyn Minigame>,
        outcome_title: &'static str,
        outcome_fact: &'static str,
        object_index: Option<usize>,
    },

    /// Win/lose screen shown after a minigame ends.
    Results {
        game: MyGame,
        outcome: MinigameOutcome,
    },

    Won,
}

impl GameState {
    pub fn init(engine: &Engine, renderer: &mut Renderer) -> Self {
        load_sprites(engine, renderer);
        let (sw, sh) = engine.screen_size();
        GameState::MainMenu(MainMenu::new(sw, sh))
    }

    pub fn update(&mut self, input: &Input, dt: f32) -> Option<(GameState, f32)> {
        match self {
            GameState::MainMenu(menu) => menu.update(input, dt).map(|s| (s, 0.0)),
            GameState::Playing(game) => game.update(input, dt).map(|s| (s, 0.0)),
            GameState::Paused(game) => game.update_paused(input).map(|s| (s, 0.0)),

            GameState::Dialogue { game, data, .. } => {
                if input.is_just_pressed(KeyCode::Enter) {
                    let next = if let Some(trigger) = data.minigame {
                        let (minigame, title, fact) = make_minigame(trigger);
                        let game = std::mem::replace(game, MyGame::new(0.0, 0.0));
                        let object_index = game.objects.iter().position(|obj| {
                            obj.interaction
                                .as_ref()
                                .and_then(|i| i.minigame)
                                .map(|t| {
                                    std::mem::discriminant(&t) == std::mem::discriminant(&trigger)
                                })
                                .unwrap_or(false)
                        });
                        GameState::Minigame {
                            game,
                            minigame,
                            outcome_title: title,
                            outcome_fact: fact,
                            object_index,
                        }
                    } else {
                        let game = std::mem::replace(game, MyGame::new(0.0, 0.0));
                        GameState::Playing(game)
                    };
                    return Some((next, 0.0));
                }
                None
            }

            GameState::Minigame {
                game,
                minigame,
                outcome_title,
                outcome_fact,
                object_index,
            } => match minigame.update(input, dt) {
                MinigameResult::Running => None,
                MinigameResult::Won { score } => {
                    let outcome = MinigameOutcome {
                        won: true,
                        score,
                        title: outcome_title,
                        fact: outcome_fact,
                        object_index: *object_index,
                    };
                    let game = std::mem::replace(game, MyGame::new(0.0, 0.0));
                    Some((GameState::Results { game, outcome }, 0.0))
                }
                MinigameResult::Lost => {
                    let outcome = MinigameOutcome {
                        won: false,
                        score: 0,
                        title: outcome_title,
                        fact: outcome_fact,
                        object_index: *object_index,
                    };
                    let game = std::mem::replace(game, MyGame::new(0.0, 0.0));
                    Some((GameState::Results { game, outcome }, 0.0))
                }
            },

            GameState::Results { game, outcome } => {
                if input.is_just_pressed(KeyCode::Enter) || input.is_just_pressed(KeyCode::Space) {
                    let delta = if outcome.won { 0.2 } else { -0.1 };

                    if outcome.won {
                        if let Some(idx) = outcome.object_index {
                            game.objects[idx].completed = true;
                            game.rebuild_collision_map();
                        }
                    }

                    let game = std::mem::replace(game, MyGame::new(0.0, 0.0));
                    return Some((GameState::Playing(game), delta));
                }
                None
            }

            GameState::Won => {
                if input.is_just_pressed(KeyCode::Enter) || input.is_just_pressed(KeyCode::Space) {
                    return Some((GameState::MainMenu(MainMenu::new(0.0, 0.0)), 0.0));
                }
                None
            }
        }
    }

    pub fn render(&self, renderer: &mut Renderer) {
        match self {
            GameState::MainMenu(menu) => menu.render(renderer),
            GameState::Playing(game) => game.render(renderer, true),
            GameState::Paused(game) => {
                game.render(renderer, false);
                Self::render_overlay(renderer, "transparent_gray");
            }
            GameState::Dialogue {
                game,
                data,
                dialogue_box,
            } => {
                game.render(renderer, false);
                dialogue_box.render(renderer, data);
            }
            GameState::Minigame { minigame, .. } => {
                minigame.render(renderer);
            }
            GameState::Results { game, outcome } => {
                game.render(renderer, false);
                Self::render_overlay(renderer, "transparent_gray");

                let cam_x = renderer.camera.position.x;
                let cam_y = renderer.camera.position.y;
                let cam_w = renderer.camera.logical_width;
                let cam_h = renderer.camera.logical_height;
                let cx = cam_x + cam_w / 2.0;
                let cy = cam_y + cam_h / 2.0;

                let font = crate::engine::font::Font::new("font");

                let title = if outcome.won { "YOU WIN" } else { "YOU LOST" };
                font.draw_ui(
                    renderer,
                    title,
                    cx - font.measure(title, 1.0) / 2.0,
                    cy - 20.0,
                    1.0,
                );

                font.draw_ui(
                    renderer,
                    outcome.fact,
                    cx - font.measure(outcome.fact, 0.5) / 2.0,
                    cy,
                    0.5,
                );

                font.draw_ui(
                    renderer,
                    "PRESS ENTER TO CONTINUE",
                    cx - font.measure("PRESS ENTER TO CONTINUE", 0.5) / 2.0,
                    cy + 16.0,
                    0.5,
                );
            }
            GameState::Won => {
                let cam_x = renderer.camera.position.x;
                let cam_y = renderer.camera.position.y;
                let cam_w = renderer.camera.logical_width;
                let cam_h = renderer.camera.logical_height;
                let cx = cam_x + cam_w / 2.0;
                let cy = cam_y + cam_h / 2.0;

                renderer.draw_sprite_ui("transparent_gray", cam_x, cam_y, cam_w, cam_h);
                let font = crate::engine::font::Font::new("font");
                let text = "BODY HEALED";
                font.draw_ui(
                    renderer,
                    text,
                    cx - font.measure(text, 1.0) / 2.0,
                    cy - 20.0,
                    1.0,
                );
                let text = "THE PATIENT RECOVERED";
                font.draw_ui(renderer, text, cx - font.measure(text, 0.5) / 2.0, cy, 0.5);
                let text = "We hope you enjoyed the Game!";
                font.draw_ui(
                    renderer,
                    text,
                    cx - font.measure(text, 0.5) / 2.0,
                    cy + 16.0,
                    0.5,
                );
            }
        }
    }

    pub fn on_resize(&mut self, w: f32, h: f32) {
        match self {
            GameState::MainMenu(menu) => menu.on_resize(w, h),
            GameState::Playing(game) => game.on_resize(w, h),
            GameState::Paused(game) => game.on_resize(w, h),
            GameState::Dialogue { game, .. } => game.on_resize(w, h),
            GameState::Minigame { game, minigame, .. } => {
                game.on_resize(w, h);
                minigame.on_resize(w, h);
            }
            GameState::Results { game, .. } => game.on_resize(w, h),
            GameState::Won => {}
        }
    }

    fn render_overlay(renderer: &mut Renderer, texture: &str) {
        renderer.draw_sprite(
            texture,
            renderer.camera.position.x,
            renderer.camera.position.y,
            renderer.camera.logical_width,
            renderer.camera.logical_height,
        );
    }

    pub fn is_main_menu(&self) -> bool {
        matches!(self, GameState::MainMenu(_))
    }
}

fn make_minigame(trigger: MinigameTrigger) -> (Box<dyn Minigame>, &'static str, &'static str) {
    match trigger {
        MinigameTrigger::CatchVirus => (
            Box::new(minigame_virus::VirusMinigame::new()),
            "IMMUNE SYSTEM",
            "White blood cells identify and destroy viruses.",
        ),
        MinigameTrigger::SortFood => (
            Box::new(PlaceholderMinigame::new("SORT THE FOOD")),
            "NUTRITION",
            "A balanced diet gives your body the fuel it needs.",
        ),
        MinigameTrigger::VaccineTiming => (
            Box::new(PlaceholderMinigame::new("VACCINE TIMING")),
            "VACCINES",
            "Vaccines train your immune system before infection.",
        ),
        MinigameTrigger::DeliverOxygen => (
            Box::new(PlaceholderMinigame::new("DELIVER OXYGEN")),
            "RESPIRATORY SYSTEM",
            "Red blood cells carry oxygen to every cell in the body.",
        ),
    }
}
