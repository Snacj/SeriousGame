struct CameraUniform {
    proj_view: mat4x4<f32>,
};
@group(0) @binding(0)
var<uniform> camera: CameraUniform;

struct TimeUniform {
    time: f32,
    _pad0: f32,
    _pad1: f32,
    _pad2: f32,
};
@group(2) @binding(0)
var<uniform> time_data: TimeUniform;

@group(1) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(1) @binding(1)
var s_diffuse: sampler;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
};

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = camera.proj_view * vec4<f32>(model.position, 1.0);
    out.tex_coords = model.tex_coords;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    var color = textureSample(t_diffuse, s_diffuse, in.tex_coords);

    let offset = in.clip_position.x * 0.003 + in.clip_position.y * 0.002;

    // Brightness oscillates between 0.7 and 1.0 at ~2 beats per second
    let pulse = 0.85 + 0.15 * sin(time_data.time * 2.0 + offset);

    // Boost red, dim green and blue keeps it in blood-red territory
    color.r = color.r * pulse;
    color.g = color.g * (pulse * 0.6);
    color.b = color.b * (pulse * 0.4);

    return color;
}
