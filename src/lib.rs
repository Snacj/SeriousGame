use std::sync::Arc;

use winit::{
    application::ApplicationHandler,
    event::*,
    event_loop::{ActiveEventLoop, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::Window,
};

use web_time::Instant;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use winit::platform::web::EventLoopExtWebSys;

mod animation;
mod camera;
mod engine;
mod game;
mod player;
mod tile;
mod input;
mod renderer;
mod texture;

use engine::Engine;
use game::{Game, MyGame};
use input::Input;
use renderer::Renderer;

/// Everything that needs async init gets sent through this on wasm.
struct Initialized {
    engine: Engine,
    renderer: Renderer,
    game: MyGame,
}

pub struct App {
    #[cfg(target_arch = "wasm32")]
    proxy: Option<winit::event_loop::EventLoopProxy<Initialized>>,
    engine: Option<Engine>,
    renderer: Option<Renderer>,
    game: Option<MyGame>,
    input: Input,
    last_frame: Instant,
    accumulator: f32,
}

impl App {
    pub fn new(#[cfg(target_arch = "wasm32")] event_loop: &EventLoop<Initialized>) -> Self {
        #[cfg(target_arch = "wasm32")]
        let proxy = Some(event_loop.create_proxy());
        Self {
            engine: None,
            renderer: None,
            game: None,
            input: Input::new(),
            last_frame: Instant::now(),
            accumulator: 0.0,
            #[cfg(target_arch = "wasm32")]
            proxy,
        }
    }
}

impl ApplicationHandler<Initialized> for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        #[allow(unused_mut)]
        let mut window_attributes = Window::default_attributes();

        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::JsCast;
            use winit::platform::web::WindowAttributesExtWebSys;
            const CANVAS_ID: &str = "canvas";
            let window = wgpu::web_sys::window().unwrap_throw();
            let document = window.document().unwrap_throw();
            let canvas = document.get_element_by_id(CANVAS_ID).unwrap_throw();
            let html_canvas_element = canvas.unchecked_into();
            window_attributes = window_attributes.with_canvas(Some(html_canvas_element));
        }

        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());

        #[cfg(not(target_arch = "wasm32"))]
        {
            let mut engine = pollster::block_on(Engine::new(window)).unwrap();
            engine.resize(engine.config.width, engine.config.height);

            let mut renderer = Renderer::new(&engine.device, engine.config.format, 180.0);
            let (sw, sh) = engine.screen_size();
            renderer.resize(sw, sh);

            let game = MyGame::init(&engine, &mut renderer);

            self.engine = Some(engine);
            self.renderer = Some(renderer);
            self.game = Some(game);
        }

        #[cfg(target_arch = "wasm32")]
        {
            if let Some(proxy) = self.proxy.take() {
                wasm_bindgen_futures::spawn_local(async move {
                    let mut engine = Engine::new(window)
                        .await
                        .expect("Unable to create engine");
                    engine.resize(engine.config.width, engine.config.height);

                    let mut renderer =
                        Renderer::new(&engine.device, engine.config.format, 180.0);
                    let (sw, sh) = engine.screen_size();
                    renderer.resize(sw, sh);

                    let game = MyGame::init(&engine, &mut renderer);

                    assert!(proxy
                        .send_event(Initialized { engine, renderer, game })
                        .is_ok());
                });
            }
        }
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, event: Initialized) {
        #[cfg(target_arch = "wasm32")]
        {
            event.engine.window.request_redraw();
        }
        self.engine = Some(event.engine);
        self.renderer = Some(event.renderer);
        self.game = Some(event.game);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        let (Some(engine), Some(renderer), Some(game)) =
            (&mut self.engine, &mut self.renderer, &mut self.game)
        else {
            return;
        };

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),

            WindowEvent::Resized(size) => {
                engine.resize(size.width, size.height);
                renderer.resize(size.width as f32, size.height as f32);
                game.on_resize(size.width as f32, size.height as f32);
            }

            WindowEvent::RedrawRequested => {
                let now = Instant::now();
                let frame_time = now.duration_since(self.last_frame).as_secs_f32();
                self.last_frame = now;

                let frame_time = frame_time.min(0.1);

                const FIXED_DT: f32 = 1.0 / 60.0;
                self. accumulator += frame_time;

                while self.accumulator >= FIXED_DT {
                    game.update(&self.input, FIXED_DT);
                    self.accumulator -= FIXED_DT;
                }
                game.render(renderer);

                match renderer.render(engine) {
                    Ok(_) => {}
                    Err(e) => {
                        log::error!("{e}");
                        event_loop.exit();
                    }
                }

                self.input.begin_frame();
                engine.window.request_redraw();
            }

            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(code),
                        state: key_state,
                        ..
                    },
                ..
            } => {
                if code == KeyCode::Escape && key_state.is_pressed() {
                    event_loop.exit();
                    return;
                }
                self.input.handle_key(code, key_state.is_pressed());
            }

            _ => {}
        }
    }
}

pub fn run() -> anyhow::Result<()> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        env_logger::init();
    }
    #[cfg(target_arch = "wasm32")]
    {
        console_log::init_with_level(log::Level::Info).unwrap_throw();
    }

    let event_loop = EventLoop::with_user_event().build()?;
    #[cfg(not(target_arch = "wasm32"))]
    {
        let mut app = App::new();
        event_loop.run_app(&mut app)?;
    }
    #[cfg(target_arch = "wasm32")]
    {
        let app = App::new(&event_loop);
        event_loop.spawn_app(app);
    }

    Ok(())
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn run_web() -> Result<(), wasm_bindgen::JsValue> {
    console_error_panic_hook::set_once();
    run().unwrap_throw();
    Ok(())
}
