use crate::{ComponentBase, UniformBuffer, Renderer};
use wgpu::util::DeviceExt;
use cgmath::{InnerSpace, SquareMatrix};
use std::any::Any;

const ID: u32 = 1;

pub struct Translation{
    pub value: cgmath::Vector3::<f32>,
    id: u32,
}

impl ComponentBase for Translation{
    fn get_id(&self) -> u32{
        self.id
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Translation{
    pub fn new(value: cgmath::Vector3::<f32>) -> Self{
        Self{
            value,
            id: ID
        }
    }
    pub fn get_component_id() -> u32{
        ID
    }
    pub fn translate(&mut self, value: cgmath::Vector3::<f32>){
        self.value += value;
    }

    pub fn magnitude(&mut self) -> f32{
        self.value.magnitude()
    }

    pub fn create_uniforms(&self, renderer_reference: &Renderer) -> TranslationUniform{
        let mut translation_uniform = TranslationUniform::new();
        translation_uniform.update(self.value);
        translation_uniform.create_uniform_buffer(renderer_reference);
        translation_uniform
    }
}

// We need this for Rust to store our data correctly for the shaders
#[repr(C)]
// This is so we can store this in a buffer
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TranslationUniform{
    value: [[f32; 4]; 4] // Store our translation as a 4x4 matrix
}
impl TranslationUniform{
    pub fn new() -> Self{
        Self{
            value: cgmath::Matrix4::identity().into()
        }
    }
    

    pub fn update(&mut self, value: cgmath::Vector3::<f32>){
        self.value = cgmath::Matrix4::from_translation(value).into();
    }

    pub fn create_uniform_buffer(&self, renderer_reference: &Renderer) -> wgpu::Buffer{
        renderer_reference.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Transform Uniform Buffer"),
                contents: bytemuck::cast_slice(&[*self]),
                usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
            }
        )
    }
}

impl UniformBuffer for TranslationUniform{}
