pub mod camera_uniform;

use camera_uniform::CameraUniform;
use crate::{Rc, RefCell, Renderer};

pub struct UniformUtils{
    pub uniforms: Vec<Box<dyn UniformBuffer>>,
    pub buffers: Vec<Box<Vec::<wgpu::Buffer>>>,

}

impl UniformUtils{
    pub fn new() -> Self{
        Self{
            uniforms: Vec::<Box<dyn UniformBuffer>>::new(),
            buffers: Vec::<Box<Vec::<wgpu::Buffer>>>::new(),
        }
    }
    pub fn create_bind_group_layout(renderer_reference: &Renderer, binding: u32, visibility: wgpu::ShaderStage) -> wgpu::BindGroupLayout{
        renderer_reference.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: binding,
                    visibility: visibility,
                    ty: wgpu::BindingType::UniformBuffer {
                        dynamic: false,
                        min_binding_size: None,
                    },
                    count: None,
                }
            ],
            label: Some("uniform_bind_group_layout"),
        })
    }

    pub fn create_bind_group(renderer_reference: &Renderer, uniform_buffer: &wgpu::Buffer, uniform_bind_group_layout: &wgpu::BindGroupLayout, binding: u32) -> wgpu::BindGroup{
        renderer_reference.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: uniform_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: binding,
                    resource: wgpu::BindingResource::Buffer(uniform_buffer.slice(..))
                }
            ],
            label: Some("uniform_bind_group"),
        })
    }
    pub fn add<T: UniformBuffer>(&mut self, k: T, v: Vec::<wgpu::Buffer>) where T: UniformBuffer + 'static{
        self.uniforms.push(Box::new(k));
        self.buffers.push(Box::new(v));
    }

    pub fn get_buffer_by_index(&self, index: usize) -> &Box<Vec::<wgpu::Buffer>>{
        &self.buffers[index]
    }
    pub fn get_uniform_by_index(&self, index: usize) -> &Box<dyn UniformBuffer>{
        &self.uniforms[index]
    }
}

pub trait UniformBuffer{}