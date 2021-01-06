use crate::{SystemBase, EntityManager, MovementComponent, Transform, Renderer, Rc, RefCell, InputManager};

pub struct MovementSystem{
    x: f32,
}

impl SystemBase for MovementSystem{
    fn execute(&mut self, renderer: &Renderer, entity_manager: &mut EntityManager, input_manager: &InputManager, delta_time: f32){
        for entity_ref in entity_manager.get_entities_with_types_mut(&[MovementComponent::get_component_id(), Transform::get_component_id()]){



            let temp_entity = Rc::new(RefCell::new(entity_ref));
            let temp = temp_entity.borrow();
            let movement_component = match temp.get_component::<MovementComponent>(MovementComponent::get_component_id()){
                Ok(v) => v,
                Err(_) => continue,
            };
            let mut speed = 0.0;

            speed = movement_component.speed * delta_time;
            drop(temp);
            
            let mut temp = temp_entity.borrow_mut();
            let transform = match temp.get_component_mut::<Transform>(Transform::get_component_id()){
                Ok(transform) => transform,
                Err(_) => continue,
            };


            
            transform.position += cgmath::Vector3::<f32> { x: speed, y: speed, z: 0.0};
            transform.rotation = cgmath::Quaternion::from(cgmath::Euler {
                x: cgmath::Deg(0.0),
                y: cgmath::Deg(0.0),
                z: cgmath::Deg(self.x),
            });
            self.x += 5.0 * delta_time;

            transform.update_uniform_buffers(&renderer);
            
        }
    }
}

impl MovementSystem{
    pub fn new() -> Self{
        Self{
            x: 0.0,
        }
    }
}