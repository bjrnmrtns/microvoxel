mod camera;

use std::borrow::Cow;

use wgpu::util::DeviceExt;
use winit::{
    event::{self, Event, WindowEvent}, event_loop::EventLoop, keyboard::PhysicalKey, window::{Window, WindowBuilder}
};

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
struct Uniform {
    pub projection: glam::Mat4,
    pub view: glam::Mat4,
    pub world: glam::Mat4,
}

unsafe impl bytemuck::Pod for Uniform {}
unsafe impl bytemuck::Zeroable for Uniform {}

async fn run(event_loop: EventLoop<()>, window: Window) {

    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::PRIMARY,
        ..Default::default()
    });
    let surface = instance.create_surface(&window).unwrap();
    let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::HighPerformance,
        compatible_surface: Some(&surface),
        force_fallback_adapter: false,
    }).await.expect("Failed to find appropriate adapter");

    let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor { label: None, required_features: wgpu::Features::empty(), required_limits: wgpu::Limits::default().using_resolution(adapter.limits()),}, None,).await.expect("Failed to create device");

    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor { label: None, source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shader.wgsl"))),});

    let mut mvp_uniform = Uniform::default();
    let mut last_mouse_position : Option<(f32, f32)> = None;
    let mut current_mouse_position : Option<(f32, f32)> = None;
    mvp_uniform.projection = glam::Mat4::perspective_rh(45.0, window.inner_size().width as f32 / window.inner_size().height as f32, 1.0, 1000.0 );

    let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Uniform buffer"),
        contents: bytemuck::cast_slice(&[mvp_uniform]),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });

    let mvp_bind_group_layout = device.create_bind_group_layout(
        &wgpu::BindGroupLayoutDescriptor {
            label: Some("mvp_bind_group_layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Uniform, has_dynamic_offset: false, min_binding_size: None },
                    count: None,
                },
            ],
        }
    );

    let mvp_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("mpv_bind_group"),
        layout: &mvp_bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
                
            }
        ],
    });

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[&mvp_bind_group_layout],
        push_constant_ranges: &[]
    });

    let swapchain_capabilities = surface.get_capabilities(&adapter);
    let swapchain_format = swapchain_capabilities.formats[0];

    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: None,
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: "vs_main",
            buffers: &[],
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: "fs_main",
            targets: &[Some(swapchain_format.into())],
        }),
        primitive: wgpu::PrimitiveState::default(),
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        multiview: None,
    });

    let mut config = surface
        .get_default_config(&adapter, window.inner_size().width, window.inner_size().height)
        .unwrap();
    surface.configure(&device, &config);

    let window = &window;
    mvp_uniform.projection = glam::Mat4::perspective_rh(45.0, window.inner_size().width as f32 / window.inner_size().height as f32, 1.0, 1000.0 );
    let mut camera = camera::Camera::new();

    event_loop
        .run(move |event, target| {
            // Have the closure take ownership of the resources.
            // `event_loop.run` never returns, therefore we must do this to ensure
            // the resources are properly cleaned up.
            let _ = (&instance, &adapter, &shader, &pipeline_layout);

            if let Event::WindowEvent {
                window_id: _,
                event,
            } = event
            {
                match event {
                    WindowEvent::Resized(new_size) => {
                        // Reconfigure the surface with the new size
                        config.width = new_size.width.max(1);
                        config.height = new_size.height.max(1);
                        surface.configure(&device, &config);
                        // On macos the window needs to be redrawn manually after resizing
                        window.request_redraw();
                    }
                    WindowEvent::RedrawRequested => {
                        mvp_uniform.view = camera.view_matrix();
                        queue.write_buffer(&uniform_buffer, 0, bytemuck::cast_slice(&[mvp_uniform]));

                        let frame = surface
                            .get_current_texture()
                            .expect("Failed to acquire next swap chain texture");
                        let view = frame
                            .texture
                            .create_view(&wgpu::TextureViewDescriptor::default());
                        let mut encoder =
                            device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                                label: None,
                            });
                        {
                            let mut rpass =
                                encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                                    label: None,
                                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                                        view: &view,
                                        resolve_target: None,
                                        ops: wgpu::Operations {
                                            load: wgpu::LoadOp::Clear(wgpu::Color::GREEN),
                                            store: wgpu::StoreOp::Store,
                                        },
                                    })],
                                    depth_stencil_attachment: None,
                                    timestamp_writes: None,
                                    occlusion_query_set: None,
                                });
                            rpass.set_pipeline(&render_pipeline);
                            rpass.set_bind_group(0, &mvp_bind_group, &[]);
                            rpass.draw(0..6, 0..1);
                        }

                        queue.submit(Some(encoder.finish()));
                        frame.present();
                        window.request_redraw();
                    }
                    WindowEvent::KeyboardInput { event: winit::event::KeyEvent { state, logical_key, .. }, .. } => {
                        match logical_key {
                            winit::keyboard::Key::Named(winit::keyboard::NamedKey::Escape) => {
                                target.exit()
                            }
                            keycode => {
                                match keycode {
                                    winit::keyboard::Key::Character(c) => match c.as_str() {
                                        "w" => camera.update(0.1, 0.0, 0.0, 0.0),
                                        "a" => camera.update(0.0, -0.1, 0.0, 0.0),
                                        "s" => camera.update(-0.1, 0.0, 0.0, 0.0),
                                        "d" => camera.update(0.0, 0.1, 0.0, 0.0),
                                        _ => ()
                                    }
                                    _ => ()
                                }
                            }
                            _ => ()
                        }
                    }
                    WindowEvent::CursorMoved { position, .. } => {
                        current_mouse_position = Some((position.x as f32, position.y as f32));
                        if let (Some(last_mouse_position), Some(current_mouse_position)) = (last_mouse_position, current_mouse_position) {
                            let delta = (last_mouse_position.0 - current_mouse_position.0, last_mouse_position.1 - current_mouse_position.1);
                            camera.update(0.0, 0.0, delta.0, -delta.1); 
                        }
                        last_mouse_position = current_mouse_position;
                    }
                    WindowEvent::CloseRequested => target.exit(),
                    _ => {}
                };
            }
        })
        .unwrap();
}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new().with_title("microvoxel").with_resizable(false).build(&event_loop).unwrap();
    
    env_logger::init();
    pollster::block_on(run(event_loop, window));
}