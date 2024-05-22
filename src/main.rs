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
struct LatticeHeaders
{
    pub size_x: u32,
    pub size_y: u32,
    pub size_z: u32,
}

fn cube_vertex(index: usize, size: &[usize; 3], offset: &[f32; 3]) -> [f32; 3] {
    let cube_vertices = [ 
        [-1,-1, 1],
        [ 1,-1, 1],
        [-1, 1, 1],
        [ 1, 1, 1],
        [-1,-1,-1],
        [ 1,-1,-1],
        [-1, 1,-1],
        [ 1, 1,-1],
    ];
    [cube_vertices[index][0] as f32 * size[0] as f32 / 2.0 + offset[0], cube_vertices[index][1] as f32 * size[1] as f32 / 2.0 + offset[1], cube_vertices[index][2] as f32 * size[2] as f32 / 2.0 + offset[2]]
}
        
fn face_x_min(size: &[usize; 3], offset: &[f32; 3]) -> [[f32; 3]; 6] {
    let size = [0, size[1], size[2]];
    let offset = [offset[0], offset[1], offset[2]];
    [
        cube_vertex(4, &size, &offset),
        cube_vertex(2, &size, &offset),
        cube_vertex(6, &size, &offset),
        cube_vertex(0, &size, &offset),
        cube_vertex(2, &size, &offset),
        cube_vertex(4, &size, &offset),
    ]
}

fn face_x_plus(size: &[usize; 3], offset: &[f32; 3]) -> [[f32; 3]; 6] {
    let size = [0, size[1], size[2]];
    let offset = [offset[0], offset[1], offset[2]];
    [
        cube_vertex(1, &size, &offset),
        cube_vertex(5, &size, &offset),
        cube_vertex(3, &size, &offset),
        cube_vertex(5, &size, &offset),
        cube_vertex(7, &size, &offset),
        cube_vertex(3, &size, &offset),
    ]
}

fn face_y_min(size: &[usize; 3], offset: &[f32; 3]) -> [[f32; 3]; 6] {
    let size = [size[0], 0, size[2]];
    let offset = [offset[0], offset[1], offset[2]];
    [
        cube_vertex(4, &size, &offset), 
        cube_vertex(5, &size, &offset),
        cube_vertex(0, &size, &offset),
        cube_vertex(5, &size, &offset),
        cube_vertex(1, &size, &offset),
        cube_vertex(0, &size, &offset),
    ]
}

fn face_y_plus(size: &[usize; 3], offset: &[f32; 3]) -> [[f32; 3]; 6] {
    let size = [size[0], 0, size[2]];
    let offset = [offset[0], offset[1], offset[2]];
    [
        cube_vertex(2, &size, &offset),
        cube_vertex(3, &size, &offset),
        cube_vertex(6, &size, &offset),
        cube_vertex(3, &size, &offset),
        cube_vertex(7, &size, &offset),
        cube_vertex(6, &size, &offset),
    ]
}

fn face_z_min(size: &[usize; 3], offset: &[f32; 3]) -> [[f32; 3]; 6] {
    let size = [size[0], size[1], 0];
    let offset = [offset[0], offset[1], offset[2]];
    [
        cube_vertex(5, &size, &offset),
        cube_vertex(4, &size, &offset),
        cube_vertex(7, &size, &offset),
        cube_vertex(4, &size, &offset),
        cube_vertex(6, &size, &offset),
        cube_vertex(7, &size, &offset),
    ]
}

fn face_z_plus(size: &[usize; 3], offset: &[f32; 3]) -> [[f32; 3]; 6] {
    let size = [size[0], size[1], 0];
    let offset = [offset[0], offset[1], offset[2]];
    [
        cube_vertex(0, &size, &offset),
        cube_vertex(1, &size, &offset),
        cube_vertex(2, &size, &offset),
        cube_vertex(1, &size, &offset),
        cube_vertex(3, &size, &offset),
        cube_vertex(2, &size, &offset),
    ]
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

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 3],
}

fn create_depth_texture(device: &wgpu::Device, width: u32, height: u32) -> wgpu::TextureView {
    let size = wgpu::Extent3d {
        width: width,
        height: height,
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

    let size_x = 3;//1024;
    let size_y = 3;//128;
    let size_z = 3;//1024;
    let mut vertices : Vec<[f32; 3]> = Vec::new();

    for y in 0..size_y {
        let offset_y = -(size_y as f32 / 2.0) + y as f32;
        face_y_plus(&[size_x, size_y, size_z], &[0.0, offset_y, 0.0]).map(|p| vertices.push(p));
    }
    for x in 0..size_x {
        let offset_x = -(size_x as f32 / 2.0) + x as f32;
        face_x_plus(&[size_x, size_y, size_z], &[offset_x, 0.0, 0.0]).map(|p| vertices.push(p));
    }
    for z in 0..size_z {
        let offset_z = -(size_z as f32 / 2.0) + z as f32;
        face_z_plus(&[size_x, size_y, size_z], &[0.0, 0.0, offset_z]).map(|p| vertices.push(p));
    }
    
    for y in 0..size_y {
        let offset_y = (size_y as f32 / 2.0) - y as f32;
        face_y_min(&[size_x, size_y, size_z], &[0.0, offset_y, 0.0]).map(|p| vertices.push(p));
    }
    for x in 0..size_x {
        let offset_x = (size_x as f32 / 2.0) - x as f32;
        face_x_min(&[size_x, size_y, size_z], &[offset_x, 0.0, 0.0]).map(|p| vertices.push(p));
    }
    for z in 0..size_z {
        let offset_z = (size_z as f32 / 2.0) - z as f32;
        face_z_min(&[size_x, size_y, size_z], &[0.0, 0.0, offset_z]).map(|p| vertices.push(p));
    }

    let mut lattice = Lattice::new(size_x, size_y, size_z);
    let lattice_headers = LatticeHeaders::new(size_x as u32, size_y as u32, size_z as u32);

    for x in 0..3 {
    for y in 0..3 {
    for z in 0..3 {
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
        let c = 0xFF000000 + r + g + b;
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
                array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
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

    let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Vertex Buffer"),
        contents: bytemuck::cast_slice(&vertices),
        usage: wgpu::BufferUsages::VERTEX,
    });

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
                            rpass.set_pipeline(&render_pipeline);
                            rpass.set_bind_group(0, &mvp_bind_group, &[]);
                            rpass.set_vertex_buffer(0, vertex_buffer.slice(..));
                            rpass.draw(0..vertices.len() as u32, 0..1);
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
