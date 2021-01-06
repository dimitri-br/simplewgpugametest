use crate::{Vertex, RenderMesh, EntityManager, PostProcessing, BloomUniform, Texture, Material, Rc, BaseUniforms};
use std::collections::HashMap;
use std::any::Any;
use winit::{
    window::Window,
};

pub struct Renderer {
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub sc_desc: wgpu::SwapChainDescriptor,
    swap_chain: wgpu::SwapChain,
    size: winit::dpi::PhysicalSize<u32>,
    //pub render_pipeline: wgpu::RenderPipeline,
    pub render_pipelines: HashMap<String, wgpu::RenderPipeline>,
    postprocessing: PostProcessing,
    pub sample_count: u32,
}

impl Renderer {
    // Creating some of the wgpu types requires async code
    pub async fn new(window: &Window, backend: &str, vsync_mode: wgpu::PresentMode) -> Self {
        let size = window.inner_size();

        // The instance is a handle to our GPU
        // BackendBit::PRIMARY => Vulkan + Metal + DX12 + Browser WebGPU

        let instance = match backend{
            "primary" => wgpu::Instance::new(wgpu::BackendBit::PRIMARY),
            "dx12" => wgpu::Instance::new(wgpu::BackendBit::DX12),
            "dx11" => wgpu::Instance::new(wgpu::BackendBit::DX11),
            "vulkan" => wgpu::Instance::new(wgpu::BackendBit::VULKAN),
            "metal" => wgpu::Instance::new(wgpu::BackendBit::METAL),
            _ => wgpu::Instance::new(wgpu::BackendBit::PRIMARY),
        };
        let surface = unsafe { instance.create_surface(window) };
        
        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
            },
        ).await.unwrap();

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::default(),
                limits: wgpu::Limits::default(),
                shader_validation: true,
            },
            None, // Trace path
        ).await.unwrap();
        
        let sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: size.width,
            height: size.height,
            present_mode: vsync_mode,
        };

        let swap_chain = device.create_swap_chain(&surface, &sc_desc);

        


        let render_pipelines = HashMap::<String, wgpu::RenderPipeline>::new();
        let sample_count = 8;

        let postprocessing = PostProcessing::new(&device, &sc_desc, sample_count, 1);

        Self {
            surface,
            device,
            queue,
            sc_desc,
            swap_chain,
            size,
            //render_pipeline,
            render_pipelines,
            postprocessing,
            sample_count
       }
    }

    
    fn generate_pipeline(device: &wgpu::Device, vs_module: wgpu::ShaderModule, fs_module: wgpu::ShaderModule,
        bind_group_layouts: &[&wgpu::BindGroupLayout], color_states: &[wgpu::ColorStateDescriptor], sample_count: u32) -> wgpu::RenderPipeline {

       let render_pipeline_layout =
       device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
           label: Some("Render Pipeline Layout"),
           bind_group_layouts: bind_group_layouts,
           push_constant_ranges: &[],
       });

       

        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Render Pipeline"),
        layout: Some(&render_pipeline_layout),
        vertex_stage: wgpu::ProgrammableStageDescriptor {
            module: &vs_module,
            entry_point: "main", // 1.
        },
        fragment_stage: Some(wgpu::ProgrammableStageDescriptor { // 2.
            module: &fs_module,
            entry_point: "main",
        }),
        rasterization_state: Some(
            wgpu::RasterizationStateDescriptor {
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: wgpu::CullMode::None,
                depth_bias: 0,
                depth_bias_slope_scale: 0.0,
                depth_bias_clamp: 0.0,
                clamp_depth: false,
            }
        ),
        color_states: color_states,

        primitive_topology: wgpu::PrimitiveTopology::TriangleList, // 1.
        depth_stencil_state: None, // 2.
        vertex_state: wgpu::VertexStateDescriptor {
            index_format: wgpu::IndexFormat::Uint32,
            vertex_buffers: &[
                Vertex::desc(), // Set our vertex buffer description here (Description defines the things like texcoords and normals)
            ],
        },
        sample_count: sample_count, // 5.
        sample_mask: !0, // 6.
        alpha_to_coverage_enabled: true, // 7.
    })
   }

   pub fn create_pipeline(&mut self, name: String, bind_group_layouts: &[&wgpu::BindGroupLayout], vertex_shader: wgpu::ShaderModuleSource, fragment_shader: wgpu::ShaderModuleSource, color_states: &[wgpu::ColorStateDescriptor], sample_count: u32){
    let vs_module = self.device.create_shader_module(vertex_shader);
    let fs_module = self.device.create_shader_module(fragment_shader);
    let new_pipeline = Renderer::generate_pipeline(&self.device, vs_module, fs_module, bind_group_layouts, color_states, sample_count);
    if !self.render_pipelines.contains_key(&name){
        self.render_pipelines.insert(name, new_pipeline);
    }else{
        *self.render_pipelines.get_mut(&name).unwrap() = new_pipeline;
    }
   }


    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
        self.sc_desc.width = new_size.width;
        self.sc_desc.height = new_size.height;
        self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
        self.postprocessing = PostProcessing::new(&self.device, &self.sc_desc, self.sample_count, 1);
    }

    pub fn update(&mut self) {
        // Not sure what to run here, maybe pipeline switching for multishader support?
    }

    pub fn render(&mut self, clear_color: wgpu::Color, entities: &EntityManager, time: &std::time::SystemTime) -> Result<(), wgpu::SwapChainError> {       
        let material = Material::new(&self, Rc::new(Texture::from_empty(&self.device).unwrap()), 1.0, 0.0, "none".to_string());
        let framebuffer = RenderMesh::new(&self, material);

        let frame = self
        .swap_chain
        .get_current_frame()?
        .output;

        

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        {
            // Pre pass
            // Main pass - Render all our shaders and objects to the screen
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[
                    wgpu::RenderPassColorAttachmentDescriptor {
                        attachment: &self.postprocessing.main_pass_draw_texture_view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(clear_color),
                            store: true,
                        }
                    },
                    wgpu::RenderPassColorAttachmentDescriptor {
                        attachment: &self.postprocessing.hdr_draw_texture_view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(clear_color),
                            store: true,
                        }
                    }
                ],
                depth_stencil_attachment: None,
            });
            for pipeline in self.render_pipelines.iter(){
                for entity in entities.get_entities_with_type(RenderMesh::get_component_id()){
                    let mesh = match entity.get_component::<RenderMesh>(RenderMesh::get_component_id()){
                        Ok(rm) => { rm }
                        Err(e) => panic!("{:?}", e)
                    };
                    if mesh.borrow_material().get_shader_name() == pipeline.0{
                        render_pass.set_pipeline(pipeline.1); // 2.

                        // 0 - texture count is reserved for textures
                        render_pass.set_bind_group(0, &mesh.borrow_material().borrow_texture().get_texture_group(), &[]);
                        let mut i: u32 = 1;
                        for uniform in entity.get_uniforms().iter(){
                            render_pass.set_bind_group(i, &uniform, &[]);
                            i += 1;
                        }
                        render_pass.set_vertex_buffer(0, mesh.get_vertex_buffer().slice(..));
                        if mesh.get_num_indices() == 0{
                            render_pass.draw(0..mesh.get_num_vertices(), 0..1);
                        }else{
                            render_pass.set_index_buffer(mesh.get_index_buffer().slice(..));
                            render_pass.draw_indexed(0..mesh.get_num_indices(), 0, 0..1);
                        }
                    }
                }
            }
        }
        {
            encoder.copy_texture_to_texture(
                wgpu::TextureCopyView{ texture: &self.postprocessing.main_pass_draw_texture, mip_level: 0, origin: wgpu::Origin3d::ZERO}, 
                wgpu::TextureCopyView{ texture: &self.postprocessing.framebuffer_render_texture, mip_level: 0, origin: wgpu::Origin3d::ZERO}, 
                self.postprocessing.size);
            encoder.copy_texture_to_texture(
                wgpu::TextureCopyView{ texture: &self.postprocessing.hdr_draw_texture, mip_level: 0, origin: wgpu::Origin3d::ZERO}, 
                wgpu::TextureCopyView{ texture: &self.postprocessing.hdr_render_texture, mip_level: 0, origin: wgpu::Origin3d::ZERO}, 
                self.postprocessing.size);
        }
        // Post Processing after this point
        {
            let mut horizontal = true;
            let mut bloom_uniform = BloomUniform::new(horizontal as u8);

            let bloom_uniform_uniform = bloom_uniform.create_uniform_group(self);
            let bind_group = bloom_uniform_uniform.0;
            let mut bundle_iterator;
            for _ in 0..self.postprocessing.bloom_intensity{
                {
                    let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        color_attachments: &[
                            wgpu::RenderPassColorAttachmentDescriptor {
                                attachment: &self.postprocessing.hdr_draw_texture_view,
                                resolve_target: None,
                                ops: wgpu::Operations {
                                    load: wgpu::LoadOp::Clear(clear_color),
                                    store: true,
                                }
                            }
                        ],
                        depth_stencil_attachment: None,
                    });
                    let mut bloom_encoder = self.device.create_render_bundle_encoder(&wgpu::RenderBundleEncoderDescriptor{
                        color_formats: &[wgpu::TextureFormat::Rgba8UnormSrgb],
                        depth_stencil_format: None,
                        sample_count: 1,
                        label: Some("Bloom encoder")
                    });
                    bloom_encoder.set_pipeline(&self.render_pipelines["bloom"]);
        
                    bloom_encoder.set_bind_group(0, &self.postprocessing.hdr_render_texture_group, &[]);
                    
    
                    bloom_encoder.set_bind_group(1, &bind_group, &[]);  
                    
                    bloom_encoder.set_vertex_buffer(0, framebuffer.get_vertex_buffer().slice(..));
                    bloom_encoder.draw(0..framebuffer.get_num_vertices(), 0..1);
    
                    horizontal = !horizontal;
                    bloom_uniform.horizonal = horizontal as u8;

                    let bundle = bloom_encoder.finish(&wgpu::RenderBundleDescriptor{
                        label: Some("Render Bundle"),
                    });
                    bundle_iterator = [bundle];
                    render_pass.execute_bundles(bundle_iterator.iter());
                }
    
                {
                    encoder.copy_texture_to_texture(
                        wgpu::TextureCopyView{ texture: &self.postprocessing.hdr_draw_texture, mip_level: 0, origin: wgpu::Origin3d::ZERO}, 
                        wgpu::TextureCopyView{ texture: &self.postprocessing.hdr_render_texture, mip_level: 0, origin: wgpu::Origin3d::ZERO}, 
                        self.postprocessing.size);
                }
            }
            
        }
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[
                    wgpu::RenderPassColorAttachmentDescriptor {
                        attachment: &self.postprocessing.main_pass_draw_texture_view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(clear_color),
                            store: true,
                        }
                    }
                ],
                depth_stencil_attachment: None,
            });
            // Post pass
            render_pass.set_pipeline(&self.render_pipelines["fxaa"]);
            render_pass.set_bind_group(0, &self.postprocessing.framebuffer_render_texture_group, &[]);
            render_pass.set_vertex_buffer(0, framebuffer.get_vertex_buffer().slice(..));
            render_pass.draw(0..framebuffer.get_num_vertices(), 0..1);
        }
        {
            
            encoder.copy_texture_to_texture(
                wgpu::TextureCopyView{ texture: &self.postprocessing.main_pass_draw_texture, mip_level: 0, origin: wgpu::Origin3d::ZERO}, 
                wgpu::TextureCopyView{ texture: &self.postprocessing.framebuffer_render_texture, mip_level: 0, origin: wgpu::Origin3d::ZERO}, 
                self.postprocessing.size);
            
        }
        let mut uniform = BaseUniforms::new();
        uniform.iTime = time.elapsed().unwrap().as_secs_f32();
        uniform.iResolution = [self.sc_desc.width as f32, self.sc_desc.height as f32];
        let bind_group = uniform.create_uniform_group(&self);
        {
            let mut render_pass;
            if self.sample_count > 1{
                render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    color_attachments: &[
                        wgpu::RenderPassColorAttachmentDescriptor {
                            attachment: &self.postprocessing.msaa_framebuffer_view,
                            resolve_target: Some(&frame.view),
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(clear_color),
                                store: true,
                            }
                        }
                    ],
                    depth_stencil_attachment: None,
                });
            }else{
                render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    color_attachments: &[
                        wgpu::RenderPassColorAttachmentDescriptor {
                            attachment: &frame.view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(clear_color),
                                store: true,
                            }
                        }
                    ],
                    depth_stencil_attachment: None,
                });
            }

            // Post pass
            render_pass.set_pipeline(&self.render_pipelines["framebuffer"]);
            render_pass.set_bind_group(0, &self.postprocessing.framebuffer_render_texture_group, &[]);
            render_pass.set_bind_group(1, &self.postprocessing.hdr_render_texture_group, &[]);
            render_pass.set_bind_group(2, &bind_group.0, &[]);
            render_pass.set_vertex_buffer(0, framebuffer.get_vertex_buffer().slice(..));
            render_pass.draw(0..framebuffer.get_num_vertices(), 0..1);
        }
        
        
        // submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish()));
        
        Ok(())    
    }

    pub fn get_window_size(&self) -> winit::dpi::PhysicalSize<u32>{
        self.size
    }

    pub fn write_buffer<T>(&self, buffer: &wgpu::Buffer, offset: u64, uniforms: &[T]) where T: bytemuck::Pod{
        self.queue.write_buffer(buffer, offset, bytemuck::cast_slice(uniforms));
    }
}