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

struct Lattice {
    pub data: Vec<u32>,
    size_x: usize,
    size_y: usize,
    size_z: usize,
}
impl Lattice {
    pub fn new(size_x: usize, size_y: usize, size_z: usize) -> Self {
        Self {
            data: vec!(0; size_x * size_y * size_z),
            size_x,
            size_y,
            size_z,
        }
    }
/* uncomment code when we want to use 8 bits index into palette
 * pub fn set_index(&mut self, index: usize, value: u8) {
        let array_index = index / 4;
        let u32_index = index % 4;
        let shift = 8 * (3 - u32_index);
        let voxel = self.data[array_index] & !(0xFF << shift); // zero 8 bits
        self.data[array_index] = voxel | (value as u32) << shift; 
    }
    pub fn set(&mut self, x: usize, y: usize, z: usize, value: u8) {
        let index = x + (z * self.size_x) + (y * self.size_x * self.size_z);
        self.set_index(index, value); 
    }
    */
    pub fn set(&mut self, x: usize, y: usize, z: usize, value: u32) {
        let index = x + (z * self.size_x) + (y * self.size_x * self.size_z);
        self.data[index] = value; 
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
struct LatticeHeader {
    face: [glam::Vec4; 6],
    start: glam::Vec4,
    step: glam::Vec4,
}

impl LatticeHeader {
    pub fn new(face: [glam::Vec4; 6], start: glam::Vec4, step: glam::Vec4) -> Self {
        Self {
            face,
            start,
            step,
        }
    }
}

unsafe impl bytemuck::Pod for LatticeHeader {}
unsafe impl bytemuck::Zeroable for LatticeHeader {}

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
struct LatticeHeaders
{
    pub size_x: u32,
    pub size_y: u32,
    pub size_z: u32,
    headers: [LatticeHeader; 3],
}

fn cube_vertex(index: usize) -> glam::Vec4 {
    let cube_vertices = [ 
        glam::Vec4::new(-0.5,-0.5, 0.5, 1.0),
        glam::Vec4::new( 0.5,-0.5, 0.5, 1.0),
        glam::Vec4::new(-0.5, 0.5, 0.5, 1.0),
        glam::Vec4::new( 0.5, 0.5, 0.5, 1.0),
        glam::Vec4::new(-0.5,-0.5,-0.5, 1.0),
        glam::Vec4::new( 0.5,-0.5,-0.5, 1.0),
        glam::Vec4::new(-0.5, 0.5,-0.5, 1.0),
        glam::Vec4::new( 0.5, 0.5,-0.5, 1.0),
    ];
    cube_vertices[index]
}

fn face_x_min() -> [glam::Vec4; 6] {
    [
        cube_vertex(4),
        cube_vertex(2),
        cube_vertex(6),
        cube_vertex(0),
        cube_vertex(2),
        cube_vertex(6),
    ]
}

fn face_x_plus() -> [glam::Vec4; 6] {
    [
        cube_vertex(1),
        cube_vertex(5),
        cube_vertex(3),
        cube_vertex(5),
        cube_vertex(7),
        cube_vertex(3),
    ]
}

fn face_y_min() -> [glam::Vec4; 6] {
    [
        cube_vertex(4),
        cube_vertex(5),
        cube_vertex(0),
        cube_vertex(5),
        cube_vertex(1),
        cube_vertex(0),
    ]
}

fn face_y_plus() -> [glam::Vec4; 6] {
    [
        cube_vertex(2),
        cube_vertex(3),
        cube_vertex(6),
        cube_vertex(3),
        cube_vertex(7),
        cube_vertex(6),
    ]
}

fn face_z_min() -> [glam::Vec4; 6] {
    [
        cube_vertex(5),
        cube_vertex(4),
        cube_vertex(7),
        cube_vertex(4),
        cube_vertex(6),
        cube_vertex(5),
    ]
}

fn face_z_plus() -> [glam::Vec4; 6] {
    [
        cube_vertex(0),
        cube_vertex(1),
        cube_vertex(2),
        cube_vertex(1),
        cube_vertex(3),
        cube_vertex(2),
    ]
}

impl LatticeHeaders {
    pub fn new(size_x: u32, size_y: u32, size_z: u32) -> Self {
        let start = glam::Vec4::new((size_x as f32 / 2.0), (size_y as f32 / 2.0), (size_z as f32 / 2.0), 0.0);
        Self {
            size_x,
            size_y,
            size_z,
            headers: [LatticeHeader::new(face_y_plus(), start, glam::Vec4::new(0.0, -1.0, 0.0, 0.0)),
                      LatticeHeader::new(face_z_plus(), start, glam::Vec4::new(0.0, 0.0, -1.0, 0.0)),
                      LatticeHeader::new(face_x_plus(), start, glam::Vec4::new(-1.0, 0.0, 0.0, 0.0))
            ],
        }
    }
}

unsafe impl bytemuck::Pod for LatticeHeaders {}
unsafe impl bytemuck::Zeroable for LatticeHeaders {}


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

    let size_x : u32 = 2;//1024;
    let size_y : u32 = 2;//128;
    let size_z : u32 = 2;//1024;
    let mut lattice = Lattice::new(size_x as usize, size_y as usize, size_z as usize);
    let lattice_headers = LatticeHeaders::new(size_x, size_y, size_z);

    lattice.set(0, 0, 0, 0x00FFFFFF);
    lattice.set(0, 0, 1, 0x00FFFF00);
    lattice.set(0, 1, 0, 0x00FF00FF);
    lattice.set(0, 1, 1, 0x00FF0000);
    lattice.set(1, 0, 0, 0x0000FFFF);
    lattice.set(1, 0, 1, 0x0000FF00);
    lattice.set(1, 1, 0, 0x000000FF);
    lattice.set(1, 1, 1, 0x00000000);
    let mut last_mouse_position : Option<(f32, f32)> = None;
    let mut current_mouse_position : Option<(f32, f32)> = None;
    mvp_uniform.projection = glam::Mat4::perspective_rh(45.0, window.inner_size().width as f32 / window.inner_size().height as f32, 1.0, 1000.0 );

    let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Uniform buffer"),
        contents: bytemuck::cast_slice(&[mvp_uniform]),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });

    let lattice_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("lattice buffer"),
        contents: bytemuck::cast_slice(lattice.data.as_slice()),
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
    });
    
    let lattice_header_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("lattice header buffer"),
        contents: bytemuck::cast_slice(&[lattice_headers]),
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
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
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Storage { read_only: true }, has_dynamic_offset: false, min_binding_size: None },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Storage { read_only: true }, has_dynamic_offset: false, min_binding_size: None },
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
                
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: lattice_buffer.as_entire_binding(),
                
            },
            wgpu::BindGroupEntry {
                binding: 2,
                resource: lattice_header_buffer.as_entire_binding(),
                
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
            compilation_options: wgpu::PipelineCompilationOptions::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: "fs_main",
            targets: &[Some(swapchain_format.into())],
            compilation_options: wgpu::PipelineCompilationOptions::default(),
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
            ..Default::default()
        },
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
                                            load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                                            store: wgpu::StoreOp::Store,
                                        },
                                    })],
                                    depth_stencil_attachment: None,
                                    timestamp_writes: None,
                                    occlusion_query_set: None,
                                });
                            rpass.set_pipeline(&render_pipeline);
                            rpass.set_bind_group(0, &mvp_bind_group, &[]);
                            rpass.draw(0..lattice_headers.size_y * 6, 0..1);
                            rpass.draw(0..lattice_headers.size_z * 6, 1..2);
                            rpass.draw(0..lattice_headers.size_x * 6, 2..3);

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
