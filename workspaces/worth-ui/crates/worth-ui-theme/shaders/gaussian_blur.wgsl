// ── Separable Gaussian Blur ───────────────────────────────────────────────────
//
// Two-pass separable blur for the backdrop glass effect.
// Pass 1 (horizontal): samples `input_texture` along X, writes to temp.
// Pass 2 (vertical):   samples temp along Y, writes to blurred output.
//
// Uniforms:
//   direction: vec2<f32> — (1, 0) for horizontal, (0, 1) for vertical
//   texel_size: vec2<f32> — 1/width, 1/height

struct BlurUniforms {
    direction:  vec2<f32>,
    texel_size: vec2<f32>,
};

@group(0) @binding(0)
var<uniform> blur_params: BlurUniforms;

@group(0) @binding(1)
var input_texture: texture_2d<f32>;

@group(0) @binding(2)
var input_sampler: sampler;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0)       uv:       vec2<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) idx: u32) -> VertexOutput {
    var out: VertexOutput;
    // Fullscreen triangle
    let x = f32(i32(idx) / 2) * 4.0 - 1.0;
    let y = f32(i32(idx) % 2) * 4.0 - 1.0;
    out.position = vec4<f32>(x, y, 0.0, 1.0);

    // Convert clip space to UV [0, 1]
    out.uv = (vec2<f32>(x, -y) + 1.0) * 0.5;
    return out;
}

// 13-tap Gaussian weights (sigma ≈ 3)
// Precomputed with: w[i] = exp(-0.5 * (i/sigma)^2), normalized
const OFFSETS: array<f32, 7> = array<f32, 7>(
    0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0
);
const WEIGHTS: array<f32, 7> = array<f32, 7>(
    0.2270270270,
    0.1945945946,
    0.1216216216,
    0.0540540541,
    0.0162162162,
    0.0030030030,
    0.0003003003
);

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let step = blur_params.direction * blur_params.texel_size;
    var result = textureSample(input_texture, input_sampler, in.uv) * WEIGHTS[0];

    for (var i = 1u; i < 7u; i = i + 1u) {
        let offset = OFFSETS[i] * step;
        result = result + textureSample(input_texture, input_sampler, in.uv + offset) * WEIGHTS[i];
        result = result + textureSample(input_texture, input_sampler, in.uv - offset) * WEIGHTS[i];
    }

    return result;
}
