use crate::engine::collision::CollisionBox;
use crate::engine::renderer::Renderer;
use crate::game::dialogue::{DialogueData, MinigameTrigger};

fn interaction_for(object_type: ObjectType) -> Option<DialogueData> {
    match object_type {
        ObjectType::House => Some(
            DialogueData::new(
                "HOUSE",
                &[
                    "This is testing",
                    "Next Line.",
                    "Love if this works!!",
                ],
            )
        ),
        ObjectType::RedBloodCell => Some(
            DialogueData::new(
                "RED BLOOD CELL",
                &[
                    "I carry oxygen through",
                    "your bloodstream.",
                    "Without me cells die!",
                ],
            )
        ),
        ObjectType::VirusStation => Some(
            DialogueData::new(
                "VIRUS DETECTED",
                &[
                    "A virus is attacking!",
                    "Help the immune system",
                    "fight it off.",
                ],
            )
            .with_minigame(MinigameTrigger::CatchVirus),
        ),
        ObjectType::FoodStation => Some(
            DialogueData::new(
                "NUTRITION LAB",
                &[
                    "What you eat matters.",
                    "Sort healthy food from",
                    "junk to heal the body.",
                ],
            )
            .with_minigame(MinigameTrigger::SortFood),
        ),
        ObjectType::VaccineStation => Some(
            DialogueData::new(
                "VACCINE STATION",
                &[
                    "Vaccines train your",
                    "immune system to fight",
                    "disease before it hits.",
                ],
            )
            .with_minigame(MinigameTrigger::VaccineTiming),
        ),
    }
}

pub struct Object {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub object_type: ObjectType,
    pub collision_box: CollisionBox,
    pub interaction: Option<DialogueData>,
}

impl Object {
    pub fn new(
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        object_type: ObjectType,
        collision_box: CollisionBox,
    ) -> Self {
        let interaction = interaction_for(object_type);
        Self {
            x,
            y,
            w,
            h,
            object_type,
            collision_box,
            interaction,
        }
    }

    pub fn object_name(&self) -> &'static str {
        self.object_type.object_name()
    }

    /// Returns true if the player is close enough to interact.
    pub fn is_near(&self, player_x: f32, player_y: f32, range: f32) -> bool {
        let cx = self.x + self.w / 2.0;
        let cy = self.y + self.h / 2.0;
        let dx = player_x - cx;
        let dy = player_y - cy;
        (dx * dx + dy * dy).sqrt() < range
    }

    pub fn render(&self, renderer: &mut Renderer) {
        renderer.draw_sprite(self.object_name(), self.x, self.y, self.w, self.h);
    }

    pub fn render_ordered(&self, renderer: &mut Renderer, order: usize, debug: bool) {
        let key = format!("{}_{}", self.object_name(), order);
        renderer.draw_sprite_keyed(&key, self.object_name(), self.x, self.y, self.w, self.h);

        if debug {
            let cb = &self.collision_box;
            let debug_key = format!("debug_obj_{}", order);
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
    RedBloodCell,
    VirusStation,
    FoodStation,
    VaccineStation,
}

impl ObjectType {
    pub fn object_name(&self) -> &'static str {
        match self {
            ObjectType::House => "house",
            ObjectType::RedBloodCell => "red_blood_cell",
            ObjectType::VirusStation => "virus_station",
            ObjectType::FoodStation => "food_station",
            ObjectType::VaccineStation => "vaccine_station",
        }
    }
}
