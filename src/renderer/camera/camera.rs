use cgmath::SquareMatrix;
use wgpu::util::DeviceExt;
use crate::{Renderer, UniformBuffer, UniformUtils};

pub struct Camera {
    pub eye: cgmath::Point3<f32>,
    pub target: cgmath::Point3<f32>,
    pub up: cgmath::Vector3<f32>,
    pub aspect: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,

    buffer: wgpu::Buffer
}

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);


impl Camera {
    pub fn new(renderer_reference: &Renderer, eye: cgmath::Point3<f32>, target: cgmath::Point3<f32>, up: cgmath::Vector3<f32>, aspect: f32, fovy: f32, znear: f32, zfar: f32) -> Self{
        Self{
            eye,
            target,
            up,
            aspect,
            fovy,
            znear,
            zfar,
            buffer: UniformUtils::generate_empty_buffer(renderer_reference)
        }
    }
    pub fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        // 1.
        let view = cgmath::Matrix4::look_at(self.eye, self.target, self.up);
        // 2.
        let proj = cgmath::perspective(cgmath::Deg(self.fovy), self.aspect, self.znear, self.zfar);

        // 3.
        return OPENGL_TO_WGPU_MATRIX * proj * view;
    }

    pub fn create_uniforms(&mut self, renderer_reference: &Renderer) -> (wgpu::BindGroup, wgpu::BindGroupLayout, CameraUniform){
        let mut translation_uniform = CameraUniform::new();
        let buffer = translation_uniform.create_uniform_buffer(renderer_reference);
        let layout = UniformUtils::create_bind_group_layout(renderer_reference, 0, wgpu::ShaderStage::VERTEX, Some("Transform"));
        self.buffer = buffer;
        (UniformUtils::create_bind_group(&renderer_reference, &self.buffer, &layout, 0, Some("Transform")), layout, translation_uniform)
    }

    pub fn get_buffer_reference(&self) -> &wgpu::Buffer{
        &self.buffer
    }
}

 


// We need this for Rust to store our data correctly for the shaders
#[repr(C)]
// This is so we can store this in a buffer
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform{
    pub view_proj: [[f32; 4]; 4],
}

impl CameraUniform{
    pub fn new() -> Self {
        Self {
            view_proj: cgmath::Matrix4::identity().into(),
        }
    }

    pub fn update_view_proj(&mut self, camera: &Camera) {
        self.view_proj = camera.build_view_projection_matrix().into();
    }

    pub fn create_uniform_buffer(&self, renderer_reference:&Renderer) -> wgpu::Buffer{
        renderer_reference.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Uniform Buffer"),
                contents: bytemuck::cast_slice(&[*self]),
                usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
            }
        )
    }
}

impl UniformBuffer for CameraUniform{}
 