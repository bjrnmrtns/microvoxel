// uniform vertex shader
struct mvp_uniform {
    projection: mat4x4<f32>,
    view: mat4x4<f32>,
    world: mat4x4<f32>,
};

struct LatticeHeaders {
    size_x: u32,
    size_y: u32,
    size_z: u32,
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

/* uncomment when using 8 bit with palette
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
*/

fn lattice_get(x: u32, y: u32, z: u32) -> u32 {
    var size_x : u32 = lattice_headers.size_x;
    var size_y : u32 = lattice_headers.size_y;
    var size_z : u32 = lattice_headers.size_z;
    var index = x + (z * size_x) + (y * size_x * size_z);
    return lattice.data[index]; 
}

fn unpack_rgba(color: u32) -> vec4<f32> {
    let r = f32((color & 0x000000FFu)) / 255.0;
    let g = f32((color & 0x0000FF00u) >> 8) / 255.0;
    let b = f32((color & 0x00FF0000u) >> 16) / 255.0;
    let a = f32((color & 0xFF000000u) >> 24) / 255.0;
    return vec4<f32>(r, g, b, a);
}

struct VertexInput {
    @location(0) position: vec3<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) vert_pos: vec3<f32>,
}

@vertex
fn vs_main(input: VertexInput, @builtin(vertex_index) in_vertex_index: u32, @builtin(instance_index) in_instance_index: u32) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = mvp.projection * mvp.view * mvp.world * vec4f(input.position, 1.0);
    out.vert_pos = input.position;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let x = u32(in.vert_pos.x + f32(lattice_headers.size_x) / 2.0);
    let y = u32(in.vert_pos.y + f32(lattice_headers.size_y) / 2.0);
    let z = u32(in.vert_pos.z + f32(lattice_headers.size_z) / 2.0);
    return unpack_rgba(lattice_get(x, y, z));
//    return vec4<f32>((in.vert_pos + 1.5) / 10.0, 1.0);
}
