#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Rect {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self { x, y, w, h }
    }

    pub fn overlaps(&self, other: &Rect) -> bool {
        self.x < other.x + other.w
            && self.x + self.w > other.x
            && self.y < other.y + other.h
            && self.y + self.h > other.y
    }

    pub fn resolve(&self, other: &Rect) -> (f32, f32) {
        let overlap_x = (self.x + self.w).min(other.x + other.w)
            - self.x.max(other.x);
        let overlap_y = (self.y + self.h).min(other.y + other.h)
            - self.y.max(other.y);

        if overlap_x < overlap_y {
            let push = if self.x < other.x { -overlap_x } else { overlap_x };
            (push, 0.0)
        } else {
            let push = if self.y < other.y { -overlap_y } else { overlap_y };
            (0.0, push)
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CollisionBox {
    pub offset_x: f32,
    pub offset_y: f32,
    pub width: f32,
    pub height: f32,
}

impl CollisionBox {
    pub fn new(offset_x: f32, offset_y: f32, width: f32, height: f32) -> Self {
        Self { offset_x, offset_y, width, height }
    }

    /// Compute the world-space Rect for an entity at (x, y).
    pub fn world_rect(&self, x: f32, y: f32) -> Rect {
        Rect::new(
            x + self.offset_x,
            y + self.offset_y,
            self.width,
            self.height,
        )
    }
}

pub struct CollisionWorld {
    pub statics: Vec<Rect>,
}

impl CollisionWorld {
    pub fn new() -> Self {
        Self { statics: Vec::new() }
    }

    pub fn add(&mut self, rect: Rect) {
        self.statics.push(rect);
    }

    pub fn clear(&mut self) {
        self.statics.clear();
    }

    pub fn resolve_player(
        &self,
        player_box: &CollisionBox,
        mut x: f32,
        mut y: f32,
    ) -> (f32, f32) {
        for _ in 0..3 {
            let player_rect = player_box.world_rect(x, y);
            let mut moved = false;

            for static_rect in &self.statics {
                let pr = player_box.world_rect(x, y);
                if pr.overlaps(static_rect) {
                    let (px, py) = pr.resolve(static_rect);
                    x += px;
                    y += py;
                    moved = true;
                }
            }

            if !moved {
                break;
            }
            let _ = player_rect;
        }
        (x, y)
    }
}
