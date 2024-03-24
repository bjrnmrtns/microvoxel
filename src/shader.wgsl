// uniform vertex shader
struct mvp_uniform {
    projection: mat4x4<f32>,
    view: mat4x4<f32>,
    world: mat4x4<f32>,
};

struct LatticeHeader {
    size_x : u32,
    size_y : u32,
    size_z : u32,
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
var<storage, read> lattice_header : LatticeHeader;

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

// first render y than z then x
// 0..size_y , size_y..size_y + size_z, size_y + size_z..size_y + size_z + size_x
// 0..127 , 128..1152, 1152..2176 
// return 0 -> y-side, 1 -> z-side, 2 -> y-side
fn lattice_vertex_index_to_side(vertex_index: u32) -> u32 {
    var face_index : u32 = vertex_index / 6;    
    var lattice_direction : u32 = u32((face_index / lattice_header.size_y) > 0) + u32((face_index / (lattice_header.size_y + lattice_header.size_z)) > 0);
    return lattice_direction;
}

struct Face
{
    face : array<vec3f, 6>,
    start: vec3f,
    step: vec3f,
    // only denoting which axis need to be scaled
    scale: vec3f,
};

const lattice_info = array<Face, 3>(
    Face(array(
      vec3f(-1.0, 1.0, 1.0), //2
      vec3f(1.0, 1.0, 1.0), //3
      vec3f(-1.0, 1.0, -1.0), //6
      vec3f(1.0, 1.0, 1.0), //3
      vec3f(1.0, 1.0, -1.0), //7
      vec3f(-1.0, 1.0, -1.0), //6
    ),
      vec3f(0.0f, 0.0f, 0.0f),
      vec3f(0.0f, -1.0f, 0.0f),
      vec3f(1.0f, 0.0f, 1.0f),
    ),
    Face(array(
      vec3f(-1.0, 1.0, 1.0), //2
      vec3f(1.0, 1.0, 1.0), //3
      vec3f(-1.0, 1.0, -1.0), //6
      vec3f(1.0, 1.0, 1.0), //3
      vec3f(1.0, 1.0, -1.0), //7
      vec3f(-1.0, 1.0, -1.0), //6
    ), 
      vec3f(0.0f, 0.0f, 0.0f),
      vec3f(0.0f, -1.0f, 0.0f),
      vec3f(1.0f, 0.0f, 1.0f),
    ),
    Face(array(
      vec3f(-1.0, 1.0, 1.0), //2
      vec3f(1.0, 1.0, 1.0), //3
      vec3f(-1.0, 1.0, -1.0), //6
      vec3f(1.0, 1.0, 1.0), //3
      vec3f(1.0, 1.0, -1.0), //7
      vec3f(-1.0, 1.0, -1.0), //6
    ), 
      vec3f(0.0f, 0.0f, 0.0f),
      vec3f(0.0f, -1.0f, 0.0f),
      vec3f(1.0f, 0.0f, 1.0f),
    ),
);

@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> @builtin(position) vec4<f32> {
    var side = lattice_vertex_index_to_side(in_vertex_index);
    var face = lattice_info[0].face;
    var start = lattice_info[0].start;
    var step = lattice_info[0].step;
    var face_index : u32 = in_vertex_index / 6;
    return mvp.projection * mvp.view * mvp.world * vec4<f32>(face[in_vertex_index % 6] + start + step * f32(face_index), 1.0);
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
