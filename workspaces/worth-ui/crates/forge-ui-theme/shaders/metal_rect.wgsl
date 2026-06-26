// ── Liquid Glass Fragment Shader ─────────────────────────────────────────────
//
// Uses:
//   - Squircle convex surface profile (Apple's preferred lens shape)
//   - Snell's Law (n1=1.0 air → n2=1.5 glass) for refraction displacement
//   - Sampled blurred backdrop texture at refracted UV coords
//   - Surface-normal-based specular rim highlight
//   - Mouse-reactive specular glint

// Bind group 0: uniforms
struct Uniforms {
    // xy = rect_min (px), zw = rect_size (px)
    rect:       vec4<f32>,
    // rgba base color (linear)
    base_color: vec4<f32>,
    // x = gloss, y = highlight_shift, z = rim_alpha, w = rounding (px)
    params:     vec4<f32>,
    // x = time (s), y = pressed (0/1), z = mouse_x (px), w = mouse_y (px)
    params2:    vec4<f32>,
    // xy = screen_size (px)
    screen:     vec4<f32>,
};

@group(0) @binding(0)
var<uniform> u: Uniforms;

// Bind group 1: blurred backdrop texture
@group(1) @binding(0)
var backdrop_tex: texture_2d<f32>;
@group(1) @binding(1)
var backdrop_sampler: sampler;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) idx: u32) -> VertexOutput {
    var out: VertexOutput;
    let x = f32(i32(idx) / 2) * 4.0 - 1.0;
    let y = f32(i32(idx) % 2) * 4.0 - 1.0;
    out.position = vec4<f32>(x, y, 0.0, 1.0);
    return out;
}

// ── Signed Distance Field ─────────────────────────────────────────────────────

fn sdf_rounded_rect(p: vec2<f32>, half_size: vec2<f32>, radius: f32) -> f32 {
    let r = min(radius, min(half_size.x, half_size.y));
    let q = abs(p) - half_size + vec2<f32>(r, r);
    return length(max(q, vec2<f32>(0.0, 0.0))) + min(max(q.x, q.y), 0.0) - r;
}

// ── Surface Profile ───────────────────────────────────────────────────────────
//
// Convex squircle: height given normalised distance from edge x in [0, 1].
// h(x) = fourth_root(1 - (1-x)^4)  (Apple's preferred lens shape)
//
// Returns height in [0, 1].
fn squircle_height(x: f32) -> f32 {
    let v = 1.0 - pow(1.0 - x, 4.0);
    return pow(max(v, 0.0), 0.25);
}

// Surface normal at position x (derivative via finite diff, rotated -90°)
fn squircle_normal(x: f32) -> vec2<f32> {
    let eps = 0.001;
    let h0 = squircle_height(max(x - eps, 0.0));
    let h1 = squircle_height(min(x + eps, 1.0));
    let dh = (h1 - h0) / (2.0 * eps);
    // Normal = derivative rotated -90°: (-dh, 1) normalized
    return normalize(vec2<f32>(-dh, 1.0));
}

// ── Snell's Law Refraction ────────────────────────────────────────────────────
//
// n1 * sin(θ1) = n2 * sin(θ2)
// incident ray is orthogonal to background plane, so θ1 = angle of surface normal.
// Returns refracted direction as 2D UV offset (in [−1, 1] range).
fn snell_refract(surface_normal: vec2<f32>, n1: f32, n2: f32) -> vec2<f32> {
    // Incident ray is (0, -1) — straight down into the glass.
    let incident = vec2<f32>(0.0, -1.0);

    // sin(θ1) = cross product magnitude of incident and normal (2D)
    let sin_theta1 = abs(surface_normal.x * incident.y - surface_normal.y * incident.x);
    let sin_theta2 = (n1 / n2) * sin_theta1;

    // Clamp to avoid total internal reflection
    let sin_theta2_clamped = min(sin_theta2, 0.999);
    let cos_theta2 = sqrt(1.0 - sin_theta2_clamped * sin_theta2_clamped);

    // The refracted ray bends toward the normal by theta2
    // Displacement direction is along the surface tangent (perpendicular to normal)
    let tangent = vec2<f32>(surface_normal.y, -surface_normal.x);
    return tangent * sin_theta2_clamped + surface_normal * (cos_theta2 - 1.0);
}

// ── Fragment Shader ───────────────────────────────────────────────────────────

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let rect_min   = u.rect.xy;
    let rect_size  = u.rect.zw;
    let screen_px  = u.screen.xy;
    let frag       = in.position.xy;
    let mouse_pos  = u.params2.zw;

    // UV within the button rect [0..1]
    let uv = (frag - rect_min) / rect_size;
    if uv.x < 0.0 || uv.x > 1.0 || uv.y < 0.0 || uv.y > 1.0 {
        discard;
    }

    let gloss    = u.params.x;
    let rounding = u.params.w;
    let pressed  = u.params2.y;

    // ── SDF + AA ────────────────────────────────────────────────────────
    let center    = frag - rect_min - rect_size * 0.5;
    let half_size = rect_size * 0.5;
    let d         = sdf_rounded_rect(center, half_size, rounding);
    let shape_alpha = 1.0 - smoothstep(-0.5, 0.5, d);
    if shape_alpha <= 0.0 { discard; }

    // ── Bezel width in UV space ──────────────────────────────────────────
    // The bezel is the curved edge region of the glass.
    // We define it as the outermost fraction of the button (e.g. 20%).
    let bezel_fraction = 0.22;
    let bevel_px = min(half_size.x, half_size.y) * bezel_fraction;

    // Distance from the nearest edge, in pixels
    let edge_distance_px = -d; // positive inside, negative outside
    let dist_from_edge   = edge_distance_px; // px into the shape

    // Normalized distance into the bezel: 0 at edge, 1 at bezel/flat boundary
    let bezel_t = clamp(dist_from_edge / bevel_px, 0.0, 1.0);

    // ── Surface Normal from Squircle Profile ─────────────────────────────
    // Direction toward nearest edge in 2D
    var edge_dir = normalize(center);
    // Clamp to avoid NaN at exact center
    if length(center) < 0.001 {
        edge_dir = vec2<f32>(0.0, -1.0);
    }

    // Surface normal from the squircle height at this bezel position
    // bezel_t = 0 → at edge (steep normal), bezel_t = 1 → flat interior
    let sn_scalar = squircle_normal(1.0 - bezel_t);
    // The 2D surface normal points inward along edge_dir for .x component
    let surface_normal_2d = vec2<f32>(sn_scalar.x * edge_dir.x + sn_scalar.y * 0.0,
                                     sn_scalar.x * edge_dir.y + sn_scalar.y * 0.0);

    // ── Refraction Displacement (Snell's Law) ────────────────────────────
    // IOR: air=1.0, glass=1.5
    let n_air  = 1.0;
    let n_glass = 1.5;

    // Only apply refraction in the bezel, not the flat interior
    let in_bezel = 1.0 - bezel_t; // 1 at edge, 0 at flat interior

    let refract_dir = snell_refract(normalize(vec2<f32>(sn_scalar.x, sn_scalar.y)), n_air, n_glass);

    // Scale displacement by bezel presence and glass thickness
    let max_displacement_px = 18.0 * gloss;
    let displacement_uv = refract_dir * in_bezel * (max_displacement_px / screen_px);

    // Sample the blurred backdrop at refracted position
    let frag_screen_uv = frag / screen_px;
    // Flip Y because wgpu framebuffer is top-left, but UV expects bottom-left
    let backdrop_uv = vec2<f32>(frag_screen_uv.x + displacement_uv.x,
                                frag_screen_uv.y + displacement_uv.y);
    let backdrop_color = textureSample(backdrop_tex, backdrop_sampler, backdrop_uv).rgb;

    // ── Tint + Base Color Mix ────────────────────────────────────────────
    // Flat interior: mostly base_color glass tint
    // Bezel: mostly refracted backdrop showing through
    let flat_tint = u.base_color.rgb * 0.4;
    let glass_body = mix(flat_tint, backdrop_color, 0.65 + in_bezel * 0.3);
    var color = glass_body;

    // ── Specular Rim (Surface Normal vs Light) ───────────────────────────
    // Light direction: toward mouse, or default top-left
    let light_dir_2d = normalize(mouse_pos - frag);
    let light_dir_3d = normalize(vec3<f32>(light_dir_2d.x, light_dir_2d.y, 0.8));

    // Surface normal in 3D (z=1 = pointing toward viewer)
    let normal_3d = normalize(vec3<f32>(sn_scalar.x * edge_dir.x,
                                        sn_scalar.x * edge_dir.y,
                                        sn_scalar.y));
    let n_dot_l = max(dot(normal_3d, light_dir_3d), 0.0);

    // Bright specular rim in the bezel region
    let rim_spec = pow(n_dot_l, 4.0) * in_bezel * gloss * 0.6;
    color = color + rim_spec;

    // ── Mouse Glint (Tight Specular) ─────────────────────────────────────
    let view_dir_3d = vec3<f32>(0.0, 0.0, 1.0);
    let half_vec_3d = normalize(light_dir_3d + view_dir_3d);
    let n_dot_h = max(dot(normal_3d, half_vec_3d), 0.0);
    let glint = pow(n_dot_h, 32.0) * gloss * 0.4;
    let mouse_dist = length(mouse_pos - frag);
    let mouse_atten = 1.0 / (1.0 + mouse_dist * mouse_dist * 0.000015);
    color = color + glint * mouse_atten;

    // ── Top Edge Highlight ───────────────────────────────────────────────
    let top_rim = smoothstep(3.0 / rect_size.y, 0.0, uv.y);
    let top_rim_mask = in_bezel * top_rim;
    color = mix(color, vec3<f32>(1.0), top_rim_mask * 0.25 * gloss);

    // ── Pressed State ────────────────────────────────────────────────────
    if pressed > 0.5 {
        color = color * 0.85;
    }

    // ── Output ───────────────────────────────────────────────────────────
    color = clamp(color, vec3<f32>(0.0), vec3<f32>(1.5));
    let final_alpha = shape_alpha * max(u.base_color.a, 0.85);
    return vec4<f32>(color * final_alpha, final_alpha);
}
