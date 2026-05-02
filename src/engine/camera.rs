//! 2D orthographic camera with logical resolution scaling.
//!
//! The camera works in "world units" not screen pixels.
//! A logical viewport of 320×180 on a 1280×720 screen means
//! each world unit = 4 screen pixels. A 16px tile fills 64 screen pixels.

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.5,
    0.0, 0.0, 0.0, 1.0,
);

pub struct Camera {
    /// Camera position in world units (top-left corner of the viewport).
    pub position: cgmath::Point2<f32>,

    /// Logical viewport size in world units.
    /// This controls how many tiles you see on screen.
    /// e.g. (320, 180) means the viewport spans 320×180 world units,
    /// so you'd see 20×11.25 tiles if each tile is 16 world units.
    pub logical_width: f32,
    pub logical_height: f32,
}

impl Camera {
    pub fn new(logical_height: f32) -> Self {
        Self {
            position: cgmath::Point2::new(0.0, 0.0),
            // Width will be recalculated on first resize.
            logical_width: logical_height * (16.0 / 9.0),
            logical_height,
        }
    }

    /// Call this when the window resizes. Adjusts logical_width to
    /// preserve the aspect ratio (logical_height stays fixed).
    pub fn update_aspect_ratio(&mut self, screen_width: f32, screen_height: f32) {
        let aspect = screen_width / screen_height;
        self.logical_width = self.logical_height * aspect;
    }

    /// Build the combined projection × view matrix.
    pub fn build_projection_view_matrix(&self) -> cgmath::Matrix4<f32> {
        // Orthographic projection: maps the logical viewport to clip space.
        // top-left origin, Y-down.
        let proj = cgmath::ortho(0.0, self.logical_width, self.logical_height, 0.0, -1.0, 1.0);

        // View matrix: translate so camera.position is the origin.
        let view = cgmath::Matrix4::from_translation(cgmath::vec3(
            -self.position.x,
            -self.position.y,
            0.0,
        ));

        OPENGL_TO_WGPU_MATRIX * proj * view
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    proj_view: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new() -> Self {
        use cgmath::SquareMatrix;
        Self {
            proj_view: cgmath::Matrix4::identity().into(),
        }
    }

    pub fn update(&mut self, camera: &Camera) {
        self.proj_view = camera.build_projection_view_matrix().into();
    }
}

pub fn create_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("camera_bind_group_layout"),
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }],
    })
}

pub fn create_buffer_and_bind_group(
    device: &wgpu::Device,
    layout: &wgpu::BindGroupLayout,
    uniform: &CameraUniform,
) -> (wgpu::Buffer, wgpu::BindGroup) {
    use wgpu::util::DeviceExt;

    let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("camera_buffer"),
        contents: bytemuck::cast_slice(&[*uniform]),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });

    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("camera_bind_group"),
        layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: buffer.as_entire_binding(),
        }],
    });

    (buffer, bind_group)
}
