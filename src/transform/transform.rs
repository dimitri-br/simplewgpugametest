use crate::{ComponentBase, UniformBuffer, Renderer, UniformUtils, Rc};
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
    pub position: cgmath::Vector3::<f32>,
    pub rotation: cgmath::Quaternion::<f32>,
    pub scale: cgmath::Vector3::<f32>,
    pub value: cgmath::Matrix4::<f32>,
    buffer: wgpu::Buffer,
    id: u32
}
impl ComponentBase for Transform{
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
impl Transform{
    pub fn new(renderer_reference: &Renderer, position: cgmath::Vector3::<f32>, rotation: cgmath::Quaternion::<f32>, scale: cgmath::Vector3::<f32>) -> Self{
        let value = cgmath::Matrix4::from_translation(position) * cgmath::Matrix4::from(rotation) * cgmath::Matrix4::from_nonuniform_scale(scale.x, scale.y, scale.z);
        Self{
            position,
            rotation,
            scale,
            value,
            buffer: UniformUtils::generate_empty_buffer(renderer_reference),
            id: ID
        }
    }
    pub fn get_component_id() -> u32{
        ID
    }

    pub fn create_uniforms(&mut self, renderer_reference: &Renderer) -> (wgpu::BindGroup, wgpu::BindGroupLayout, TransformUniform){
        let mut translation_uniform = TransformUniform::new();
        translation_uniform.update(OPENGL_TO_WGPU_MATRIX * self.value);
        let buffer = translation_uniform.create_uniform_buffer(renderer_reference);
        let layout = UniformUtils::create_bind_group_layout(renderer_reference, 0, wgpu::ShaderStage::VERTEX, Some("Transform"));
        self.buffer = buffer;
        (UniformUtils::create_bind_group(&renderer_reference, &self.buffer, &layout, 0, Some("Transform")), layout, translation_uniform)
    }

    pub fn get_buffer_reference(&self) -> &wgpu::Buffer{
        &self.buffer
    }

    pub fn generate_matrix(&mut self) -> cgmath::Matrix4::<f32>{
        self.value = cgmath::Matrix4::from_translation(self.position) * 
        cgmath::Matrix4::from(self.rotation) * cgmath::Matrix4::from_nonuniform_scale(self.scale.x, self.scale.y, self.scale.z);

        OPENGL_TO_WGPU_MATRIX * self.value
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