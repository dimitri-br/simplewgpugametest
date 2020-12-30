use wgpu::util::DeviceExt;
use crate::{Renderer, Vertex, Material, Rc, RefCell};



pub struct RenderMesh{
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer, 
    num_vertices: u32,
    num_indices: u32,
    material: Material,
    uniforms: Vec::<Rc::<wgpu::BindGroup>>
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

        let mut uniforms = Vec::<Rc::<wgpu::BindGroup>>::new();

        Self{
            vertex_buffer,
            index_buffer,
            num_vertices,
            num_indices,
            material,
            uniforms,
        }
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

    pub fn add_new_uniform(&mut self, uniform: Rc<wgpu::BindGroup>){
        self.uniforms.push(uniform);
    }

    pub fn get_uniforms(&self) -> Rc<&Vec::<Rc<wgpu::BindGroup>>>{
        Rc::new(&self.uniforms)
    }
}