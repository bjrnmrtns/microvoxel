// uniform vertex shader
struct mvp_uniform {
    projection: mat4x4<f32>,
    view: mat4x4<f32>,
    world: mat4x4<f32>,
};

struct LatticeHeader {
    face: array<vec4f, 6>,
    start: vec4f,
    step: vec4f,
};

struct LatticeHeaders {
    size_x: u32,
    size_y: u32,
    size_z: u32,
    data: array<LatticeHeader>,
};

// lattice used in fragment shader
struct Lattice {
    data: array<u32>,
};

@group(0) @binding(0)
var<uniform> mvp: mvp_uniform;

@group(0) @binding(1)
var<storage, read> lattice : Lattice;

@group(0) @binding(2)
var<storage, read> lattice_headers : LatticeHeaders;

fn lattice_get_index(index: u32) -> u32 {
    var array_index = index / 4;
    var u32_index = index % 4;
    return (lattice.data[array_index] >> (8u * (3 - u32_index))) & 0xFFu;
}

fn lattice_get(x: u32, y: u32, z: u32) -> u32 {
    var size_x : u32 = lattice_headers.size_x;
    var size_y : u32 = lattice_headers.size_y;
    var size_z : u32 = lattice_headers.size_z;
    var index = x + (z * size_x) + (y * size_x * size_z);
    return lattice_get_index(index); 
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) vert_pos: vec3<f32>,
}

@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32, @builtin(instance_index) in_instance_index: u32) -> VertexOutput {
    var out: VertexOutput;
    var face_nr : u32 = in_vertex_index / 6;
    var lattice_header = lattice_headers.data[in_instance_index];
    var size = vec3<f32>(f32(lattice_headers.size_x), f32(lattice_headers.size_y), f32(lattice_headers.size_z));
    var face_scaled = lattice_header.face[in_vertex_index % 6].xyz * size.xyz;

    var world_pos = face_scaled.xyz + lattice_header.start.xyz + (lattice_header.step.xyz * f32(face_nr));
    out.clip_position = mvp.projection * mvp.view * mvp.world * vec4f(world_pos, 1.0);
    out.vert_pos = world_pos;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    var colors : array<vec4<f32>, 8> = array<vec4<f32>, 8>(
        vec4<f32>(1.0, 0.0, 0.0, 1.0),  // Red
        vec4<f32>(0.0, 1.0, 0.0, 1.0),  // Green
        vec4<f32>(0.0, 0.0, 1.0, 1.0),  // Blue
        vec4<f32>(1.0, 1.0, 0.0, 1.0),  // Yellow
        vec4<f32>(1.0, 0.0, 0.0, 1.0),  // Cyan
        vec4<f32>(0.0, 0.0, 1.0, 1.0),  // Cyan
        vec4<f32>(0.0, 1.0, 0.0, 1.0),  // Cyan
        vec4<f32>(0.0, 1.0, 1.0, 1.0)   // Cyan
    );
    var lattice_index = in.vert_pos + vec3<f32>(1.5, 1.5, 1.5);
    // I think we need to use in.vert_pos to calculate the index into lattice
    //return colors[lattice_get(23u, 94u, 122u)];
    return vec4<f32>(in.vert_pos, 1.0);
//    return colors[lattice_get(u32(lattice_index.x), u32(lattice_index.y), u32(lattice_index.z))];
}
