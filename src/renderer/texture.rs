use image::GenericImageView;
use anyhow::*;
use crate::{Renderer};

#[derive(std::fmt::Debug)]
pub struct Texture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,

    pub texture_bind_group: wgpu::BindGroup,
    pub texture_bind_group_layout: wgpu::BindGroupLayout,
}

impl Texture {
    pub fn get_texture_layout(&self) -> &wgpu::BindGroupLayout{
        &self.texture_bind_group_layout
    }
    pub fn get_texture_group(&self) -> &wgpu::BindGroup{
        &self.texture_bind_group
    }

    // Generates texture bind layout
    pub fn generate_texture_layout(renderer: &Renderer) -> wgpu::BindGroupLayout{
        renderer.device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStage::FRAGMENT,
                        ty: wgpu::BindingType::SampledTexture {
                            multisampled: false,
                            dimension: wgpu::TextureViewDimension::D2,
                            component_type: wgpu::TextureComponentType::Uint,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStage::FRAGMENT,
                        ty: wgpu::BindingType::Sampler {
                            comparison: false,
                        },
                        count: None,
                    },
                ],
                label: Some("texture_bind_group_layout"),
            }
        )
    }

    pub fn load_texture(renderer_reference: &Renderer, path: &str) -> Result<Self>{
        use std::fs::File;
        use std::io::{BufReader, Read};

        let file = File::open(path);
        let file = match file{
            Ok(v) => v,
            Err(e) => panic!("Error opening file: {:?}", e),
        };
        let mut buf_reader = BufReader::new(file);
        let mut contents = Vec::<u8>::new();
        buf_reader.read_to_end(&mut contents).unwrap();

        let img = image::load_from_memory(&contents)?;
        Self::from_image(renderer_reference, &img, Some(path))
    }

    pub fn from_bytes(
        renderer_reference: &Renderer,
        bytes: &[u8], 
        label: &str,
    ) -> Result<Self> {
        let img = image::load_from_memory(bytes)?;
        Self::from_image(renderer_reference, &img, Some(label))
    }

    pub fn from_image(
        renderer_reference: &Renderer,
        img: &image::DynamicImage,
        label: Option<&str>,
    ) -> Result<Self> {
        let queue = &renderer_reference.queue;
        let device = &renderer_reference.device;

        let rgba = img.as_rgba8().unwrap();
        let dimensions = img.dimensions();

        let size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth: 1,
        };
        let texture = device.create_texture(
            &wgpu::TextureDescriptor {
                label,
                size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST,
            }
        );

        queue.write_texture(
            wgpu::TextureCopyView {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            rgba,
            wgpu::TextureDataLayout {
                offset: 0,
                bytes_per_row: 4 * dimensions.0,
                rows_per_image: dimensions.1,
            },
            size,
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(
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
        let texture_bind_group_layout = Texture::generate_texture_layout(renderer_reference);
        let texture_bind_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout: &texture_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&sampler),
                    }
                ],
                label: label,
            }
        );
        log::info!("Texture {:?} loaded", label);
        
        Ok(Self { texture, view, sampler, texture_bind_group, texture_bind_group_layout })
    }
}
