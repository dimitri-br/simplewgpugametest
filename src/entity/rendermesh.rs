use wgpu::util::DeviceExt;
use crate::{Renderer, Vertex, Material, MaterialUniform, Rc, ComponentBase, UniformUtils};
use std::any::Any;

const ID: u32 = 0;

#[derive(std::fmt::Debug)]
pub struct RenderMesh{
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer, 
    num_vertices: u32,
    num_indices: u32,
    material: Material,
    pub id: u32,
}
impl ComponentBase for RenderMesh{
    fn get_id(&self) -> u32{
        self.id
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
impl RenderMesh{
    pub fn new(renderer_reference: &Renderer, material: Material) -> Self{
        const VERTICES: &[Vertex] = &[
            // Changed
            Vertex { position: [-1.0, 1.0, 0.0], tex_coords: [1.0, 0.0], }, // A
            Vertex { position: [-1.0, -1.0, 0.0], tex_coords: [1.0, 1.0], }, // A
            Vertex { position: [1.0, -1.0, 0.0], tex_coords: [0.0, 1.0], }, // A
            
            Vertex { position: [-1.0, 1.0, 0.0], tex_coords: [1.0, 0.0], }, // A
            Vertex { position: [1.0, -1.0, 0.0], tex_coords: [0.0, 1.0], }, // A
            Vertex { position: [1.0, 1.0, 0.0], tex_coords: [0.0, 0.0], }, // A
            
        ];


        const INDICES: &[u16] = &[
            
        ];

        let vertex_buffer = renderer_reference.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(VERTICES),
                usage: wgpu::BufferUsage::VERTEX,
            }
        );

        let index_buffer = renderer_reference.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(INDICES),
                usage: wgpu::BufferUsage::INDEX,
            }
        );

        let num_vertices = VERTICES.len() as u32;
        let num_indices = INDICES.len() as u32;


        Self{
            vertex_buffer,
            index_buffer,
            num_vertices,
            num_indices,
            material,
            id: ID
        }
    }
    pub fn get_component_id() -> u32{
        ID
    }

    pub fn get_vertex_buffer(&self) -> &wgpu::Buffer{
        &self.vertex_buffer
    }

    pub fn get_index_buffer(&self) -> &wgpu::Buffer{
        &self.index_buffer
    }

    pub fn get_num_vertices(&self) -> u32{
        self.num_vertices
    }

    pub fn get_num_indices(&self) -> u32{
        self.num_indices
    }

    pub fn borrow_material(&self) -> &Material{
        &self.material
    }

    pub fn generate_material_uniforms(&mut self, renderer_reference: &Renderer) -> (wgpu::BindGroup, wgpu::BindGroupLayout, MaterialUniform){
        self.material.create_uniform_group(renderer_reference)
    }

    pub fn draw<'a>(&'a self, render_pass: &'a mut wgpu::RenderPass<'a>, uniforms: &'a Vec::<Rc<wgpu::BindGroup>>){
        // 0 - texture count is reserved for textures
        render_pass.set_bind_group(0, self.borrow_material().borrow_texture().get_texture_group(), &[]);
        let mut i: u32 = 1;
        for uniform in uniforms.iter(){
            render_pass.set_bind_group(i, &uniform, &[]);
            i += 1;
        }
        render_pass.set_vertex_buffer(0, self.get_vertex_buffer().slice(..));
        render_pass.draw(0..self.get_num_vertices(), 0..1);
    }
    
}