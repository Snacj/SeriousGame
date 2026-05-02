pub struct Timer {
    length: f32,
    time: f32,
}

impl Timer {
    pub fn new(length: f32) -> Self {
        Self { length, time: 0.0 }
    }

    pub fn step(&mut self, dt: f32) -> bool {
        self.time += dt;
        if self.time >= self.length {
            self.time -= self.length;
            true
        } else {
            false
        }
    }

    pub fn reset(&mut self) {
        self.time = 0.0;
    }

    pub fn progress(&self) -> f32 {
        self.time / self.length
    }
}

pub struct Animation {
    timer: Timer,
    pub frame_count: usize,
    pub atlas_row: usize,
    pub one_shot: bool,
    finished: bool,
}

impl Animation {
    pub fn new(frame_count: usize, duration: f32, atlas_row: usize) -> Self {
        Self {
            timer: Timer::new(duration),
            frame_count,
            atlas_row,
            one_shot: false,
            finished: false,
        }
    }

    pub fn one_shot(mut self) -> Self {
        self.one_shot = true;
        self
    }

    pub fn step(&mut self, dt: f32) {
        if self.finished {
            return;
        }
        if self.timer.step(dt) && self.one_shot {
            self.finished = true;
        }
    }

    pub fn current_frame(&self) -> usize {
        if self.finished {
            return self.frame_count - 1;
        }
        let progress = self.timer.progress().min(0.9999);
        (progress * self.frame_count as f32) as usize
    }

    pub fn is_finished(&self) -> bool {
        self.finished
    }

    pub fn reset(&mut self) {
        self.timer.reset();
        self.finished = false;
    }
}

pub struct AnimationSet {
    animations: Vec<(String, Animation)>,
    current: usize,
}

impl AnimationSet {
    pub fn new() -> Self {
        Self {
            animations: Vec::new(),
            current: 0,
        }
    }

    pub fn add(mut self, name: &str, animation: Animation) -> Self {
        self.animations.push((name.to_string(), animation));
        self
    }

    pub fn play(&mut self, name: &str) {
        let idx = self.animations.iter().position(|(n, _)| n == name);
        if let Some(idx) = idx {
            if idx != self.current {
                self.animations[idx].1.reset();
                self.current = idx;
            }
        }
    }

    pub fn step(&mut self, dt: f32) {
        if let Some((_, anim)) = self.animations.get_mut(self.current) {
            anim.step(dt);
        }
    }

    pub fn current_frame(&self) -> (usize, usize) {
        if let Some((_, anim)) = self.animations.get(self.current) {
            (anim.atlas_row, anim.current_frame())
        } else {
            (0, 0)
        }
    }

    pub fn current_name(&self) -> &str {
        self.animations
            .get(self.current)
            .map(|(n, _)| n.as_str())
            .unwrap_or("")
    }

    pub fn is_finished(&self) -> bool {
        self.animations
            .get(self.current)
            .map(|(_, a)| a.is_finished())
            .unwrap_or(true)
    }
}
