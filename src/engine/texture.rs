use anyhow::*;
use image::GenericImageView;

use crate::engine::{engine::Engine, renderer::Renderer};

pub struct Texture {
    #[allow(unused)]
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
}

impl Texture {
    pub fn from_bytes(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        bytes: &[u8],
        label: &str,
    ) -> Result<Self> {
        let img = image::load_from_memory(bytes)?;
        Self::from_image(device, queue, &img, Some(label))
    }

    pub fn from_image(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        img: &image::DynamicImage,
        label: Option<&str>,
    ) -> Result<Self> {
        let rgba = img.to_rgba8();
        let dimensions = img.dimensions();

        let size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };
        let format = wgpu::TextureFormat::Rgba8UnormSrgb;
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label,
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                aspect: wgpu::TextureAspect::All,
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            &rgba,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * dimensions.0),
                rows_per_image: Some(dimensions.1),
            },
            size,
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::MipmapFilterMode::Nearest,
            ..Default::default()
        });

        Ok(Self {
            texture,
            view,
            sampler,
        })
    }
}

/// Helper function to load sprites
pub fn load_sprites(engine: &Engine, renderer: &mut Renderer) {
    renderer.load_texture(
        &engine.device,
        &engine.queue,
        "grass",
        include_bytes!("../../assets/grass.png"),
    );

    renderer.load_texture(
        &engine.device,
        &engine.queue,
        "body_background",
        include_bytes!("../../assets/body_background_tile.png"),
    );

    renderer.load_texture(
        &engine.device,
        &engine.queue,
        "body_obstacle",
        include_bytes!("../../assets/body_obstacle_tile.png"),
    );

    renderer.load_texture(
        &engine.device,
        &engine.queue,
        "player",
        include_bytes!("../../assets/player_v3.png"),
    );

    renderer.load_texture(
        &engine.device,
        &engine.queue,
        "house",
        include_bytes!("../../assets/house.png"),
    );

    renderer.load_texture(
        &engine.device,
        &engine.queue,
        "font",
        include_bytes!("../../assets/font2.png"),
    );

    renderer.load_texture(
        &engine.device,
        &engine.queue,
        "dialogue_background",
        include_bytes!("../../assets/dialogue_background.png"),
    );

    renderer.create_solid_texture(&engine.device, &engine.queue, "debug_red", [255, 0, 0, 100]);
    renderer.create_solid_texture(
        &engine.device,
        &engine.queue,
        "transparent_gray",
        [128, 128, 128, 100],
    );
    renderer.create_solid_texture(&engine.device, &engine.queue, "ui_panel", [20, 20, 40, 210]);
}
