use crate::{Entity, ComponentBase, Rc, b2, Physics, World};
use std::collections::HashMap;
use rayon::prelude::*;

pub struct EntityManager{
    pub entities: Vec::<Entity>,
}

impl EntityManager{
    pub fn new() -> Self{
        log::info!("Entity Manager Initialized");

        Self{
            entities: Vec::<Entity>::new(),
        }
    }

    pub fn create_entity(&mut self, components: Vec::<Box<dyn ComponentBase>>, uniforms: Vec::<Rc<wgpu::BindGroup>>){
        let component_count = format!("Entity created with {:?} components and {:?} uniforms", components.len(), uniforms.len());
        log::info!("{}", &component_count);
        let mut entity = Entity::new(components, self.entities.len());
        entity.set_uniforms(uniforms);
        self.entities.push(entity);
    }

    pub fn add_component_data<T: ComponentBase + 'static>(&mut self, entity_id: &usize, component: Box<dyn ComponentBase>) -> Result<&mut T, &str>{
        let entity = &mut self.entities[*entity_id];
        let component_id = component.get_id();
        entity.add_component(component);
        self.entities[*entity_id].get_component_mut(component_id)
    }

    pub fn remove_component_data(&mut self, entity: &mut Entity, component_id: u32){
        entity.try_remove_component(component_id).unwrap();
    }

    pub fn get_component_data<T: ComponentBase + 'static>(&mut self, entity: usize, component_id: u32) -> Result<&mut T, &str>{
        self.entities[entity].get_component_mut(component_id)
    }

    // Returns the index of the entity
    pub fn find_entity(&mut self, id: usize) -> Result<usize, &str>{
        if let Some(index) = self.entities.iter_mut().position(|x| x.id == id){
            return Ok(index);
        }
        Err("Entity not found")
    }

    pub fn get_entities_with_type(&self, id: u32) -> Vec::<&Entity>{
        let mut entities = Vec::<&Entity>::new();
        for entity in self.entities.iter(){
            if let Ok(_) = entity.try_find_component(id){
                entities.push(&entity);
            }
        }

        entities
    }

    pub fn get_entities_with_type_mut(&mut self, id: u32) -> Vec::<&mut Entity>{
        let mut entities = Vec::<&mut Entity>::new();
        for entity in self.entities.iter_mut(){
            match entity.try_find_component(id){
                Ok(_) => entities.push(&mut *entity),
                Err(_) => {}
            };
        }
        entities
    }


    pub fn get_entities_with_types_mut(&mut self, ids: &[u32]) -> Vec::<&mut Entity>{
        // Create an empty vec to store our entity indexes
        let mut entities = HashMap::<usize, usize>::new();

        let mut loop_count = 0;
        // Iterate through the type ids
        for id in ids{
            // Get all entities with the ID
            let mut entities_to_add = self.get_entities_with_type(*id);
            let mut contained_entities = HashMap::<usize, usize>::new();
            // Iterate through entities with ID
            for entity in entities_to_add{
                // If we've just started, add entity ID into the array regardless
                if loop_count == 0{
                    entities.insert(entity.id, 0);
                }else{

                    // Otherwise, check that our Vec contains the entity (By checking the ID of the entity is contained in our entity array)
                    if entities.contains_key(&entity.id){
                        contained_entities.insert(entity.id, 0);
                    }else{
                        // Otherwise, don't add it
                    }
                }
            }
            // Make sure this isn't our first loop. Save some precious render time
            if loop_count != 0{
                // Set our entities to the ones that are contained so we only have the entities we want. Filter it down
                entities = contained_entities;
            }

            loop_count += 1;
        }
        let ret_entities : Vec::<&mut Entity> = self.entities.iter_mut().filter(|item| entities.contains_key(&item.id)).collect();
        ret_entities
    }

    pub fn get_entities_with_types(&mut self, ids: &[u32]) -> Vec::<&Entity>{
        // Create an empty vec to store our entity indexes
        let mut entities = HashMap::<usize, usize>::new();
        let mut loop_count = 0;
        // Iterate through the type ids
        for id in ids{
            // Get all entities with the ID
            let entities_to_add = self.get_entities_with_type(*id);
            // Vec will act as a buffer to check what entities already exist, so we can filter down
            let mut contained_entities = HashMap::<usize, usize>::new();

            // Iterate through entities with ID
            for entity in entities_to_add.iter(){
                // If we've just started, add entity ID into the array regardless
                if loop_count == 0{
                    entities.insert(entity.id, 0);
                }else{

                    // Otherwise, check that our Vec contains the entity (By checking the ID of the entity is contained in our entity array)
                    if entities.contains_key(&entity.id){
                        contained_entities.insert(entity.id, 0);
                    }else{
                        // Otherwise, don't add it
                    }
                }
            }
            // Make sure this isn't our first loop. Save some precious render time
            if loop_count != 0{
                // Set our entities to the ones that are contained so we only have the entities we want. Filter it down
                entities = contained_entities;
            }

            loop_count += 1;
        }
        let ret_entities : Vec::<&Entity> = self.entities.iter().filter(|item| entities.contains_key(&item.id)).collect();
        ret_entities
    }
}