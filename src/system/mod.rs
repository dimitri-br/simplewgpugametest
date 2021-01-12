// Systems for entities
pub mod movement_system;
pub mod player_movement_system;
pub mod systemmanager;
pub mod physics_system;

use crate::{Renderer, EntityManager, Rc, Physics, InputManager, Camera};

pub trait SystemBase{
    fn execute(&mut self, renderer: &Renderer, entity_manager: &mut EntityManager, input_manager: &InputManager, physics: &mut Physics, delta_time: f32, camera: &mut Camera);
}

