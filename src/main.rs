mod camera;

use std::{borrow::Cow, time::Instant};

use wgpu::util::DeviceExt;
use winit::{
    event::{self, Event, WindowEvent}, event_loop::EventLoop, keyboard::PhysicalKey, window::{Window, WindowBuilder}
};

struct Fps {
    size: usize,
    steps: usize,
    current: usize,
    time_points: Vec<Instant>,
}

impl Fps {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            steps: size - 1,
            current: 0,
            time_points: vec![Instant::now(); size],
        }
    }

    pub fn add_timepoint(&mut self) {
        self.time_points[self.current] = Instant::now();
        self.current = (self.current + 1) % self.size;
    }

    pub fn value(&self) -> usize {
        let mut microseconds = 0;
        for i in 0..self.steps { // nine steps
            let first = (self.current  + i) % self.size;
            let second = (self.current + i + 1) % self.size;
            let duration = self.time_points[second].duration_since(self.time_points[first]);
            microseconds += duration.as_micros();
        }
        (1000000.0 / (microseconds as f32 / self.steps as f32)) as usize // nine steps
    }
}

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
struct LatticeHeaders
{
    pub size_x: u32,
    pub size_y: u32,
    pub size_z: u32,
}

fn cube_face(axis: usize, indices: &[usize; 6]) -> [[u32; 3]; 6] {
    const PLANE_VERTICES_X : [[u32; 3]; 4] = [
        [0, 0, 0],
        [0, 0, 1],
        [0, 1, 0],
        [0, 1, 1],
    ];
    const PLANE_VERTICES_Y : [[u32; 3]; 4]  = [
        [0, 0, 0],
        [1, 0, 0],
        [0, 0, 1],
        [1, 0, 1],
    ];
    const PLANE_VERTICES_Z : [[u32; 3]; 4]  = [
        [1, 0, 0],
        [0, 0, 0],
        [1, 1, 0],
        [0, 1, 0],
    ];
    const CUBE_VERTICES : [[[u32; 3]; 4]; 3]  = [
        PLANE_VERTICES_X,
        PLANE_VERTICES_Y,
        PLANE_VERTICES_Z,
    ];
    [
        CUBE_VERTICES[axis][indices[0]],
        CUBE_VERTICES[axis][indices[1]],
        CUBE_VERTICES[axis][indices[2]],
        CUBE_VERTICES[axis][indices[3]],
        CUBE_VERTICES[axis][indices[4]],
        CUBE_VERTICES[axis][indices[5]],
    ]
}

const PLUS_WINDING : [usize; 6] = [1, 2, 3, 0, 2, 1]; 
const MIN_WINDING : [usize; 6] = [0, 1, 2, 1, 3, 2]; 

fn cube_x_min() -> [[u32; 3]; 6] {
    cube_face(0, &MIN_WINDING)
}

fn cube_x_plus() -> [[u32; 3]; 6] {
    cube_face(0, &PLUS_WINDING)
}

fn cube_y_min() -> [[u32; 3]; 6] {
    cube_face(1, &MIN_WINDING)
}

fn cube_y_plus() -> [[u32; 3]; 6] {
    cube_face(1, &PLUS_WINDING)
}

fn cube_z_min() -> [[u32; 3]; 6] {
    cube_face(2, &MIN_WINDING)
}

fn cube_z_plus() -> [[u32; 3]; 6] {
    cube_face(2, &PLUS_WINDING)
}

fn cube_scale(input: [u32; 3], scale: [u32; 3]) -> [u32; 3] {
    [input[0] * scale[0], input[1] * scale[1], input[2] * scale[2]]
}

fn cube_offset(input: [u32; 3], offset: [u32; 3]) -> [u32; 3] {
    [input[0] + offset[0], input[1] + offset[1], input[2] + offset[2]]
}

fn cube_correct_and_to_float(input: [u32; 3], offset: [f32; 3]) -> [f32; 3] {
    [input[0] as f32 + offset[0], input[1] as f32 + offset[1], input[2] as f32 + offset[2]]
}

impl LatticeHeaders {
    pub fn new(size_x: u32, size_y: u32, size_z: u32) -> Self {
        Self {
            size_x,
            size_y,
            size_z,
        }
    }
}

unsafe impl bytemuck::Pod for LatticeHeaders {}
unsafe impl bytemuck::Zeroable for LatticeHeaders {}

fn create_depth_texture(device: &wgpu::Device, width: u32, height: u32) -> wgpu::TextureView {
    let size = wgpu::Extent3d {
        width,
        height,
        depth_or_array_layers: 1,
    };

    let desc = wgpu::TextureDescriptor {
        label: Some("Depth Texture"),
        size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Depth32Float,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
        view_formats: &[],
    };

    let texture = device.create_texture(&desc);
    texture.create_view(&wgpu::TextureViewDescriptor::default())
}

async fn run(event_loop: EventLoop<()>, window: Window) {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::GL,
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

    let size_x = 128;//1024;
    let size_y = 32;//128;
    let size_z = 128;//1024;
    let mut vertices_y_min : Vec<[f32; 3]> = Vec::new();
    let mut vertices_x_min : Vec<[f32; 3]> = Vec::new();
    let mut vertices_z_min : Vec<[f32; 3]> = Vec::new();
    let mut vertices_y_plus : Vec<[f32; 3]> = Vec::new();
    let mut vertices_x_plus : Vec<[f32; 3]> = Vec::new();
    let mut vertices_z_plus : Vec<[f32; 3]> = Vec::new();

    for y in 0..size_y {
        let y = size_y - y;
        cube_y_plus().map(|p| vertices_y_plus.push(
                cube_correct_and_to_float(
                    cube_offset(cube_scale(
                            p, [size_x as u32, size_y as u32, size_z as u32]), 
                               [0, y as u32, 0]),
                               [0.0, -0.001, 0.0])
                ));
    }
    for y in 0..size_y {
        cube_y_min().map(|p| vertices_y_min.push(
                cube_correct_and_to_float(
                    cube_offset(cube_scale(
                            p, [size_x as u32, size_y as u32, size_z as u32]), 
                               [0, y as u32, 0]),
                               [0.0, 0.001, 0.0])
                ));
    }
    for x in 0..size_x {
        let x = size_x - x;
        cube_x_plus().map(|p| vertices_x_plus.push(
                cube_correct_and_to_float(
                    cube_offset(cube_scale(
                            p, [size_x as u32, size_y as u32, size_z as u32]), 
                               [x as u32, 0, 0]),
                               [-0.001, 0.0, 0.0])
                ));
    }
    for x in 0..size_x {
        cube_x_min().map(|p| vertices_x_min.push(
                cube_correct_and_to_float(
                    cube_offset(cube_scale(
                            p, [size_x as u32, size_y as u32, size_z as u32]), 
                               [x as u32, 0, 0]),
                               [0.001, 0.0, 0.0])
                ));
    }
    for z in 0..size_z {
        let z = size_z - z;
        cube_z_plus().map(|p| vertices_z_plus.push(
                cube_correct_and_to_float(
                    cube_offset(cube_scale(
                            p, [size_x as u32, size_y as u32, size_z as u32]), 
                               [0, 0, z as u32]),
                               [0.0, 0.0, -0.001])
                ));
    }
    for z in 0..size_z {
        cube_z_min().map(|p| vertices_z_min.push(
                cube_correct_and_to_float(
                    cube_offset(cube_scale(
                            p, [size_x as u32, size_y as u32, size_z as u32]), 
                               [0, 0, z as u32]),
                               [0.0, 0.0, 0.001])
                ));
    }

    let mut lattice = Lattice::new(size_x, size_y, size_z);
    let lattice_headers = LatticeHeaders::new(size_x as u32, size_y as u32, size_z as u32);

    for x in 0..size_x {
    for y in 0..size_y {
    for z in 0..size_z {
        let r = if x % 2 == 0 {
            0x000000FF
        } else {
            0x00000000
        };
        let g = if y % 2 == 0 {
            0x0000FF00
        } else {
            0x00000000
        };
        let b = if z % 2 == 0 {
            0x00FF0000
        } else {
            0x00000000
        };
        let c = 0xBB000000 + r + g + b;
        lattice.set(x, y, z, c);
    }
    }
    }

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
    let depth_view = create_depth_texture(&device, window.inner_size().width, window.inner_size().height);

    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: None,
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: "vs_main",
            buffers: &[wgpu::VertexBufferLayout {
                array_stride: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &[
                    wgpu::VertexAttribute {
                        offset: 0,
                        shader_location: 0,
                        format: wgpu::VertexFormat::Float32x3,
                    },
                ],
            }],
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
        depth_stencil: Some(wgpu::DepthStencilState {
            format: wgpu::TextureFormat::Depth32Float,
            depth_write_enabled: true,
            depth_compare: wgpu::CompareFunction::Less,
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        }),
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

    let vertex_buffer_y_min = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Vertex Buffer"),
        contents: bytemuck::cast_slice(&vertices_y_min),
        usage: wgpu::BufferUsages::VERTEX,
    });
    let vertex_buffer_x_min = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Vertex Buffer"),
        contents: bytemuck::cast_slice(&vertices_x_min),
        usage: wgpu::BufferUsages::VERTEX,
    });
    let vertex_buffer_z_min = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Vertex Buffer"),
        contents: bytemuck::cast_slice(&vertices_z_min),
        usage: wgpu::BufferUsages::VERTEX,
    });
    let vertex_buffer_y_plus = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Vertex Buffer"),
        contents: bytemuck::cast_slice(&vertices_y_plus),
        usage: wgpu::BufferUsages::VERTEX,
    });
    let vertex_buffer_x_plus = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Vertex Buffer"),
        contents: bytemuck::cast_slice(&vertices_x_plus),
        usage: wgpu::BufferUsages::VERTEX,
    });
    let vertex_buffer_z_plus = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Vertex Buffer"),
        contents: bytemuck::cast_slice(&vertices_z_plus),
        usage: wgpu::BufferUsages::VERTEX,
    });

    let mut fps = Fps::new(10);

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
                                    depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                                            view: &depth_view,
                                            depth_ops: Some(wgpu::Operations {
                                                load: wgpu::LoadOp::Clear(1.0),
                                                store: wgpu::StoreOp::Store,
                                            }),
                                            stencil_ops: None,
                                        }),
                                    timestamp_writes: None,
                                    occlusion_query_set: None,
                                });
                            fps.add_timepoint();
                            println!("fps: {}", fps.value());
                            rpass.set_pipeline(&render_pipeline);
                            rpass.set_bind_group(0, &mvp_bind_group, &[]);
                            rpass.set_vertex_buffer(0, vertex_buffer_y_min.slice(..));
                            rpass.draw(0..vertices_y_min.len() as u32, 0..1);
                            rpass.set_vertex_buffer(0, vertex_buffer_x_min.slice(..));
                            rpass.draw(0..vertices_x_min.len() as u32, 0..1);
                            rpass.set_vertex_buffer(0, vertex_buffer_z_min.slice(..));
                            rpass.draw(0..vertices_z_min.len() as u32, 0..1);
                            rpass.set_vertex_buffer(0, vertex_buffer_y_plus.slice(..));
                            rpass.draw(0..vertices_y_plus.len() as u32, 3..4);
                            rpass.set_vertex_buffer(0, vertex_buffer_x_plus.slice(..));
                            rpass.draw(0..vertices_x_plus.len() as u32, 0..1);
                            rpass.set_vertex_buffer(0, vertex_buffer_z_plus.slice(..));
                            rpass.draw(0..vertices_z_plus.len() as u32, 0..1);
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
