use crate::{SystemBase, EntityManager, MovementComponent, Transform, Rc, RefCell, Renderer, PlayerMovementComponent, InputManager, Camera, Physics, PhysicsComponent, b2};
use cgmath::InnerSpace;
use cgmath::Rotation;

pub struct MovementSystem{
    x: f32,
    move_dir: cgmath::Vector3::<f32>
}

impl SystemBase for MovementSystem{
    fn execute(&mut self, renderer: &Renderer, entity_manager: &mut EntityManager, input_manager: &InputManager, physics: &mut Physics, delta_time: f32, camera: &mut Camera){
        for entity_ref in entity_manager.get_entities_with_types(&[PlayerMovementComponent::get_component_id()]){
            if entity_ref.try_find_component(PlayerMovementComponent::get_component_id()).is_ok(){

                let component = entity_ref.get_component::<PlayerMovementComponent>(PlayerMovementComponent::get_component_id()).unwrap();
                self.move_dir = component.position;
            }
        }
        let mut points = 0;
        for entity_ref in entity_manager.get_entities_with_types_mut(&[MovementComponent::get_component_id(), Transform::get_component_id(), PhysicsComponent::get_component_id()]){


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

            let mut reset_pos = false;
            if transform.position.y < -10.0{
                reset_pos = true;
            }

            let mut move_dir: cgmath::Vector3::<f32> = cgmath::Vector3::<f32> { x: 0.0, y: 0.0, z: 0.0};
            if (transform.position - self.move_dir).magnitude() > -1.0{
                move_dir = (transform.position - self.move_dir).normalize();
            }

            if transform.position.y < -5.0{
                move_dir.y = 0.0;
            }
            /*
                //transform.position += move_dir * speed;
                transform.rotation = cgmath::Quaternion::from(cgmath::Euler {
                    x: cgmath::Deg(0.0),
                    y: cgmath::Deg(0.0),
                    z: cgmath::Deg(self.x),
                });
                self.x = lerp(self.x, self.x + 32.0 * delta_time, 0.25);
            }*/
            
            

            transform.update_uniform_buffers(&renderer);

            drop(temp);

            let mut temp = temp_entity.borrow_mut();
            let phys_ref = match temp.get_component_mut::<PhysicsComponent>(PhysicsComponent::get_component_id()){
                Ok(transform) => transform,
                Err(_) => panic!("Error - component not found!"),
            };
            if reset_pos{
                phys_ref.update_position(physics, b2::Vec2 { x: 0.0, y: 5.0 });
                points += 1;
            }
            let body = physics.world.body(phys_ref.handle);
            let lin_vel = body.linear_velocity();
            let vel_y = lin_vel.y;
            let vel_x = lin_vel.x;
            let center = *body.world_center();
            drop(body);
            let mut body = physics.world.body_mut(phys_ref.handle);
            body.apply_linear_impulse(&b2::Vec2{ x: (move_dir.x * speed), y: (move_dir.y * speed ) }, &center, true);
            drop(body);
            //phys_ref.set_velocity(physics, b2::Vec2{ x: (move_vec.x * speed * delta_time) + x_force, y: (move_vec.y * speed * delta_time) + gravity });

        }

        for entity_ref in entity_manager.get_entities_with_types_mut(&[PlayerMovementComponent::get_component_id()]){
            if entity_ref.try_find_component(PlayerMovementComponent::get_component_id()).is_ok(){

                let mut component = entity_ref.get_component_mut::<PlayerMovementComponent>(PlayerMovementComponent::get_component_id()).unwrap();
                
                component.points += points;
            }
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