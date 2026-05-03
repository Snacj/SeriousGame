use crate::engine::{collision::CollisionBox, renderer::Renderer};

pub struct Object {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub object_type: ObjectType,
    pub collision_box: CollisionBox,
}

impl Object {
    pub fn new(x: f32, y: f32, w: f32, h: f32, object_type: ObjectType, collision_box: CollisionBox) -> Self {
        Self {
            x,
            y,
            w,
            h,
            object_type,
            collision_box,
        }
    }

    pub fn object_name(&self) -> &'static str {
        self.object_type.object_name()
    }

    pub fn render(&self, renderer: &mut Renderer) {
        renderer.draw_sprite(self.object_name(), self.x, self.y, self.w, self.h);
    }

    /// Render with a unique batch key so Y-sorting works when multiple
    /// objects share the same texture.
    pub fn render_ordered(&self, renderer: &mut Renderer, order: usize, debug: bool) {
        let key = format!("{}_{}", self.object_name(), order);
        renderer.draw_sprite_keyed(&key, self.object_name(), self.x, self.y, self.w, self.h);

        if debug {
            let cb = &self.collision_box;
            let debug_key = format!("debug_player_{}", order);
            renderer.draw_sprite_keyed(
                &debug_key,
                "debug_red",
                self.x + cb.offset_x,
                self.y + cb.offset_y,
                cb.width,
                cb.height,
            );
        }
    }

    pub fn feet_y(&self) -> f32 {
        self.y + self.h
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum ObjectType {
    House,
}

impl ObjectType {
    pub fn object_name(&self) -> &'static str {
        match self {
            ObjectType::House => "house",
        }
    }
}
