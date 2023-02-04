use crate::window::Window;

pub struct GraphicContext {
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
}

impl GraphicContext {
    pub async fn new(window: &Window) -> GraphicContext {
        let size = &window.window.inner_size();

        // The instance is a handle to our GPU
        let instance = wgpu::Instance::new(
            wgpu::InstanceDescriptor {
                backends: wgpu::Backends::all(),
                dx12_shader_compiler: wgpu::Dx12Compiler::Fxc,
            }
            // the code above also equals wgpu::Instance::Default();
        );

        let surface = unsafe { instance.create_surface(&window.window).expect("Surface cannot be created!") };

        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            }).await.unwrap();

        // Select a device to use
        let (device, queue) = adapter.request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                },
                None,
            ).await.unwrap();

        // Config for surface

        let caps = surface.get_capabilities(&adapter);
        let formats = caps.formats;
        let present_modes = caps.present_modes;
        let alpha_modes = caps.alpha_modes;

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: formats[0],
            width: size.width,
            height: size.height,
            present_mode: Default::default(),
            alpha_mode: Default::default(),
            view_formats: vec![wgpu::TextureFormat::Bgra8UnormSrgb],
        };
        surface.configure(&device, &config);

        GraphicContext {
            surface,
            device,
            queue,
            config,
        }
    }
}
