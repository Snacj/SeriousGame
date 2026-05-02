use indexmap::IndexMap;
use wgpu::util::DeviceExt;

use crate::camera;
use crate::texture;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
}

impl Vertex {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ],
        }
    }
}

const MAX_SPRITES: usize = 8192;
const MAX_VERTICES: usize = MAX_SPRITES * 4;
const MAX_INDICES: usize = MAX_SPRITES * 6;

pub struct Renderer {
    pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,

    // Camera
    pub camera: camera::Camera,
    camera_uniform: camera::CameraUniform,
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,

    // Textures stored by name
    texture_bind_group_layout: wgpu::BindGroupLayout,
    textures: IndexMap<String, StoredTexture>,

    // Per-frame sprite batch, grouped by texture
    batches: IndexMap<String, Vec<Vertex>>,
}

struct StoredTexture {
    #[allow(dead_code)]
    texture: texture::Texture,
    bind_group: wgpu::BindGroup,
}

impl Renderer {
    pub fn new(
        device: &wgpu::Device,
        surface_format: wgpu::TextureFormat,
        logical_height: f32,
    ) -> Self {
        // Texture bind group layout
        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
                label: Some("texture_bind_group_layout"),
            });

        // Camera
        let camera = camera::Camera::new(logical_height);
        let mut camera_uniform = camera::CameraUniform::new();
        camera_uniform.update(&camera);

        let camera_bind_group_layout = camera::create_bind_group_layout(device);
        let (camera_buffer, camera_bind_group) = camera::create_buffer_and_bind_group(
            device,
            &camera_bind_group_layout,
            &camera_uniform,
        );

        // Shader
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        // Pipeline
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[
                    Some(&camera_bind_group_layout),
                    Some(&texture_bind_group_layout),
                ],
                immediate_size: 0,
            });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[Vertex::desc()],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview_mask: None,
            cache: None,
        });

        // Pre-allocate GPU buffers
        let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Sprite Vertex Buffer"),
            size: (MAX_VERTICES * std::mem::size_of::<Vertex>()) as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Pre-compute index buffer: every quad is (0,1,2, 0,2,3) offset by 4n
        let mut indices = Vec::with_capacity(MAX_INDICES);
        for i in 0..MAX_SPRITES as u16 {
            let base = i * 4;
            indices.extend_from_slice(&[base, base + 1, base + 2, base, base + 2, base + 3]);
        }
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Sprite Index Buffer"),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        Self {
            pipeline,
            vertex_buffer,
            index_buffer,
            camera,
            camera_uniform,
            camera_buffer,
            camera_bind_group,
            texture_bind_group_layout,
            textures: IndexMap::new(),
            batches: IndexMap::new(),
        }
    }

    pub fn load_texture(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        name: &str,
        bytes: &[u8],
    ) {
        let tex = texture::Texture::from_bytes(device, queue, bytes, name).unwrap();
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &self.texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&tex.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&tex.sampler),
                },
            ],
            label: Some(&format!("{name}_bind_group")),
        });
        self.textures.insert(
            name.to_string(),
            StoredTexture {
                texture: tex,
                bind_group,
            },
        );
    }

    /// Queue a textured quad at (x, y) with size (w, h) in world units.
    pub fn draw_sprite(&mut self, texture_name: &str, x: f32, y: f32, w: f32, h: f32) {
        let batch = self.batches.entry(texture_name.to_string()).or_default();
        batch.extend_from_slice(&[
            Vertex {
                position: [x, y, 0.0],
                tex_coords: [0.0, 0.0],
            },
            Vertex {
                position: [x + w, y, 0.0],
                tex_coords: [1.0, 0.0],
            },
            Vertex {
                position: [x + w, y + h, 0.0],
                tex_coords: [1.0, 1.0],
            },
            Vertex {
                position: [x, y + h, 0.0],
                tex_coords: [0.0, 1.0],
            },
        ]);
    }

    pub fn draw_sprite_frame(
        &mut self,
        texture_name: &str,
        x: f32,
        y: f32,
        draw_w: f32,
        draw_h: f32,
        frame_col: usize,
        frame_row: usize,
        frame_w: f32,
        frame_h: f32,
        texture_w: f32,
        texture_h: f32,
    ) {
        let u0 = (frame_col as f32 * frame_w) / texture_w;
        let u1 = u0 + frame_w / texture_w;
        let v0 = (frame_row as f32 * frame_h) / texture_h;
        let v1 = v0 + frame_h / texture_h;

        let batch = self.batches.entry(texture_name.to_string()).or_default();
        batch.extend_from_slice(&[
            Vertex {
                position: [x, y, 0.0],
                tex_coords: [u0, v0],
            },
            Vertex {
                position: [x + draw_w, y, 0.0],
                tex_coords: [u1, v0],
            },
            Vertex {
                position: [x + draw_w, y + draw_h, 0.0],
                tex_coords: [u1, v1],
            },
            Vertex {
                position: [x, y + draw_h, 0.0],
                tex_coords: [u0, v1],
            },
        ]);
    }

    pub fn resize(&mut self, screen_width: f32, screen_height: f32) {
        self.camera.update_aspect_ratio(screen_width, screen_height);
    }

    /// Upload camera, flush all sprite batches, present.
    pub fn render(&mut self, engine: &crate::engine::Engine) -> anyhow::Result<()> {
        if !engine.is_surface_configured {
            return Ok(());
        }

        // Update camera uniform
        self.camera_uniform.update(&self.camera);
        engine.queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[self.camera_uniform]),
        );

        // Acquire frame
        let output = match engine.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(t) => t,
            wgpu::CurrentSurfaceTexture::Suboptimal(t) => {
                engine.surface.configure(&engine.device, &engine.config);
                t
            }
            wgpu::CurrentSurfaceTexture::Timeout
            | wgpu::CurrentSurfaceTexture::Occluded
            | wgpu::CurrentSurfaceTexture::Validation => return Ok(()),
            wgpu::CurrentSurfaceTexture::Outdated => {
                engine.surface.configure(&engine.device, &engine.config);
                return Ok(());
            }
            wgpu::CurrentSurfaceTexture::Lost => {
                anyhow::bail!("Lost device");
            }
        };

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = engine
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
                multiview_mask: None,
            });

            let mut all_vertices: Vec<Vertex> = Vec::new();
            let mut draw_calls: Vec<(String, u32, u32)> = Vec::new();

            for (tex_name, vertices) in self.batches.drain(..) {
                let num_sprites = vertices.len() / 4;
                if num_sprites == 0 {
                    continue;
                }
                let base_vertex = all_vertices.len() as u32;
                all_vertices.extend_from_slice(&vertices);
                draw_calls.push((tex_name, base_vertex, num_sprites as u32));
            }

            if !all_vertices.is_empty() {
                engine.queue.write_buffer(
                    &self.vertex_buffer,
                    0,
                    bytemuck::cast_slice(&all_vertices),
                );
            }

            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

            for (tex_name, base_vertex, num_sprites) in &draw_calls {
                let stored = match self.textures.get(tex_name) {
                    Some(s) => s,
                    None => continue,
                };
                render_pass.set_bind_group(1, &stored.bind_group, &[]);
                render_pass.draw_indexed(0..(num_sprites * 6), *base_vertex as i32, 0..1);
            }
        }

        engine.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
