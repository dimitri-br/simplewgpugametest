use crate::{SystemBase, EntityManager, MovementComponent, Transform, Rc, RefCell, Renderer, PlayerMovementComponent, InputManager, Camera};
use cgmath::InnerSpace;
use cgmath::Rotation;

pub struct MovementSystem{
    x: f32,
    move_dir: cgmath::Vector3::<f32>
}

impl SystemBase for MovementSystem{
    fn execute(&mut self, renderer: &Renderer, entity_manager: &mut EntityManager, input_manager: &InputManager, delta_time: f32, camera: &mut Camera){
        for entity_ref in entity_manager.get_entities_with_types(&[PlayerMovementComponent::get_component_id()]){
            if entity_ref.try_find_component(PlayerMovementComponent::get_component_id()).is_ok(){

                let component = entity_ref.get_component::<PlayerMovementComponent>(PlayerMovementComponent::get_component_id()).unwrap();
                self.move_dir = component.position;
            }
        }
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


            let move_dir = (transform.position - self.move_dir).normalize();
            if (transform.position - self.move_dir).magnitude() > 2.0{
                transform.position += move_dir * speed;
                transform.rotation = cgmath::Quaternion::from(cgmath::Euler {
                    x: cgmath::Deg(0.0),
                    y: cgmath::Deg(0.0),
                    z: cgmath::Deg(self.x),
                });
                self.x = lerp(self.x, self.x + 32.0 * delta_time, 0.25);
            }
            
            

            transform.update_uniform_buffers(&renderer);
            
        }
    }
}

impl MovementSystem{
    pub fn new() -> Self{
        Self{
            x: 0.0,
            move_dir: cgmath::Vector3::<f32> { x: 0.0, y: 0.0, z: 0.0}
        }
    }
}

fn lerp(start: f32, end: f32, t: f32) -> f32{
    start * (1.0 - t) + end * t
}