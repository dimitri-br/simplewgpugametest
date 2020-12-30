use crate::{ComponentBase, UniformBuffer, Renderer};
use wgpu::util::DeviceExt;
use cgmath::{InnerSpace, SquareMatrix};
use std::any::Any;

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);

const ID: u32 = 4;

pub struct Transform{
    position: cgmath::Vector3::<f32>,
    rotation: cgmath::Quaternion::<f32>,
    scale: cgmath::Vector3::<f32>,
    pub value: cgmath::Matrix4::<f32>,
    id: u32
}
impl ComponentBase for Transform{
    fn get_id(&self) -> u32{
        self.id
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
impl Transform{
    pub fn new(position: cgmath::Vector3::<f32>, rotation: cgmath::Quaternion::<f32>, scale: cgmath::Vector3::<f32>) -> Self{
        let value = cgmath::Matrix4::from_translation(position) * cgmath::Matrix4::from(rotation) * cgmath::Matrix4::from_nonuniform_scale(scale.x, scale.y, scale.z);
        Self{
            position,
            rotation,
            scale,
            value,
            id: ID
        }
    }
    pub fn get_component_id() -> u32{
        ID
    }

    pub fn create_uniforms(&self, renderer_reference: &Renderer) -> TransformUniform{
        let mut translation_uniform = TransformUniform::new();
        translation_uniform.update(OPENGL_TO_WGPU_MATRIX * self.value);
        translation_uniform.create_uniform_buffer(renderer_reference);
        translation_uniform
    }
}

// We need this for Rust to store our data correctly for the shaders
#[repr(C)]
// This is so we can store this in a buffer
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TransformUniform{
    transform: [[f32; 4]; 4] // Store our rotation as a 4x4 matrix
}
impl TransformUniform{
    pub fn new() -> Self{
        Self{
            transform: cgmath::Matrix4::identity().into()
        }
    }

    pub fn update(&mut self, value: cgmath::Matrix4::<f32>){
        self.transform = value.into();
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

impl UniformBuffer for TransformUniform{}