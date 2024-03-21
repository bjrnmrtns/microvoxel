// uniform vertex shader
struct mvp_uniform {
    projection: mat4x4<f32>,
    view: mat4x4<f32>,
    world: mat4x4<f32>,
};

@group(0) @binding(0)
var<uniform> mvp: mvp_uniform;

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
    return vec4<f32>(1.0, 0.0, 0.0, 1.0);
}
