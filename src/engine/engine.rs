use std::sync::Arc;
use winit::window::Window;

pub struct Engine {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub surface: wgpu::Surface<'static>,
    pub config: wgpu::SurfaceConfiguration,
    pub window: Arc<Window>,
    pub is_surface_configured: bool,
}

impl Engine {
    pub async fn new(window: Arc<Window>) -> anyhow::Result<Self> {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            #[cfg(not(target_arch = "wasm32"))]
            backends: wgpu::Backends::PRIMARY,
            #[cfg(target_arch = "wasm32")]
            backends: wgpu::Backends::GL,
            flags: Default::default(),
            memory_budget_thresholds: Default::default(),
            backend_options: Default::default(),
            display: None,
        });

        let surface = instance.create_surface(window.clone()).unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                experimental_features: wgpu::ExperimentalFeatures::disabled(),
                required_limits: if cfg!(target_arch = "wasm32") {
                    wgpu::Limits::downlevel_webgl2_defaults()
                } else {
                    wgpu::Limits::default()
                },
                memory_hints: Default::default(),
                trace: wgpu::Trace::Off,
            })
            .await
            .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let (initial_width, initial_height) = Self::physical_size_from_window(&window);
        let (initial_width, initial_height) =
            Self::clamp_to_max_dim(initial_width, initial_height, device.limits().max_texture_dimension_2d);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: initial_width.max(1),
            height: initial_height.max(1),
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &config);

        Ok(Self {
            device,
            queue,
            surface,
            config,
            window,
            is_surface_configured: true,
        })
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        // The WebGL2 backend caps the surface at `max_texture_dimension_2d`
        // (2048 with downlevel defaults). Monitors wider/taller than that would
        // otherwise produce an invalid render target -> black/green screen.
        // Scale both axes by the same factor so the aspect ratio is preserved;
        // CSS upscales the canvas back to fill the window.
        let max_dim = self.device.limits().max_texture_dimension_2d;
        let (w, h) = Self::clamp_to_max_dim(width, height, max_dim);
        self.config.width = w;
        self.config.height = h;
        self.surface.configure(&self.device, &self.config);
        self.is_surface_configured = true;
    }

    pub fn resize_auto(&mut self) {
        let (w, h) = Self::physical_size_from_window(&self.window);
        self.resize(w, h);
    }

    pub fn screen_size(&self) -> (f32, f32) {
        (self.config.width as f32, self.config.height as f32)
    }

    fn physical_size_from_window(window: &Window) -> (u32, u32) {
        let size = window.inner_size();
        (size.width, size.height)
    }

    /// Scale (width, height) down uniformly so neither axis exceeds `max_dim`,
    /// preserving aspect ratio. Used to keep the WebGL2 surface within the
    /// backend's `max_texture_dimension_2d` limit.
    fn clamp_to_max_dim(width: u32, height: u32, max_dim: u32) -> (u32, u32) {
        let w = width.max(1);
        let h = height.max(1);
        if w <= max_dim && h <= max_dim {
            return (w, h);
        }
        let scale = (max_dim as f32 / w as f32).min(max_dim as f32 / h as f32);
        (
            ((w as f32 * scale).floor() as u32).clamp(1, max_dim),
            ((h as f32 * scale).floor() as u32).clamp(1, max_dim),
        )
    }
}
