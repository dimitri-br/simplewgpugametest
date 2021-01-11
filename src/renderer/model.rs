use crate::{Vertex, Material, Renderer, Texture, Rc};
use std::path::Path;
use wgpu::util::DeviceExt;
use anyhow::Context;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ModelVertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
    pub normal: [f32; 3],
}

impl Vertex for ModelVertex {
    fn desc<'a>() -> wgpu::VertexBufferDescriptor<'a> {
        use std::mem;
        wgpu::VertexBufferDescriptor {
            stride: mem::size_of::<ModelVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttributeDescriptor {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float3,
                },
                wgpu::VertexAttributeDescriptor {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float2,
                },
                wgpu::VertexAttributeDescriptor {
                    offset: mem::size_of::<[f32; 5]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float3,
                },
            ],
        }
    }
}


pub struct Model {
    pub meshes: Vec<Mesh>,
    pub materials: Vec<Material>,
}

pub struct Mesh {
    pub name: String,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_elements: u32,
    pub material: usize,
}

impl Model {
    pub fn load<P: AsRef<Path>>(
        renderer_reference: &Renderer,
        layout: &wgpu::BindGroupLayout,
        path: P,
    ) -> Result<Self, ()> {
        let device = &renderer_reference.device;

        let (obj_models, obj_materials) = tobj::load_obj(path.as_ref(), true).unwrap();

        // We're assuming that the texture files are stored with the obj file
        let containing_folder = path.as_ref().parent()
            .context("Directory has no parent").unwrap();

        let mut materials = Vec::new();
        for mat in obj_materials {
            let diffuse_path = mat.diffuse_texture;
            let mut diffuse_texture;
            if diffuse_path == ""{
                diffuse_texture = Texture::no_image(renderer_reference).unwrap();
            }else{
                diffuse_texture = Texture::load(renderer_reference, containing_folder.join(diffuse_path)).unwrap();
            }

            let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&diffuse_texture.view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&diffuse_texture.sampler),
                    },
                ],
                label: None,
            });

            materials.push(Material::new(renderer_reference, Rc::new(diffuse_texture), cgmath::Vector3::<f32> { x: 1.0, y: 1.0, z: 1.0}, 0.5, 0.0, 0, "main".to_string()));
        }

        let mut meshes = Vec::new();
        for m in obj_models {
            let mut vertices = Vec::new();
            for i in 0..m.mesh.positions.len() / 3 {
                vertices.push(ModelVertex {
                    position: [
                        m.mesh.positions[i * 3],
                        m.mesh.positions[i * 3 + 1],
                        m.mesh.positions[i * 3 + 2],
                    ],
                    tex_coords: [m.mesh.texcoords[i * 2], m.mesh.texcoords[i * 2 + 1]],
                    normal: [
                        m.mesh.normals[i * 3],
                        m.mesh.normals[i * 3 + 1],
                        m.mesh.normals[i * 3 + 2],
                    ],
                });
            }

            let vertex_buffer = device.create_buffer_init(
                &wgpu::util::BufferInitDescriptor {
                    label: Some(&format!("{:?} Vertex Buffer", path.as_ref())),
                    contents: bytemuck::cast_slice(&vertices),
                    usage: wgpu::BufferUsage::VERTEX,
                }
            );
            let index_buffer = device.create_buffer_init(
                &wgpu::util::BufferInitDescriptor {
                    label: Some(&format!("{:?} Index Buffer", path.as_ref())),
                    contents: bytemuck::cast_slice(&m.mesh.indices),
                    usage: wgpu::BufferUsage::INDEX,
                }
            );

            meshes.push(Mesh {
                name: m.name,
                vertex_buffer,
                index_buffer,
                num_elements: m.mesh.indices.len() as u32,
                material: m.mesh.material_id.unwrap_or(0),
            });
        }

        Ok(Self { meshes, materials })
    }

    pub fn generate_material_uniforms(&mut self, renderer_reference: &Renderer) -> Vec::<Rc<wgpu::BindGroup>>{
        let mut bind_groups = Vec::<Rc<wgpu::BindGroup>>::new();
        for material in self.materials.iter_mut(){
            let (bg, _, _) = material.create_uniform_group(renderer_reference);
            let bg = Rc::new(bg);
            bind_groups.push(bg);
        }
        return bind_groups;
    }
}


pub trait DrawModel<'a, 'b>
where
    'b: 'a,
{
    fn draw_mesh(&mut self, mesh: &'b Mesh, material_bind_group: &'b wgpu::BindGroup);
    fn draw_mesh_instanced(
        &mut self,
        material_bind_group: &'b wgpu::BindGroup,
        mesh: &'b Mesh,
        instances: std::ops::Range<u32>,
    );

}

impl<'a, 'b> DrawModel<'a, 'b> for wgpu::RenderPass<'a>
where
    'b: 'a,
{
    fn draw_mesh(&mut self, mesh: &'b Mesh, material_bind_group: &'b wgpu::BindGroup) {
        self.draw_mesh_instanced(material_bind_group, mesh, 0..1);
    }

    fn draw_mesh_instanced(
        &mut self,
        material_bind_group: &'b wgpu::BindGroup,
        mesh: &'b Mesh,
        instances: std::ops::Range<u32>,
    ) {
        self.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
        self.set_index_buffer(mesh.index_buffer.slice(..));
        self.set_bind_group(0, material_bind_group, &[]);
        self.draw_indexed(0..mesh.num_elements, 0, instances);
    }
}
