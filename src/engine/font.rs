//! Bitmap font renderer for a 3-row spritesheet:
//!   Row 0: A–M  (13 characters)
//!   Row 1: N–Z  (13 characters)
//!   Row 2: 0–9  (10 characters)

use crate::engine::renderer::Renderer;

const CHAR_SIZE: f32 = 16.0;
const ATLAS_W: f32 = 208.0; // 13 cols × 16px
const ATLAS_H: f32 = 64.0; //  4 rows × 16px

pub struct Font {
    texture_name: String,
    /// Letter spacing in world units (added between characters).
    pub letter_spacing: f32,
}

impl Font {
    pub fn new(texture_name: &str) -> Self {
        Self {
            texture_name: texture_name.to_string(),
            letter_spacing: -8.0,
        }
    }

    pub fn with_spacing(mut self, spacing: f32) -> Self {
        self.letter_spacing = spacing;
        self
    }

    pub fn draw(&self, renderer: &mut Renderer, text: &str, x: f32, y: f32, scale: f32) {
        let char_draw_size = CHAR_SIZE * scale;
        let advance = char_draw_size + self.letter_spacing * scale;

        let mut cursor_x = x;
        for ch in text.chars() {
            if ch == ' ' {
                cursor_x += advance;
                continue;
            }

            if let Some((col, row)) = char_to_atlas(ch) {
                renderer.draw_sprite_frame(
                    &self.texture_name,
                    cursor_x,
                    y,
                    char_draw_size,
                    char_draw_size,
                    col,
                    row,
                    CHAR_SIZE,
                    CHAR_SIZE,
                    ATLAS_W,
                    ATLAS_H,
                );
            }

            cursor_x += advance;
        }
    }

    /// Draw text centered horizontally around `cx`.
    pub fn draw_centered(&self, renderer: &mut Renderer, text: &str, cx: f32, y: f32, scale: f32) {
        let w = self.measure(text, scale);
        self.draw(renderer, text, cx - w / 2.0, y, scale);
    }

    /// Returns the total drawn width of a string in world units at the given scale.
    pub fn measure(&self, text: &str, scale: f32) -> f32 {
        if text.is_empty() {
            return 0.0;
        }
        let advance = CHAR_SIZE * scale + self.letter_spacing * scale;
        // Last character doesn't add spacing after it
        advance * text.len() as f32 - self.letter_spacing * scale
    }
}

fn char_to_atlas(ch: char) -> Option<(usize, usize)> {
    let ch = ch.to_ascii_uppercase();
    match ch {
        'A'..='M' => Some(((ch as usize) - ('A' as usize), 0)),
        'N'..='Z' => Some(((ch as usize) - ('N' as usize), 1)),
        '0'..='9' => Some(((ch as usize) - ('0' as usize), 2)),
        _ => None,
    }
}
