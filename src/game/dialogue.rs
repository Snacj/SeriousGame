use crate::engine::{font::Font, renderer::Renderer};

#[derive(Clone)]
pub struct DialogueData {
    pub title: &'static str,
    pub lines: &'static [&'static str],
    pub minigame: Option<MinigameTrigger>,
}

#[derive(Debug, Clone)]
pub enum MinigameTrigger {
    CatchVirus,
    SortFood,
    VaccineTiming,
    DeliverOxygen,
}

impl DialogueData {
    pub const fn new(title: &'static str, lines: &'static [&'static str]) -> Self {
        Self {
            title,
            lines,
            minigame: None,
        }
    }

    pub const fn with_minigame(mut self, trigger: MinigameTrigger) -> Self {
        self.minigame = Some(trigger);
        self
    }
}

pub struct DialogueBox {
    font: Font,
}

impl DialogueBox {
    pub fn new() -> Self {
        Self {
            font: Font::new("font"),
        }
    }

    /// Draw the dialogue box
    pub fn render(&self, renderer: &mut Renderer, data: &DialogueData) {
        let cam_x = renderer.camera.position.x;
        let cam_y = renderer.camera.position.y;
        let cam_w = renderer.camera.logical_width;
        let cam_h = renderer.camera.logical_height;

        // Box dimensions in world units
        let padding = 6.0;
        let box_w = cam_w - padding * 2.0;
        let line_h = 10.0; // space between lines at scale 0.6
        let num_lines = data.lines.len() as f32;
        let box_h = 14.0              // title area
            + (num_lines * line_h)    // body lines
            + 12.0;                   // hint line + bottom padding

        // Pin to bottom of screen
        let box_x = cam_x + padding;
        let box_y = cam_y + cam_h - box_h - padding;

        renderer.draw_sprite("ui_panel", box_x, box_y, box_w, box_h);

        // Title
        self.font.draw(
            renderer,
            data.title,
            box_x + padding,
            box_y + padding,
            0.7,
        );

        // Body lines
        for (i, line) in data.lines.iter().enumerate() {
            self.font.draw(
                renderer,
                line,
                box_x + padding,
                box_y + 14.0 + i as f32 * line_h,
                0.6,
            );
        }

        // Hint
        let hint = if data.minigame.is_some() {
            "SPACE START MINIGAME"
        } else {
            "ENTER TO CLOSE"
        };
        let hint_w = self.font.measure(hint, 0.5);
        self.font.draw(
            renderer,
            hint,
            box_x + box_w - hint_w - padding,
            box_y + box_h - 8.0,
            0.5,
        );
    }
}

