use crate::engine::{input::Input, renderer::Renderer};

pub enum MinigameResult {
    Running,
    Won { score: u32 },
    Lost,
}

pub trait Minigame {
    fn update(&mut self, input: &Input, dt: f32) -> MinigameResult;
    fn render(&self, renderer: &mut Renderer);
    fn on_resize(&mut self, w: f32, h: f32);
}

pub struct MinigameOutcome {
    pub won: bool,
    pub score: u32,

    pub title: &'static str,

    pub fact: &'static str,
}

pub struct PlaceholderMinigame {
    pub title: &'static str,
    timer: f32,
}

impl PlaceholderMinigame {
    pub fn new(title: &'static str) -> Self {
        Self { title, timer: 0.0 }
    }
}

impl Minigame for PlaceholderMinigame {
    fn update(&mut self, input: &Input, dt: f32) -> MinigameResult {
        self.timer += dt;

        if input.is_just_pressed(winit::keyboard::KeyCode::Space) {
            return MinigameResult::Won { score: 100 };
        }

        if input.is_just_pressed(winit::keyboard::KeyCode::Escape) {
            return MinigameResult::Lost;
        }

        MinigameResult::Running
    }

    fn render(&self, renderer: &mut Renderer) {
        let x = renderer.camera.position.x + 8.0;
        let y = renderer.camera.position.y + 8.0;

        let _ = (x, y);
    }

    fn on_resize(&mut self, _w: f32, _h: f32) {}
}
