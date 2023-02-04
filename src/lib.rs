use crate::graphic_context::GraphicContext;
use crate::window::{Window, WindowEvents};

mod window;
mod consts;
mod graphic_context;
mod render_pass;

struct State {
    size: winit::dpi::PhysicalSize<u32>,
    context: GraphicContext,
}

impl State {
    async fn new(window: &Window) -> Self {
        // Save the window size for use later
        let size = window.window.inner_size();

        // Initialize the graphic context
        let ctx = GraphicContext::new(&window).await;

        Self {
            size,
            context: ctx,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.context.config.width = new_size.width;
            self.context.config.height = new_size.height;
            self.context.surface.configure(&self.context.device, &self.context.config);
        }
    }

    pub fn update(&self) {
    }

    pub fn render(&self) -> Result<(), wgpu::SurfaceError> {
        let output = self.context.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.context.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });
        {
            let mut r_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[
                    Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                r: 0.1,
                                g: 0.2,
                                b: 0.3,
                                a: 1.0,
                            }),
                            store: true,
                        },
                    })],
                depth_stencil_attachment: None,
            });
        }
        self.context.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}

pub async fn run() {
    env_logger::init();

    let window = Window::new();
    let mut app = State::new(&window).await;


    window.run(move |event| match event {
        WindowEvents::Resized { width, height } => {
            app.resize(winit::dpi::PhysicalSize { width, height });
        }
        WindowEvents::Draw => {
            app.update();
            app.render().unwrap();
        }
        WindowEvents::Keyboard {
            state,
            virtual_keycode,
        } => {}
    });
}