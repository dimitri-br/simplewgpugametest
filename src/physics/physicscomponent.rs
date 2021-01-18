use wrapped2d::b2;
use crate::{ComponentBase, Physics, LayerType};
use std::any::Any;
use wrapped2d::user_data::UserData;


const ID: u32 = 7;

// Tag that states entity should be in physics simulation
pub struct PhysicsComponent{
    id: u32,
    pub shape: Box<dyn b2::Shape>,
    pub body: b2::BodyDef,
    pub handle: b2::BodyHandle,
    pub layer_type: u32,
}

impl PhysicsComponent{
    pub fn new(physics: &mut Physics, position: cgmath::Vector3::<f32>, scale: (f32, f32), body_type: b2::BodyType, layer_type: LayerType) -> Self{
        let shape = physics.create_shape(scale.0, scale.1);
        let body = physics.create_body(body_type, b2::Vec2 { x: position.x, y: position.y });
        let handle = physics.create_handle(&body);
        physics.bind_to_world(&handle, &shape);
        physics.world.body_mut(handle).set_user_data(Some(layer_type));

        Self{
            id: ID,
            shape,
            body,
            handle,
            layer_type,
        }
    }

    pub fn update_position(&mut self, physics: &mut Physics, new_pos: b2::Vec2){
        self.body.position = new_pos;
        self.update(physics);
    }

    pub fn update(&mut self, physics: &mut Physics){
        physics.world.destroy_body(self.handle);

        self.handle = physics.create_handle(&self.body);
        physics.bind_to_world(&self.handle, &self.shape);
        physics.world.body_mut(self.handle).set_user_data(Some(self.layer_type));
    }

    pub fn set_velocity(&mut self, physics: &mut Physics, offset: b2::Vec2){
        self.body.linear_velocity = offset;
        self.update(physics);
    }

    pub fn get_component_id() -> u32{
        ID
    }
}

impl ComponentBase for PhysicsComponent{

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
