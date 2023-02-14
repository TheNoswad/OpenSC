// struct VertexOut {
//     @location(0) color: vec4<f32>,
//     @builtin(position) position: vec4<f32>,
// };

// struct Uniforms {
//     @size(16) angle: f32, // pad to 16 bytes
// };

// @group(0) @binding(0)
// var<uniform> uniforms: Uniforms;

// var<private> v_positions: array<vec2<f32>, 3> = array<vec2<f32>, 3>(
//     vec2<f32>(0.0, 1.0),
//     vec2<f32>(1.0, -1.0),
//     vec2<f32>(-1.0, -1.0),
// );

// var<private> v_colors: array<vec4<f32>, 3> = array<vec4<f32>, 3>(
//     vec4<f32>(1.0, 0.0, 0.0, 1.0),
//     vec4<f32>(0.0, 1.0, 0.0, 1.0),
//     vec4<f32>(0.0, 0.0, 1.0, 1.0),
// );

// @vertex
// fn vs_main(@builtin(vertex_index) v_idx: u32) -> VertexOut {
//     var out: VertexOut;

//     out.position = vec4<f32>(v_positions[v_idx], 0.0, 1.0);
//     out.position.x = out.position.x * cos(uniforms.angle);
//     out.color = v_colors[v_idx];

//     return out;
// }

// @fragment
// fn fs_main(in: VertexOut) -> @location(0) vec4<f32> {
//     return in.color;
// }





// Vertex shader

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.color = model.color;
    out.clip_position = vec4<f32>(model.position, 1.0);
    return out;
}

// Fragment shader

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}
