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
    size: vec4f,
};

struct LatticeHeaders {
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
    var size_x : u32 = 1024u;
    var size_y : u32 = 128u;
    var size_z : u32 = 1024u;
    var index = x + (z * size_x) + (y * size_x * size_z);
    return lattice_get_index(index); 
}

@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32, @builtin(instance_index) in_instance_index: u32) -> @builtin(position) vec4<f32> {
    var face_nr : u32 = in_vertex_index / 6;
    var lattice_header = lattice_headers.data[in_instance_index];
    var face_scaled = lattice_header.face[in_vertex_index % 6].xyz * (lattice_header.size / 2.0).xyz;
    return mvp.projection * mvp.view * mvp.world * vec4f((face_scaled.xyz + lattice_header.start.xyz + (lattice_header.step.xyz * f32(face_nr))), 1.0);
}

@fragment
fn fs_main() -> @location(0) vec4<f32> {
    var colors : array<vec4<f32>, 5> = array<vec4<f32>, 5>(
        vec4<f32>(1.0, 0.0, 0.0, 1.0),  // Red
        vec4<f32>(0.0, 1.0, 0.0, 1.0),  // Green
        vec4<f32>(0.0, 0.0, 1.0, 1.0),  // Blue
        vec4<f32>(1.0, 1.0, 0.0, 1.0),  // Yellow
        vec4<f32>(0.0, 1.0, 1.0, 1.0)   // Cyan
    );
    return colors[lattice_get(23u, 94u, 122u)];
}
