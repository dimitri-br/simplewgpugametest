use crate::{Renderer, UniformBuffer, UniformUtils};
use wgpu::util::DeviceExt;


// We need this for Rust to store our data correctly for the shaders
#[repr(C)]
// This is so we can store this in a buffer
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct BaseUniforms{
    pub iResolution: [f32; 2], // Store our rotation as a 4x4 matrix
    pub iTime: f32
}
impl BaseUniforms{
    pub fn new() -> Self{
        Self{
            iResolution: [0.0; 2],
            iTime: 0.0
        }
    }

    pub fn update(&mut self, resolution: cgmath::Vector2::<f32>, time: f32){
        self.iResolution = resolution.into();
        self.iTime = time;
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

    pub fn create_uniform_group(&mut self, renderer_reference: &Renderer) -> (wgpu::BindGroup, wgpu::BindGroupLayout){
        let buffer = self.create_uniform_buffer(renderer_reference);
        let layout = BaseUniforms::create_uniform_layout(renderer_reference);
        (UniformUtils::create_bind_group(renderer_reference, &buffer, &layout, 0, Some("base uniforms")), layout)
    }

    pub fn create_uniform_layout(renderer_reference: &Renderer) -> wgpu::BindGroupLayout{
        UniformUtils::create_bind_group_layout(renderer_reference, 0, wgpu::ShaderStage::FRAGMENT, Some("base uniforms"))
    }
}

impl UniformBuffer for BaseUniforms{}