pub struct Object {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub object_type: ObjectType,
}

impl Object {
    pub fn new(x: f32, y: f32, w: f32, h: f32, object_type: ObjectType) -> Self {
        Self {
            x,
            y,
            w,
            h,
            object_type,
        }
    }

    pub fn object_name(&self) -> &'static str {
        self.object_type.object_name()
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
