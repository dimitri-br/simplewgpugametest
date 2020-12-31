use crate::{ComponentBase, UniformBuffer, Renderer};
use wgpu::util::DeviceExt;
use cgmath::{InnerSpace, SquareMatrix, VectorSpace};
use cgmath::Rotation as rotation;
use std::any::Any;

const ID: u32 = 2;

pub struct Rotation{
    pub value: cgmath::Quaternion::<f32>,
    id: u32,
}

impl ComponentBase for Rotation{
    fn get_id(&self) -> u32{
        self.id
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut Any {
        self
    }
}

impl Rotation{
    pub fn new(value: cgmath::Quaternion::<f32>) -> Self{
        Self{
            value,
            id: ID
        }
    }
    pub fn get_component_id() -> u32{
        ID
    }
    pub fn rotate_vector(&mut self, value: cgmath::Vector3::<f32>){
        self.value.rotate_vector(value);
    }

    pub fn rotate_point(&mut self, value: cgmath::Point3::<f32>){
        self.value.rotate_point(value);
    }

    pub fn lerp(&mut self, to: cgmath::Quaternion::<f32>, t: f32){
        self.value.lerp(to, t);
    }
}