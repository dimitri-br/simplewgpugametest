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
    pub body_type: b2::BodyType,
    pub layer_type: u32,
    pub allow_sleep: bool,
}

impl PhysicsComponent{
    pub fn new_box(physics: &mut Physics, position: cgmath::Vector3::<f32>, scale: (f32, f32), mass: f32, body_type: b2::BodyType, layer_type: LayerType, allow_sleep: bool) -> Self{
        let shape = physics.create_box_shape(scale.0, scale.1);
        let body = physics.create_body(body_type, b2::Vec2 { x: position.x, y: position.y }, allow_sleep);
        let handle = physics.create_handle(&body);
        match body_type{
            b2::BodyType::Static => {
                physics.bind_to_world(&handle, &shape, 0.0, 0.3);
            },
            b2::BodyType::Kinematic => {
                physics.bind_to_world(&handle, &shape, mass, 0.3);
            },
            b2::BodyType::Dynamic => {
                physics.bind_to_world(&handle, &shape, mass, 0.3);
            },
        }
        physics.world.body_mut(handle).set_user_data(Some(layer_type));

        Self{
            id: ID,
            shape,
            body,
            handle,
            body_type,
            layer_type,
            allow_sleep,
        }
    }

    pub fn new_circle(physics: &mut Physics, position: cgmath::Vector3::<f32>, scale: f32, mass: f32, body_type: b2::BodyType, layer_type: LayerType, allow_sleep: bool) -> Self{
        let shape = physics.create_circle_shape(scale);
        let body = physics.create_body(body_type, b2::Vec2 { x: position.x, y: position.y }, allow_sleep);
        let handle = physics.create_handle(&body);
        match body_type{
            b2::BodyType::Static => {
                physics.bind_to_world(&handle, &shape, 0.0, 0.3);
            },
            b2::BodyType::Kinematic => {
                physics.bind_to_world(&handle, &shape, mass, 0.3);
            },
            b2::BodyType::Dynamic => {
                physics.bind_to_world(&handle, &shape, mass, 0.3);
            },
        }
        physics.world.body_mut(handle).set_user_data(Some(layer_type));

        Self{
            id: ID,
            shape,
            body,
            handle,
            body_type,
            layer_type,
            allow_sleep,
        }
    }

    pub fn update_position(&mut self, physics: &mut Physics, new_pos: b2::Vec2){
        self.body.position = new_pos;
        self.update(physics);
    }

    pub fn update(&mut self, physics: &mut Physics){
        physics.world.destroy_body(self.handle);

        self.handle = physics.create_handle(&self.body);
        match self.body_type{
            b2::BodyType::Static => {
                physics.bind_to_world(&self.handle, &self.shape, 0.0, 0.3);
            },
            b2::BodyType::Kinematic => {
                physics.bind_to_world(&self.handle, &self.shape, 1.0, 0.3);
            },
            b2::BodyType::Dynamic => {
                physics.bind_to_world(&self.handle, &self.shape, 1.0, 0.3);
            },
        }
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
