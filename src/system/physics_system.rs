use crate::{SystemBase, EntityManager, PhysicsComponent, Transform, b2, Entity, Renderer, PhysicsFilter, InputManager, Camera, Physics};
use cgmath::InnerSpace;
use cgmath::Rotation;
use wrapped2d::user_data::UserData;
pub struct PhysicsSystem{

}

impl SystemBase for PhysicsSystem{
    fn execute(&mut self, renderer: &Renderer, entity_manager: &mut EntityManager, input_manager: &InputManager, physics: &mut Physics, delta_time: f32, camera: &mut Camera){
        {
            physics.world.step(0.016, 6, 2);
        }

        let mut entities = entity_manager.get_entities_with_types_mut(&[PhysicsComponent::get_component_id(), Transform::get_component_id()]);
        for i in 0..entities.len(){
            let mut entity_ref = &mut entities[i];
            let body_type = entity_ref.get_component_mut::<PhysicsComponent>(PhysicsComponent::get_component_id()).unwrap().body.body_type;

            if body_type != b2::BodyType::Dynamic{
            }else{
                
                let phys_ref = entity_ref.get_component::<PhysicsComponent>(PhysicsComponent::get_component_id()).unwrap();
                let body = physics.world.body(phys_ref.handle);
                let transform = entity_ref.get_component_mut::<Transform>(Transform::get_component_id()).unwrap();
                transform.position = cgmath::Vector3::<f32> { x: body.position().x, y: body.position().y, z: 0.0};
                transform.rotation = cgmath::Quaternion::from(cgmath::Euler {
                    x: cgmath::Deg(0.0),
                    y: cgmath::Deg(0.0),
                    z: cgmath::Rad(body.angle()).into(),
                });

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

