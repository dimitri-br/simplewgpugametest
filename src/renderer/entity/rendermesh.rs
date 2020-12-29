use wgpu::util::DeviceExt;
use crate::{Renderer, Vertex, Rc, RefCell};



pub struct RenderMesh{
    renderer_reference: Rc<RefCell<Renderer>>,
    vertex_buffer: wgpu::Buffer,
    num_verticies: u32,
    index_buffer: wgpu::Buffer, 
    num_indices: u32,
}

impl RenderMesh{
    pub fn new(renderer_reference: Rc<RefCell<Renderer>>) -> Self{
        const VERTICES: &[Vertex] = &[
            // Changed
            Vertex { position: [-0.0868241, 0.49240386, 0.0], tex_coords: [0.4131759, 0.00759614], }, // A
            Vertex { position: [-0.49513406, 0.06958647, 0.0], tex_coords: [0.0048659444, 0.43041354], }, // B
            Vertex { position: [-0.21918549, -0.44939706, 0.0], tex_coords: [0.28081453, 0.949397057], }, // C
            Vertex { position: [0.35966998, -0.3473291, 0.0], tex_coords: [0.85967, 0.84732911], }, // D
            Vertex { position: [0.44147372, 0.2347359, 0.0], tex_coords: [0.9414737, 0.2652641], }, // E
        ];


        const INDICES: &[u16] = &[
            0, 1, 4,
            1, 2, 4,
            2, 3, 4,
        ];

        let vertex_buffer = renderer_reference.borrow().device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(VERTICES),
                usage: wgpu::BufferUsage::VERTEX,
            }
        );

        let index_buffer = renderer_reference.borrow().device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(INDICES),
                usage: wgpu::BufferUsage::INDEX,
            }
        );

        let num_verticies = VERTICES.len() as u32;
        let num_indices = INDICES.len() as u32;


        Self{
            renderer_reference,
            vertex_buffer,
            num_verticies,
            index_buffer,
            num_indices
        }
    }

    pub fn get_vertex_buffer(&self) -> &wgpu::Buffer{
        &self.vertex_buffer
    }

    pub fn get_index_buffer(&self) -> &wgpu::Buffer{
        &self.index_buffer
    }

    pub fn get_num_verticies(&self) -> u32{
        self.num_verticies
    }

    pub fn get_num_indices(&self) -> u32{
        self.num_indices
    }
}