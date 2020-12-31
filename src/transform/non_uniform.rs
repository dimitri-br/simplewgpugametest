use crate::{ComponentBase, UniformBuffer, Renderer};
use wgpu::util::DeviceExt;
use cgmath::{InnerSpace, SquareMatrix};
use std::any::Any;

const ID: u32 = 3;

pub struct NonUniformScale{
    pub value: cgmath::Vector3::<f32>,
    id: u32,
}

impl ComponentBase for NonUniformScale{
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

impl NonUniformScale{
    pub fn new(value: cgmath::Vector3::<f32>) -> Self{
        Self{
            value,
            id: ID
        }
    }
    pub fn get_component_id() -> u32{
        ID
    }
    pub fn set_scale(&mut self, value: cgmath::Vector3::<f32>){
        self.value = value;
    }
}
