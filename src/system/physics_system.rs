use crate::{SystemBase, EntityManager, PhysicsComponent, Transform, b2, Entity, Renderer, PhysicsFilter, InputManager, Camera, Physics};
use cgmath::InnerSpace;
use cgmath::Rotation;
use wrapped2d::user_data::UserData;
pub struct PhysicsSystem{

}

impl SystemBase for PhysicsSystem{
    fn execute(&mut self, renderer: &Renderer, entity_manager: &mut EntityManager, input_manager: &InputManager, physics: &mut Physics, delta_time: f32, camera: &mut Camera){
        {
            physics.world.step(delta_time, 6, 2);
        }

        let mut entities = entity_manager.get_entities_with_types_mut(&[PhysicsComponent::get_component_id(), Transform::get_component_id()]);
        for i in 0..entities.len(){
            let mut entity_ref = &mut entities[i];
            let body_type = entity_ref.get_component_mut::<PhysicsComponent>(PhysicsComponent::get_component_id()).unwrap().body.body_type;

            if body_type != b2::BodyType::Dynamic{
                // Get the position first, as immutable
                let transform = entity_ref.get_component::<Transform>(Transform::get_component_id()).unwrap();
                let position = transform.position;
                // Check the physics position against the transform
                let phys_ref = entity_ref.get_component::<PhysicsComponent>(PhysicsComponent::get_component_id()).unwrap();
                let body = physics.world.body(phys_ref.handle);
                let phys_pos = body.position();
                let mut repos = false;
                if phys_pos.x == position.x && phys_pos.y == position.y{
                    repos = true;
                }
                drop(body);
                drop(phys_ref);

                // Only update the position if the position of the transform has changed, to free resources
                if repos{
                    let phys_ref = entity_ref.get_component_mut::<PhysicsComponent>(PhysicsComponent::get_component_id()).unwrap();
                    phys_ref.update_position(physics, b2::Vec2 { x: position.x, y: position.y });
                }
            }else{
                
                let phys_ref = entity_ref.get_component::<PhysicsComponent>(PhysicsComponent::get_component_id()).unwrap();
                let body = physics.world.body(phys_ref.handle);
                let transform = entity_ref.get_component_mut::<Transform>(Transform::get_component_id()).unwrap();
                transform.position = cgmath::Vector3::<f32> { x: body.position().x, y: body.position().y, z: 0.0};
                transform.update_uniform_buffers(&renderer);

            }

        }
    }
}

impl PhysicsSystem{
    pub fn new() -> Self{
        Self{

        }
    }
}

