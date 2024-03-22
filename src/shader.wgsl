// uniform vertex shader
struct mvp_uniform {
    projection: mat4x4<f32>,
    view: mat4x4<f32>,
    world: mat4x4<f32>,
};

// lattice used in fragment shader
struct Lattice {
    data: array<u32>,
};

@group(0) @binding(0)
var<uniform> mvp: mvp_uniform;

@group(0) @binding(1)
var<storage, read> lattice : Lattice;

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
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> @builtin(position) vec4<f32> {
    var direction = vec3f(0.0f, -1.0f, 0.0f);
    // 2 3 6 7
    // 2 3 6 3 7 6
    var top_face = array(
      vec3f(-1.0, 1.0, 1.0), //2
      vec3f(1.0, 1.0, 1.0), //3
      vec3f(-1.0, 1.0, -1.0), //6
      vec3f(1.0, 1.0, 1.0), //3
      vec3f(1.0, 1.0, -1.0), //7
      vec3f(-1.0, 1.0, -1.0), //6
    );
    return mvp.projection * mvp.view * mvp.world * vec4<f32>(top_face[in_vertex_index % 6].x, -2.0f, top_face[in_vertex_index % 6].z, 1.0);
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
