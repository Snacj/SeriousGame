use crate::engine::texture::load_sprites;
use crate::engine::{engine::Engine, input::Input, renderer::Renderer};
use crate::game::dialogue::{DialogueBox, DialogueData};
use crate::game::game::MyGame;
use crate::game::main_menu::MainMenu;

pub mod dialogue;
pub mod game;
pub mod main_menu;
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
}

impl GameState {
    pub fn init(engine: &Engine, renderer: &mut Renderer) -> Self {
        load_sprites(engine, renderer);

        let (sw, sh) = engine.screen_size();
        GameState::MainMenu(MainMenu::new(sw, sh))
    }

    /// Returns Some(next_state) to transition, None to stay.
    pub fn update(&mut self, input: &Input, dt: f32) -> Option<GameState> {
        match self {
            GameState::MainMenu(menu) => menu.update(input),
            GameState::Playing(game) => game.update(input, dt),
            GameState::Paused(game) => game.update_paused(input),
            GameState::Dialogue { game, data, .. } => {
                if input.is_just_pressed(winit::keyboard::KeyCode::Enter) {
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
                // Render the game world underneath the pause overlay
                game.render(renderer);
                Self::render_pause_overlay(renderer);
            }
            GameState::Dialogue {
                game,
                data,
                dialogue_box,
            } => {
                game.render(renderer);
                dialogue_box.render(renderer, data);
            }
        }
    }

    pub fn on_resize(&mut self, w: f32, h: f32) {
        match self {
            GameState::MainMenu(menu) => menu.on_resize(w, h),
            GameState::Playing(game) => game.on_resize(w, h),
            GameState::Paused(game) => game.on_resize(w, h),
            GameState::Dialogue { game, .. } => game.on_resize(w, h),
        }
    }

    fn render_pause_overlay(renderer: &mut Renderer) {
        renderer.draw_sprite(
            "transparent_gray",
            renderer.camera.position.x,
            renderer.camera.position.y,
            renderer.camera.logical_width,
            renderer.camera.logical_height,
        );
    }
}
