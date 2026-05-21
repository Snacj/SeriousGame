use crate::engine::{font::Font, renderer::Renderer};

#[derive(Clone)]
pub struct DialogueData {
    pub title: &'static str,
    pub lines: &'static [&'static str],
    pub minigame: Option<MinigameTrigger>,
}

#[allow(unused)]
#[derive(Debug, Clone, Copy)]
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
        let line_h = 8.0; // space between lines at scale 0.6
        let num_lines = data.lines.len() as f32;
        let title_h = 22.0;
        let box_h = title_h              // title area
            + (num_lines * line_h)    // body lines
            + 12.0; // hint line + bottom padding

        // Pin to bottom of screen
        let box_x = cam_x + padding;
        let box_y = cam_y + cam_h - box_h - padding;

        draw_9slice(
            renderer,
            "dialogue_background",
            box_x,
            box_y,
            box_w,
            box_h,
            24.0,
            24.0,
            8.0,
        );

        // Title
        self.font
            .draw_ui(renderer, data.title, box_x + padding, box_y + padding, 0.8);

        // Body lines
        for (i, line) in data.lines.iter().enumerate() {
            self.font.draw_ui(
                renderer,
                line,
                box_x + padding,
                box_y + title_h + i as f32 * line_h,
                0.6,
            );
        }

        // Hint
        let hint = if data.minigame.is_some() {
            "ENTER TO START MINIGAME"
        } else {
            "ENTER TO CLOSE"
        };
        let hint_w = self.font.measure(hint, 0.5);
        self.font.draw_ui(
            renderer,
            hint,
            box_x + box_w - hint_w - padding,
            box_y + box_h - 8.0,
            0.5,
        );
    }
}

pub fn draw_9slice(
    renderer: &mut Renderer,
    texture: &str,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    tex_w: f32,
    tex_h: f32,
    corner: f32,
) {
    // Corners don't stretch
    let c = corner;

    // Top-left corner
    renderer.draw_sprite_frame_ui(texture, x, y, c, c, 0, 0, c, c, tex_w, tex_h);
    // Top-right corner
    renderer.draw_sprite_frame_ui(texture, x + w - c, y, c, c, 2, 0, c, c, tex_w, tex_h);
    // Bottom-left corner
    renderer.draw_sprite_frame_ui(texture, x, y + h - c, c, c, 0, 2, c, c, tex_w, tex_h);
    // Bottom-right corner
    renderer.draw_sprite_frame_ui(
        texture,
        x + w - c,
        y + h - c,
        c,
        c,
        2,
        2,
        c,
        c,
        tex_w,
        tex_h,
    );

    // Top edge stretches horizontally
    renderer.draw_sprite_frame_ui(texture, x + c, y, w - c * 2.0, c, 1, 0, c, c, tex_w, tex_h);
    // Bottom edge stretches horizontally
    renderer.draw_sprite_frame_ui(
        texture,
        x + c,
        y + h - c,
        w - c * 2.0,
        c,
        1,
        2,
        c,
        c,
        tex_w,
        tex_h,
    );
    // Left edge stretches vertically
    renderer.draw_sprite_frame_ui(texture, x, y + c, c, h - c * 2.0, 0, 1, c, c, tex_w, tex_h);
    // Right edge stretches vertically
    renderer.draw_sprite_frame_ui(
        texture,
        x + w - c,
        y + c,
        c,
        h - c * 2.0,
        2,
        1,
        c,
        c,
        tex_w,
        tex_h,
    );

    // Center stretches both ways
    renderer.draw_sprite_frame_ui(
        texture,
        x + c,
        y + c,
        w - c * 2.0,
        h - c * 2.0,
        1,
        1,
        c,
        c,
        tex_w,
        tex_h,
    );
}
