//! Bitmap font renderer for a 16x16 glyph spritesheet (13 columns x 4 rows):
//!   Row 0: A-M  (13 glyphs)
//!   Row 1: N-Z  (13 glyphs)
//!   Row 2: 0-9  (10 glyphs)
//!   Row 3: ! . ? ,  AE OE UE  ae oe ue   (punctuation + umlauts, see `char_to_atlas`)
//!
//! Each glyph lives in the top-left corner of its 16x16 cell and is smaller than
//! the cell. We sample exactly the glyph's pixel rectangle (see `glyph_width` /
//! `glyph_height`) from the cell origin and draw it at that size, so the empty
//! part of the cell is never touched. Horizontal spacing is proportional via the
//! per-glyph advance width. For a monospaced look, make the width table constant.

use crate::engine::renderer::Renderer;

/// Size of one atlas cell in pixels (glyphs sit in the top-left of this).
const CHAR_SIZE: f32 = 16.0;
const ATLAS_COLS: f32 = 13.0;
const ATLAS_ROWS: f32 = 4.0;
const ATLAS_W: f32 = CHAR_SIZE * ATLAS_COLS; // 208
const ATLAS_H: f32 = CHAR_SIZE * ATLAS_ROWS; //  64

/// Advance width of a space, in atlas pixels.
const SPACE_WIDTH: f32 = 6.0;
/// Default glyph height in atlas pixels (glyphs are top-aligned in their cell).
const DEFAULT_GLYPH_HEIGHT: f32 = 9.0;
/// Line height as a multiple of the scaled cell size.
const LINE_SPACING: f32 = 1.2;

/// Which batch layer text is drawn into.
#[derive(Clone, Copy)]
enum Layer {
    World,
    Ui,
}

pub struct Font {
    texture_name: &'static str,
    /// Extra tracking added after every glyph, in atlas pixels.
    pub letter_spacing: f32,
}

impl Font {
    pub fn new(texture_name: &'static str) -> Self {
        Self {
            texture_name,
            letter_spacing: 1.0,
        }
    }

    #[allow(dead_code)]
    pub fn with_spacing(mut self, spacing: f32) -> Self {
        self.letter_spacing = spacing;
        self
    }

    pub fn draw(&self, renderer: &mut Renderer, text: &str, x: f32, y: f32, scale: f32) {
        self.draw_impl(renderer, text, x, y, scale, Layer::World, None);
    }

    pub fn draw_ui(&self, renderer: &mut Renderer, text: &str, x: f32, y: f32, scale: f32) {
        self.draw_impl(renderer, text, x, y, scale, Layer::Ui, None);
    }

    pub fn draw_keyed(
        &self,
        renderer: &mut Renderer,
        text: &str,
        x: f32,
        y: f32,
        scale: f32,
        key_prefix: &str,
    ) {
        self.draw_impl(renderer, text, x, y, scale, Layer::World, Some(key_prefix));
    }

    /// Draw text centered horizontally around `cx`.
    pub fn draw_centered(&self, renderer: &mut Renderer, text: &str, cx: f32, y: f32, scale: f32) {
        let w = self.measure(text, scale);
        self.draw(renderer, text, cx - w / 2.0, y, scale);
    }

    /// Draw text centered horizontally around `cx`, onto the UI layer (drawn on top).
    pub fn draw_centered_ui(&self, renderer: &mut Renderer, text: &str, cx: f32, y: f32, scale: f32) {
        let w = self.measure(text, scale);
        self.draw_ui(renderer, text, cx - w / 2.0, y, scale);
    }

    /// Word-wrap `text` and draw it as centered lines on the world layer.
    #[allow(dead_code)]
    pub fn draw_paragraph(
        &self,
        renderer: &mut Renderer,
        text: &str,
        cx: f32,
        top_y: f32,
        scale: f32,
        max_width: f32,
    ) {
        let line_h = self.line_height(scale);
        for (i, line) in self.wrap(text, scale, max_width).iter().enumerate() {
            self.draw_centered(renderer, line, cx, top_y + i as f32 * line_h, scale);
        }
    }

    /// Word-wrap `text` and draw it as centered lines on the UI layer (drawn on top).
    pub fn draw_paragraph_ui(
        &self,
        renderer: &mut Renderer,
        text: &str,
        cx: f32,
        top_y: f32,
        scale: f32,
        max_width: f32,
    ) {
        let line_h = self.line_height(scale);
        for (i, line) in self.wrap(text, scale, max_width).iter().enumerate() {
            self.draw_centered_ui(renderer, line, cx, top_y + i as f32 * line_h, scale);
        }
    }

    /// Total drawn width of a string in world units at the given scale.
    pub fn measure(&self, text: &str, scale: f32) -> f32 {
        let mut width = 0.0;
        let mut drew_any = false;
        for ch in text.chars() {
            let advance = if ch == ' ' {
                SPACE_WIDTH
            } else if char_to_atlas(ch).is_some() {
                glyph_width(ch)
            } else {
                continue; // unsupported glyph: skip, no advance
            };
            width += (advance + self.letter_spacing) * scale;
            drew_any = true;
        }
        if drew_any {
            width -= self.letter_spacing * scale; // no trailing spacing
        }
        width.max(0.0)
    }

    /// Vertical distance between successive wrapped lines at the given scale.
    pub fn line_height(&self, scale: f32) -> f32 {
        CHAR_SIZE * scale * LINE_SPACING
    }

    /// Greedily word-wrap `text` so each line fits within `max_width` at `scale`.
    pub fn wrap(&self, text: &str, scale: f32, max_width: f32) -> Vec<String> {
        let mut lines: Vec<String> = Vec::new();
        let mut current = String::new();

        for word in text.split_whitespace() {
            let candidate = if current.is_empty() {
                word.to_string()
            } else {
                format!("{current} {word}")
            };

            if !current.is_empty() && self.measure(&candidate, scale) > max_width {
                lines.push(current);
                current = word.to_string();
            } else {
                current = candidate;
            }
        }

        if !current.is_empty() {
            lines.push(current);
        }
        lines
    }

    fn draw_impl(
        &self,
        renderer: &mut Renderer,
        text: &str,
        x: f32,
        y: f32,
        scale: f32,
        layer: Layer,
        key_prefix: Option<&str>,
    ) {
        let mut cursor_x = x;

        for (i, ch) in text.chars().enumerate() {
            if ch == ' ' {
                cursor_x += (SPACE_WIDTH + self.letter_spacing) * scale;
                continue;
            }

            let Some((col, row)) = char_to_atlas(ch) else {
                continue; // unsupported glyph: skip entirely
            };

            // Sample the glyph's pixel rect from the top-left of its cell.
            let gw = glyph_width(ch);
            let gh = glyph_height(ch);
            let src_x = col as f32 * CHAR_SIZE;
            let src_y = row as f32 * CHAR_SIZE;
            let draw_w = gw * scale;
            let draw_h = gh * scale;

            match (layer, key_prefix) {
                (Layer::World, None) => renderer.draw_sprite_sub(
                    self.texture_name, cursor_x, y, draw_w, draw_h,
                    src_x, src_y, gw, gh, ATLAS_W, ATLAS_H,
                ),
                (Layer::Ui, None) => renderer.draw_sprite_sub_ui(
                    self.texture_name, cursor_x, y, draw_w, draw_h,
                    src_x, src_y, gw, gh, ATLAS_W, ATLAS_H,
                ),
                (Layer::World, Some(prefix)) => {
                    let key = format!("{prefix}_{i}");
                    renderer.draw_sprite_sub_keyed(
                        &key, self.texture_name, cursor_x, y, draw_w, draw_h,
                        src_x, src_y, gw, gh, ATLAS_W, ATLAS_H,
                    );
                }
                (Layer::Ui, Some(prefix)) => {
                    let key = format!("{prefix}_{i}");
                    renderer.draw_sprite_sub_ui_keyed(
                        &key, self.texture_name, cursor_x, y, draw_w, draw_h,
                        src_x, src_y, gw, gh, ATLAS_W, ATLAS_H,
                    );
                }
            }

            cursor_x += (gw + self.letter_spacing) * scale;
        }
    }
}

/// Atlas cell (column, row) for a character, or `None` if unsupported.
fn char_to_atlas(ch: char) -> Option<(usize, usize)> {
    // Row 3: punctuation and umlauts (upper- and lower-case fold to one glyph).
    match ch {
        '!' => return Some((0, 3)),
        '.' => return Some((1, 3)),
        '?' => return Some((2, 3)),
        ',' => return Some((3, 3)),
        'Ä' | 'ä' => return Some((4, 3)),
        'Ö' | 'ö' => return Some((5, 3)),
        'Ü' | 'ü' => return Some((6, 3)),
        _ => {}
    }

    // Letters are single-case: fold lowercase onto the uppercase glyph.
    let ch = ch.to_ascii_uppercase();
    match ch {
        'A'..='M' => Some(((ch as usize) - ('A' as usize), 0)),
        'N'..='Z' => Some(((ch as usize) - ('N' as usize), 1)),
        '0'..='9' => Some(((ch as usize) - ('0' as usize), 2)),
        _ => None,
    }
}

/// Per-glyph advance width in atlas pixels (out of `CHAR_SIZE`).
/// Tune these to match the actual artwork in `font2.png`.
fn glyph_width(ch: char) -> f32 {
    // Umlauts share the width of their base vowel.
    let ch = match ch {
        'ä' | 'Ä' => 'A',
        'ö' | 'Ö' => 'O',
        'ü' | 'Ü' => 'U',
        other => other.to_ascii_uppercase(),
    };

    match ch {
        'I' => 5.0,
        'J' => 4.0,
        'W' => 8.0,
        'X' | 'M' => 7.0,
        'A'..='Z' => 6.0,
        '0'..='9' => 7.0,
        '!' | '.' | ',' => 3.0,
        '?' => 9.0,
        _ => 6.0,
    }
}

/// Per-glyph height in atlas pixels (sampled from the top of the cell).
/// All glyphs default to 9px; add per-glyph overrides here as needed.
fn glyph_height(ch: char) -> f32 {
    let _ = ch;

    match ch {
        'Ä' => 12.0,
        'Ü' => 12.0,
        'Ö' => 12.0,
        _ => DEFAULT_GLYPH_HEIGHT,
    }
}
