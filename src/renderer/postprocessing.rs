use crate::{Texture, Renderer, UniformUtils};
use wgpu::util::DeviceExt;

pub struct PostProcessing{
    // Texture Info
    pub main_pass_draw_texture: wgpu::Texture, // Texture to draw scene to
    pub main_pass_draw_texture_view: wgpu::TextureView, // View

    pub hdr_draw_texture: wgpu::Texture, // Texture to draw bloom to
    pub hdr_draw_texture_view: wgpu::TextureView, // View

    pub msaa_framebuffer: wgpu::Texture, // Texture to sample MSAA to
    pub msaa_framebuffer_view: wgpu::TextureView, // View

    pub shadow_framebuffer: wgpu::Texture, // Texture to sample 1d shadowmap to
    pub shadow_framebuffer_view: wgpu::TextureView, // View

    pub framebuffer_render_texture: wgpu::Texture, // Texture to use in postpass
    pub framebuffer_render_texture_group: wgpu::BindGroup,

    pub hdr_render_texture: wgpu::Texture, // Texture to use in postpass
    pub hdr_render_texture_group: wgpu::BindGroup,

    
    pub shadow_render_texture: wgpu::Texture, // Texture to use in postpass
    pub shadow_render_texture_group: wgpu::BindGroup,

    pub size: wgpu::Extent3d,
    pub d_size: wgpu::Extent3d,


    // PPS info
    pub bloom_intensity: u32,
}

impl PostProcessing{
    // Must be recreated if swapchain is recreated!
    pub fn new(device: &wgpu::Device, sc_desc: &wgpu::SwapChainDescriptor, sample_count: u32, bloom_intensity: u32) -> Self{
        let size = wgpu::Extent3d {
            width: sc_desc.width,
            height: sc_desc.height,
            depth: 1,
        };

        let d_size = wgpu::Extent3d {
            width: 32,
            height: 32,
            depth: 1,
        };


        
        // The render pipeline renders data into this texture
        let main_pass_draw_texture = device.create_texture(&wgpu::TextureDescriptor {
            size: size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage:  sc_desc.usage | wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST | wgpu::TextureUsage::COPY_SRC,
            label: None,
        });

        let main_pass_draw_texture_view = main_pass_draw_texture.create_view(&wgpu::TextureViewDescriptor::default());


        let msaa_framebuffer = device.create_texture(&wgpu::TextureDescriptor {
            size: size,
            mip_level_count: 1,
            sample_count: sample_count,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            usage:  sc_desc.usage | wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST | wgpu::TextureUsage::COPY_SRC,
            label: None,
        });

        let msaa_framebuffer_view = msaa_framebuffer.create_view(&wgpu::TextureViewDescriptor::default());

        let shadow_framebuffer = device.create_texture(&wgpu::TextureDescriptor {
            size: d_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage:  sc_desc.usage | wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST | wgpu::TextureUsage::COPY_SRC,
            label: None,
        });

        let shadow_framebuffer_view = shadow_framebuffer.create_view(&wgpu::TextureViewDescriptor::default());


        let hdr_draw_texture = device.create_texture(&wgpu::TextureDescriptor {
            size: size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage:  sc_desc.usage | wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST | wgpu::TextureUsage::COPY_SRC,
            label: None,
        });

        let hdr_draw_texture_view = hdr_draw_texture.create_view(&wgpu::TextureViewDescriptor::default());


        
        let framebuffer_render_texture = device.create_texture(
            &wgpu::TextureDescriptor {
                label: None,
                size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST,
            }
        );
        
        let fb_view = framebuffer_render_texture.create_view(&wgpu::TextureViewDescriptor::default());
        
        let framebuffer_sampler = device.create_sampler(
            &wgpu::SamplerDescriptor {
                address_mode_u: wgpu::AddressMode::ClampToEdge,
                address_mode_v: wgpu::AddressMode::ClampToEdge,
                address_mode_w: wgpu::AddressMode::ClampToEdge,
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Nearest,
                mipmap_filter: wgpu::FilterMode::Nearest,
                ..Default::default()
            }
        );
        let framebuffer_render_texture_group_layout = Texture::generate_texture_layout_from_device(&device);
        let framebuffer_render_texture_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout: &framebuffer_render_texture_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&fb_view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&framebuffer_sampler),
                    }
                ],
                label: None,
            }
        );

        let hdr_render_texture = device.create_texture(
            &wgpu::TextureDescriptor {
                label: Some("HDR render tex"),
                size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST,
            }
        );
        
        let hdr_view = hdr_render_texture.create_view(&wgpu::TextureViewDescriptor::default());
        
        let hdr_sampler = device.create_sampler(
            &wgpu::SamplerDescriptor {
                address_mode_u: wgpu::AddressMode::ClampToEdge,
                address_mode_v: wgpu::AddressMode::ClampToEdge,
                address_mode_w: wgpu::AddressMode::ClampToEdge,
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Nearest,
                mipmap_filter: wgpu::FilterMode::Nearest,
                ..Default::default()
            }
        );
        let hdr_render_texture_group_layout = Texture::generate_texture_layout_from_device(&device);
        let hdr_render_texture_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout: &hdr_render_texture_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&hdr_view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&hdr_sampler),
                    }
                ],
                label: None,
            }
        );



        let shadow_render_texture = device.create_texture(
            &wgpu::TextureDescriptor {
                label: Some("HDR render tex"),
                size: d_size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST,
            }
        );
        
        let shadow_view = shadow_render_texture.create_view(&wgpu::TextureViewDescriptor::default());
        
        let shadow_sampler = device.create_sampler(
            &wgpu::SamplerDescriptor {
                address_mode_u: wgpu::AddressMode::ClampToEdge,
                address_mode_v: wgpu::AddressMode::ClampToEdge,
                address_mode_w: wgpu::AddressMode::ClampToEdge,
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Nearest,
                mipmap_filter: wgpu::FilterMode::Nearest,
                ..Default::default()
            }
        );
        let shadow_render_texture_layout = Texture::generate_texture_layout_from_device(&device);
        let shadow_render_texture_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout: &shadow_render_texture_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&shadow_view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&shadow_sampler),
                    }
                ],
                label: None,
            }
        );



        Self{
            main_pass_draw_texture,
            main_pass_draw_texture_view,

            hdr_draw_texture,
            hdr_draw_texture_view,

            msaa_framebuffer,
            msaa_framebuffer_view,

            shadow_framebuffer,
            shadow_framebuffer_view,

            framebuffer_render_texture,
            framebuffer_render_texture_group,



            hdr_render_texture,
            hdr_render_texture_group,

            shadow_render_texture,
            shadow_render_texture_group,
            
            size,
            d_size,


            bloom_intensity,
        }
    }
}


// We need this for Rust to store our data correctly for the shaders
#[repr(C)]
// This is so we can store this in a buffer
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct BloomUniform{
    pub horizonal: u8
}
impl BloomUniform{
    pub fn new(horizonal: u8) -> Self{
        Self{
            horizonal
        }
    }
    pub fn create_uniform_buffer(&self, renderer_reference: &Renderer) -> wgpu::Buffer{
        renderer_reference.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Material Uniform Buffer"),
                contents: bytemuck::cast_slice(&[*self]),
                usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
            }
        )
    }

    pub fn create_uniform_group(&mut self, renderer_reference: &Renderer) -> (wgpu::BindGroup, wgpu::BindGroupLayout){
        let buffer = self.create_uniform_buffer(renderer_reference);
        let layout = BloomUniform::create_uniform_layout(renderer_reference);
        (UniformUtils::create_bind_group(renderer_reference, &buffer, &layout, 0, Some("material")), layout)
    }

    pub fn create_uniform_layout(renderer_reference: &Renderer) -> wgpu::BindGroupLayout{
        UniformUtils::create_bind_group_layout(renderer_reference, 0, wgpu::ShaderStage::FRAGMENT, Some("material"))
    }
}


impl crate::UniformBuffer for BloomUniform{
}