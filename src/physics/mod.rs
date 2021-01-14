use wrapped2d::b2;

pub mod physicscomponent;

use crate::{World};

pub struct Physics{
    pub world: World,
}

impl Physics{
    pub fn new() -> Self{
        /* Physics */
        let mut world = World::new(&b2::Vec2 { x: 0., y: -10. }); // Physics world
        log::info!("Initialized the physics world");
        Self{
            world
        }
    }

    pub fn create_shape(&self, width: f32, height: f32) -> Box<dyn b2::Shape>{
        log::info!("Generating new physics shape");
        Box::new(b2::PolygonShape::new_box(width, height))
    }

    pub fn create_handle(&mut self, body_def: &b2::BodyDef) -> b2::BodyHandle{
        self.world.create_body(body_def)
    }

    pub fn create_body(&self, body_type: b2::BodyType, position: b2::Vec2) -> b2::BodyDef{
        log::info!("Generating new body definition");
        b2::BodyDef {
            body_type,
            position,
            ..b2::BodyDef::new()
        }
    }

    pub fn bind_to_world(&mut self, body_handle: &b2::BodyHandle, shape: &Box<dyn b2::Shape>){
        let mut fixture = b2::FixtureDef{
            density: 1.0,
            friction: 0.3,
            ..b2::FixtureDef::new()
        };
        self.world.body_mut(*body_handle).create_fixture(&**shape, &mut fixture);
        
    }
}