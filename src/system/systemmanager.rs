use crate::{Entity, ComponentBase, Rc, Renderer, SystemBase, RefCell, EntityManager, InputManager};
use std::collections::HashMap;

pub struct SystemManager{
    systems: Vec<Box<dyn SystemBase>>
}

impl SystemManager{
    pub fn new() -> Self{
        Self{
            systems: Vec::<Box<dyn SystemBase>>::new()
        }
    }
    pub fn add_system(&mut self, system: Box<dyn SystemBase>){
        self.systems.push(system);
    }

    pub fn update_systems(&mut self, renderer_reference: &Renderer, entity_manager: &mut EntityManager, input_manager: &InputManager){
        for system in self.systems.iter_mut(){
            system.execute(renderer_reference, entity_manager, input_manager);
        }
    }
}