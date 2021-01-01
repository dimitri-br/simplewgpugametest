use crate::{ComponentBase};
use std::any::Any;

pub const ID: u32 = 6;

pub struct PlayerMovementComponent{
    pub speed: f32,
    pub movement_vector: cgmath::Vector2::<f32>,
    id: u32
}

impl ComponentBase for PlayerMovementComponent{
    fn get_id(&self) -> u32{
        self.id
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl PlayerMovementComponent{
    pub fn new(speed: f32) -> Self{
        Self{
            speed,
            movement_vector: cgmath::Vector2::<f32> { x: 0.0, y: 0.0 },
            id: ID
        }
    }

    pub fn get_component_id() -> u32{
        ID
    }
}