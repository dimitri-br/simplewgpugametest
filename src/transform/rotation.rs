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

    pub fn create_uniforms(&self, renderer_reference: &Renderer) -> RotationUniform{
        let mut rotation_uniform = RotationUniform::new();
        rotation_uniform.update(self.value);
        rotation_uniform.create_uniform_buffer(renderer_reference);
        rotation_uniform
    }
}

// We need this for Rust to store our data correctly for the shaders
#[repr(C)]
// This is so we can store this in a buffer
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct RotationUniform{
    value: [[f32; 4]; 4] // Store our rotation as a 4x4 matrix
}
impl RotationUniform{
    pub fn new() -> Self{
        Self{
            value: cgmath::Matrix4::identity().into()
        }
    }

    pub fn update(&mut self, value: cgmath::Quaternion::<f32>){
        let mat4 : cgmath::Matrix4::<f32> = value.into();
        self.value = mat4.into();
    }

    pub fn create_uniform_buffer(&self, renderer_reference: &Renderer) -> wgpu::Buffer{
        renderer_reference.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Rotation Uniform Buffer"),
                contents: bytemuck::cast_slice(&[*self]),
                usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
            }
        )
    }
}

impl UniformBuffer for RotationUniform{}