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
pub mod object;
pub mod player;
pub mod tile;

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
    },

    /// Win/lose screen shown after a minigame ends.
    Results {
        game: MyGame,
        outcome: MinigameOutcome,
    },
}

impl GameState {
    pub fn init(engine: &Engine, renderer: &mut Renderer) -> Self {
        load_sprites(engine, renderer);
        let (sw, sh) = engine.screen_size();
        GameState::MainMenu(MainMenu::new(sw, sh))
    }

    pub fn update(&mut self, input: &Input, dt: f32) -> Option<GameState> {
        match self {
            GameState::MainMenu(menu) => menu.update(input),
            GameState::Playing(game) => game.update(input, dt),
            GameState::Paused(game) => game.update_paused(input),

            GameState::Dialogue { game, data, .. } => {
                if input.is_just_pressed(KeyCode::Enter) {
                    let next = if let Some(trigger) = data.minigame {
                        let (minigame, title, fact) = make_minigame(trigger);
                        let game = std::mem::replace(game, MyGame::new(0.0, 0.0));
                        GameState::Minigame { game, minigame, outcome_title: title, outcome_fact: fact }
                    } else {
                        let game = std::mem::replace(game, MyGame::new(0.0, 0.0));
                        GameState::Playing(game)
                    };
                    return Some(next);
                }
                None
            }

            GameState::Minigame { game, minigame, outcome_title, outcome_fact } => {
                match minigame.update(input, dt) {
                    MinigameResult::Running => None,
                    MinigameResult::Won { score } => {
                        let outcome = MinigameOutcome {
                            won: true,
                            score,
                            title: outcome_title,
                            fact: outcome_fact,
                        };
                        let game = std::mem::replace(game, MyGame::new(0.0, 0.0));
                        Some(GameState::Results { game, outcome })
                    }
                    MinigameResult::Lost => {
                        let outcome = MinigameOutcome {
                            won: false,
                            score: 0,
                            title: outcome_title,
                            fact: outcome_fact,
                        };
                        let game = std::mem::replace(game, MyGame::new(0.0, 0.0));
                        Some(GameState::Results { game, outcome })
                    }
                }
            }

            GameState::Results { game, .. } => {
                if input.is_just_pressed(KeyCode::Enter) || input.is_just_pressed(KeyCode::Space) {
                    let game = std::mem::replace(game, MyGame::new(0.0, 0.0));
                    return Some(GameState::Playing(game));
                }
                None
            }
        }
    }

    pub fn render(&self, renderer: &mut Renderer) {
        match self {
            GameState::MainMenu(menu) => menu.render(renderer),
            GameState::Playing(game) => game.render(renderer),
            GameState::Paused(game) => {
                game.render(renderer);
                Self::render_overlay(renderer, "transparent_gray");
            }
            GameState::Dialogue { game, data, dialogue_box } => {
                game.render(renderer);
                dialogue_box.render(renderer, data);
            }
            GameState::Minigame { minigame, .. } => {
                minigame.render(renderer);
            }
            GameState::Results { game, outcome } => {
                game.render(renderer);
                Self::render_overlay(renderer, "transparent_gray");
                // TODO: draw outcome.title, outcome.fact, win/lose with font
                let _ = outcome;
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
}

fn make_minigame(trigger: MinigameTrigger) -> (Box<dyn Minigame>, &'static str, &'static str) {
    match trigger {
        MinigameTrigger::CatchVirus => (
            Box::new(PlaceholderMinigame::new("CATCH THE VIRUS")),
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
