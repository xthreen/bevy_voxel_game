#import bevy_ui::ui_vertex_output::UiVertexOutput

@group(1) @binding(0) var texture: texture_2d<f32>;
@group(1) @binding(1) var texture_sampler: sampler;

@group(1) @binding(2)
var<uniform> north: f32;

@group(1) @binding(3)
var<uniform> dir: f32;

@group(1) @binding(4)
var<uniform> alpha: f32;

@group(1) @binding(5)
var<uniform> tau: f32;

@group(1) @binding(6)
var<uniform> fade_width: f32;

@fragment
fn fragment(in: UiVertexOutput) -> @location(0) vec4<f32> {
    let u_offset = (north - dir) / tau;
    let wrapped_u = (in.uv.x + u_offset % 1.0 + 1.0) % 1.0;
    let uv = vec2<f32>(wrapped_u, in.uv.y);

    var color = textureSample(texture, texture_sampler, uv);

    let fade = smoothstep(0.0, fade_width, in.uv.x)
             * smoothstep(1.0, 1.0 - fade_width, in.uv.x);

    color.a *= alpha * fade;

    return color;
}
