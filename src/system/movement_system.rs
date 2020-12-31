use crate::{System, Entity, ComponentBase, Transform, Renderer};

pub struct MovementSystem{
    x: f32,
}

impl System for MovementSystem{}

impl MovementSystem{
    pub fn new() -> Self{
        Self{
            x: 0.0
        }
    }

    pub fn execute(&mut self, renderer: &Renderer, entities: &mut Vec::<Entity>){
        for entity in entities.iter_mut(){
            match entity.get_component_mut::<Transform>(Transform::get_component_id()){
                Ok(transform) => {
                    transform.position += cgmath::Vector3::<f32> { x: 0.001, y: 0.001, z: 0.0};
                    transform.rotation = cgmath::Quaternion::from(cgmath::Euler {
                        x: cgmath::Deg(0.0),
                        y: cgmath::Deg(0.0),
                        z: cgmath::Deg(self.x),
                    });
                    self.x += 0.6;

                    let transform_uniform_ref = transform.get_uniform();
                    let mut transform_uniform = transform_uniform_ref.borrow_mut();

                    transform_uniform.update(transform.generate_matrix());
                    renderer.write_buffer(transform.get_buffer_reference(), 0, &[*transform_uniform]);
                },
                Err(_) => {}
            };
            
        }
    }
}