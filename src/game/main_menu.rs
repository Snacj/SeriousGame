use winit::keyboard::KeyCode;

use crate::engine::camera::Camera;
use crate::engine::font::Font;
use crate::engine::input::Input;
use crate::engine::renderer::Renderer;
use crate::game::GameState;
use crate::game::game::MyGame;

pub struct MainMenu {
    camera: Camera,
    font: Font,
    screen_width: f32,
    screen_height: f32,
}

impl MainMenu {
    pub fn new(screen_width: f32, screen_height: f32) -> Self {
        let mut camera = Camera::new(180.0);
        camera.update_aspect_ratio(screen_width, screen_height);
        Self {
            camera,
            font: Font::new("font"),
            screen_width,
            screen_height,
        }
    }

    /// Returns Some(GameState) to transition, None to stay on the menu.
    pub fn update(&mut self, input: &Input) -> Option<GameState> {
        if input.is_just_pressed(KeyCode::Space) || input.is_just_pressed(KeyCode::Enter) {
            return Some(GameState::Playing(MyGame::new(
                self.screen_width,
                self.screen_height,
            )));
        }
        None
    }

    pub fn render(&self, renderer: &mut Renderer) {
        // Pin camera to origin for UI rendering
        renderer.camera.position.x = 0.0;
        renderer.camera.position.y = 0.0;

        let cx = renderer.camera.logical_width / 2.0;
        let cy = renderer.camera.logical_height / 2.0;

        self.font
            .draw_centered(renderer, "FOREST EXPLORER", cx, cy - 30.0, 1.0);
        self.font
            .draw_centered(renderer, "PRESS SPACE TO START", cx, cy + 10.0, 0.8);
    }

    pub fn on_resize(&mut self, w: f32, h: f32) {
        self.screen_width = w;
        self.screen_height = h;
        self.camera.update_aspect_ratio(w, h);
    }
}
