use crate::{Renderer, Rc};
use wgpu::util::DeviceExt;

pub struct UniformUtils{
    pub uniforms: Vec<Rc<dyn UniformBuffer>>,
    pub buffers: Vec<Rc<Vec::<Rc<wgpu::Buffer>>>>,
    idx: u32,
}

impl UniformUtils{
    pub fn new() -> Self{
        Self{
            uniforms: Vec::<Rc<dyn UniformBuffer>>::new(),
            buffers: Vec::<Rc<Vec::<Rc<wgpu::Buffer>>>>::new(),
            idx: 0
        }
    }
    pub fn create_bind_group_layout(renderer_reference: &Renderer, binding: u32, visibility: wgpu::ShaderStage, label: Option<&str>) -> wgpu::BindGroupLayout{
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
            label,
        })
    }

    pub fn create_bind_group(renderer_reference: &Renderer, uniform_buffer: &wgpu::Buffer, uniform_bind_group_layout: &wgpu::BindGroupLayout, binding: u32, label: Option<&str>) -> wgpu::BindGroup{
        renderer_reference.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: uniform_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: binding,
                    resource: wgpu::BindingResource::Buffer(uniform_buffer.slice(..))
                }
            ],
            label,
        })
    }

    // Meant ONLY for initializing components!!!!
    pub fn generate_empty_buffer(renderer_reference: &Renderer) -> wgpu::Buffer{
        renderer_reference.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Rotation Uniform Buffer"),
                contents: bytemuck::cast_slice(&[0]),
                usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
            }
        )
    }

}

pub trait UniformBuffer{
}