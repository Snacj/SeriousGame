use indexmap::IndexMap;
use std::collections::HashMap;
use wgpu::util::DeviceExt;

use crate::engine::camera;
use crate::engine::texture;

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

const MAX_SPRITES: usize = 8192 * 2;
const MAX_VERTICES: usize = MAX_SPRITES * 4;
const MAX_INDICES: usize = MAX_SPRITES * 6;

pub struct Renderer {
    pipeline: wgpu::RenderPipeline,
    pipeline_pulse: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    pub camera: camera::Camera,
    camera_uniform: camera::CameraUniform,
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,
    texture_bind_group_layout: wgpu::BindGroupLayout,
    textures: HashMap<String, StoredTexture>,
    pulse_batches: IndexMap<String, Vec<Vertex>>,
    world_batches: IndexMap<String, Vec<Vertex>>,
    ui_batches: IndexMap<String, Vec<Vertex>>,
    batch_texture_map: HashMap<String, String>,
    time_buffer: wgpu::Buffer,
    time_bind_group_layout: wgpu::BindGroupLayout,
    time_bind_group: wgpu::BindGroup,
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

        let camera = camera::Camera::new(logical_height);
        let mut camera_uniform = camera::CameraUniform::new();
        camera_uniform.update(&camera);

        let camera_bind_group_layout = camera::create_bind_group_layout(device);
        let (camera_buffer, camera_bind_group) = camera::create_buffer_and_bind_group(
            device,
            &camera_bind_group_layout,
            &camera_uniform,
        );

        let time_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("time_bind_group_layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        let time_data: [f32; 4] = [0.0; 4];
        let time_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("time_buffer"),
            contents: bytemuck::cast_slice(&time_data),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let time_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("time_bind_group"),
            layout: &time_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: time_buffer.as_entire_binding(),
            }],
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[
                    Some(&camera_bind_group_layout),
                    Some(&texture_bind_group_layout),
                    Some(&time_bind_group_layout),
                ],
                immediate_size: 0,
            });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../../shader/shader.wgsl").into()),
        });
        let pipeline =
            Self::build_pipeline(device, &render_pipeline_layout, &shader, surface_format);

        let pulse_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Pulse Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../../shader/shader_pulse.wgsl").into()),
        });
        let pipeline_pulse = Self::build_pipeline(
            device,
            &render_pipeline_layout,
            &pulse_shader,
            surface_format,
        );

        let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Sprite Vertex Buffer"),
            size: (MAX_VERTICES * std::mem::size_of::<Vertex>()) as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

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
            pipeline_pulse,
            vertex_buffer,
            index_buffer,
            camera,
            camera_uniform,
            camera_buffer,
            camera_bind_group,
            texture_bind_group_layout,
            textures: HashMap::new(),
            pulse_batches: IndexMap::new(),
            world_batches: IndexMap::new(),
            ui_batches: IndexMap::new(),
            batch_texture_map: HashMap::new(),
            time_buffer,
            time_bind_group_layout,
            time_bind_group,
        }
    }

    fn build_pipeline(
        device: &wgpu::Device,
        layout: &wgpu::PipelineLayout,
        shader: &wgpu::ShaderModule,
        surface_format: wgpu::TextureFormat,
    ) -> wgpu::RenderPipeline {
        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(layout),
            vertex: wgpu::VertexState {
                module: shader,
                entry_point: Some("vs_main"),
                buffers: &[Vertex::desc()],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: shader,
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
        })
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

    pub fn create_solid_texture(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        name: &str,
        color: [u8; 4],
    ) {
        use wgpu::util::DeviceExt;
        let size = wgpu::Extent3d {
            width: 1,
            height: 1,
            depth_or_array_layers: 1,
        };
        let texture = device.create_texture_with_data(
            queue,
            &wgpu::TextureDescriptor {
                label: Some(name),
                size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                view_formats: &[],
            },
            wgpu::util::TextureDataOrder::LayerMajor,
            &color,
        );
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &self.texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
            label: Some(&format!("{name}_bind_group")),
        });
        self.textures.insert(
            name.to_string(),
            StoredTexture {
                texture: crate::engine::texture::Texture {
                    texture,
                    view,
                    sampler,
                },
                bind_group,
            },
        );
    }

    pub fn draw_sprite(&mut self, texture_name: &str, x: f32, y: f32, w: f32, h: f32) {
        let batch = self
            .world_batches
            .entry(texture_name.to_string())
            .or_default();
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
        let batch = self
            .world_batches
            .entry(texture_name.to_string())
            .or_default();
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

    pub fn draw_sprite_keyed(
        &mut self,
        batch_key: &str,
        texture_name: &str,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
    ) {
        self.batch_texture_map
            .insert(batch_key.to_string(), texture_name.to_string());
        let batch = self.world_batches.entry(batch_key.to_string()).or_default();
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

    pub fn draw_sprite_frame_keyed(
        &mut self,
        batch_key: &str,
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
        self.batch_texture_map
            .insert(batch_key.to_string(), texture_name.to_string());
        let u0 = (frame_col as f32 * frame_w) / texture_w;
        let u1 = u0 + frame_w / texture_w;
        let v0 = (frame_row as f32 * frame_h) / texture_h;
        let v1 = v0 + frame_h / texture_h;
        let batch = self.world_batches.entry(batch_key.to_string()).or_default();
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

    pub fn draw_sprite_pulsing(&mut self, texture_name: &str, x: f32, y: f32, w: f32, h: f32) {
        let batch = self
            .pulse_batches
            .entry(texture_name.to_string())
            .or_default();
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

    pub fn draw_sprite_pulsing_keyed(
        &mut self,
        batch_key: &str,
        texture_name: &str,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
    ) {
        self.batch_texture_map
            .insert(batch_key.to_string(), texture_name.to_string());
        let batch = self.pulse_batches.entry(batch_key.to_string()).or_default();
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

    pub fn draw_sprite_ui(&mut self, texture_name: &str, x: f32, y: f32, w: f32, h: f32) {
        let batch = self.ui_batches.entry(texture_name.to_string()).or_default();
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

    pub fn draw_sprite_ui_keyed(
        &mut self,
        batch_key: &str,
        texture_name: &str,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
    ) {
        self.batch_texture_map
            .insert(batch_key.to_string(), texture_name.to_string());
        let batch = self.ui_batches.entry(batch_key.to_string()).or_default();
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

    pub fn draw_sprite_frame_ui(
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
        let batch = self.ui_batches.entry(texture_name.to_string()).or_default();
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

    pub fn draw_sprite_frame_ui_keyed(
        &mut self,
        batch_key: &str,
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
        self.batch_texture_map
            .insert(batch_key.to_string(), texture_name.to_string());
        let u0 = (frame_col as f32 * frame_w) / texture_w;
        let u1 = u0 + frame_w / texture_w;
        let v0 = (frame_row as f32 * frame_h) / texture_h;
        let v1 = v0 + frame_h / texture_h;
        let batch = self.ui_batches.entry(batch_key.to_string()).or_default();
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

    pub fn render(
        &mut self,
        engine: &crate::engine::engine::Engine,
        time: f32,
    ) -> anyhow::Result<()> {
        if !engine.is_surface_configured {
            return Ok(());
        }

        self.camera_uniform.update(&self.camera);
        engine.queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[self.camera_uniform]),
        );

        let time_data: [f32; 4] = [time, 0.0, 0.0, 0.0];
        engine
            .queue
            .write_buffer(&self.time_buffer, 0, bytemuck::cast_slice(&time_data));

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
            wgpu::CurrentSurfaceTexture::Lost => anyhow::bail!("Lost device"),
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
                            g: 0.0,
                            b: 0.0,
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

            let mut pulse_calls: Vec<(String, u32, u32)> = Vec::new();
            for (key, vertices) in self.pulse_batches.drain(..) {
                let num = vertices.len() / 4;
                if num == 0 {
                    continue;
                }
                let base = all_vertices.len() as u32;
                all_vertices.extend_from_slice(&vertices);
                pulse_calls.push((key, base, num as u32));
            }

            let mut world_calls: Vec<(String, u32, u32)> = Vec::new();
            for (key, vertices) in self.world_batches.drain(..) {
                let num = vertices.len() / 4;
                if num == 0 {
                    continue;
                }
                let base = all_vertices.len() as u32;
                all_vertices.extend_from_slice(&vertices);
                world_calls.push((key, base, num as u32));
            }

            let mut ui_calls: Vec<(String, u32, u32)> = Vec::new();
            for (key, vertices) in self.ui_batches.drain(..) {
                let num = vertices.len() / 4;
                if num == 0 {
                    continue;
                }
                let base = all_vertices.len() as u32;
                all_vertices.extend_from_slice(&vertices);
                ui_calls.push((key, base, num as u32));
            }

            if !all_vertices.is_empty() {
                engine.queue.write_buffer(
                    &self.vertex_buffer,
                    0,
                    bytemuck::cast_slice(&all_vertices),
                );
            }

            render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
            render_pass.set_bind_group(2, &self.time_bind_group, &[]);

            render_pass.set_pipeline(&self.pipeline_pulse);
            self.draw_batch(&mut render_pass, &pulse_calls);

            render_pass.set_pipeline(&self.pipeline);
            self.draw_batch(&mut render_pass, &world_calls);
            self.draw_batch(&mut render_pass, &ui_calls);

            self.batch_texture_map.clear();
        }

        engine.queue.submit(std::iter::once(encoder.finish()));
        output.present();
        Ok(())
    }

    fn draw_batch(&self, render_pass: &mut wgpu::RenderPass, calls: &[(String, u32, u32)]) {
        for (batch_key, base_vertex, num_sprites) in calls {
            let real_tex = self
                .batch_texture_map
                .get(batch_key)
                .cloned()
                .unwrap_or_else(|| batch_key.clone());
            let stored = match self.textures.get(&real_tex) {
                Some(s) => s,
                None => continue,
            };
            let byte_offset = *base_vertex as u64 * std::mem::size_of::<Vertex>() as u64;
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(byte_offset..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.set_bind_group(1, &stored.bind_group, &[]);
            render_pass.draw_indexed(0..(num_sprites * 6), 0, 0..1);
        }
    }
}
