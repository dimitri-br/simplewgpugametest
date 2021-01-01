use crate::{ComponentBase};
use std::any::Any;

pub const ID: u32 = 5;

pub struct MovementComponent{
    pub speed: f32,
    id: u32
}

impl ComponentBase for MovementComponent{
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

impl MovementComponent{
    pub fn new(speed: f32) -> Self{
        Self{
            speed,
            id: ID
        }
    }

    pub fn get_component_id() -> u32{
        ID
    }
}