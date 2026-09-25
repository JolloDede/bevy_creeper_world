#import bevy_ui::ui_vertex_output::UiVertexOutput

@group(1) @binding(0) var<uniform> base_color: vec4<f32>;
@group(1) @binding(1) var<uniform> light_pos: vec4<f32>;

@fragment
fn fragment(in: UiVertexOutput) -> @location(0) vec4<f32> {
    // in.uv gives normalized coordinates (0.0 to 1.0) across the button surface
    let uv = in.uv;

    // Calculate distance from the current pixel to the virtual light source
    let dist = distance(uv, light_pos.xy);

    // Create a smooth radial falloff for the light
    let light_intensity = max(0.0, .5 - dist * 1.8) * 0.6;

    // Add the highlight contribution on top of the base color
    let final_rgb = base_color.rgb + vec3(light_intensity);

    return vec4(final_rgb, base_color.a);
}
