use std::collections::HashSet;
use winit::keyboard::KeyCode;

/// Tracks which keys are held, just pressed, or just released.
/// Call `begin_frame()` at the start of each frame to reset per-frame state.
pub struct Input {
    held: HashSet<KeyCode>,
    just_pressed: HashSet<KeyCode>,
    just_released: HashSet<KeyCode>,
}

impl Input {
    pub fn new() -> Self {
        Self {
            held: HashSet::new(),
            just_pressed: HashSet::new(),
            just_released: HashSet::new(),
        }
    }

    /// Call at the start of each frame before processing events.
    pub fn begin_frame(&mut self) {
        self.just_pressed.clear();
        self.just_released.clear();
    }

    /// Feed a winit key event into the tracker.
    pub fn handle_key(&mut self, key: KeyCode, pressed: bool) {
        if pressed {
            if self.held.insert(key) {
                self.just_pressed.insert(key);
            }
        } else if self.held.remove(&key) {
            self.just_released.insert(key);
        }
    }

    /// True every frame the key is held down.
    pub fn is_held(&self, key: KeyCode) -> bool {
        self.held.contains(&key)
    }

    /// True only on the first frame the key goes down.
    pub fn is_just_pressed(&self, key: KeyCode) -> bool {
        self.just_pressed.contains(&key)
    }

    /// True only on the frame the key is released.
    pub fn is_just_released(&self, key: KeyCode) -> bool {
        self.just_released.contains(&key)
    }
}
