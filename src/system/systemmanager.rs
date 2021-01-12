use crate::{Entity, ComponentBase, Rc, Renderer, SystemBase, Physics, EntityManager, InputManager, Camera};
use std::collections::HashMap;
use rayon::prelude::*;

pub struct SystemManager{
    systems: Vec<Box<dyn SystemBase>>,
    pub delta_time: f32,
}

impl SystemManager{
    pub fn new() -> Self{
        Self{
            systems: Vec::<Box<dyn SystemBase>>::new(),
            delta_time: 0.0
        }
    }
    pub fn add_system(&mut self, system: Box<dyn SystemBase>){
        self.systems.push(system);
    }

    pub fn update_systems(&mut self, renderer_reference: &Renderer, entity_manager: &mut EntityManager, input_manager: &InputManager, physics: &mut Physics, camera: &mut Camera){
        for system in self.systems.iter_mut(){
            system.execute(renderer_reference, entity_manager, input_manager, physics, self.delta_time, camera);
        }
    }
}