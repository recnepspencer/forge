pub(super) fn plane_tangent_frame(normal: &[f64; 3]) -> ([f64; 3], [f64; 3]) {
    let seed = if normal[0].abs() < 0.9 {
        [1.0, 0.0, 0.0]
    } else {
        [0.0, 1.0, 0.0]
    };
    let u = normalize(cross(&seed, normal));
    let v = cross(normal, &u);
    (u, v)
}

pub(super) fn cylinder_frame(axis: &[f64; 3]) -> ([f64; 3], [f64; 3]) {
    plane_tangent_frame(axis)
}

pub(super) fn combine_frame(
    axis_u: &[f64; 3],
    axis_v: &[f64; 3],
    axis_w: &[f64; 3],
    local: [f64; 3],
) -> [f64; 3] {
    [
        local[0] * axis_u[0] + local[1] * axis_v[0] + local[2] * axis_w[0],
        local[0] * axis_u[1] + local[1] * axis_v[1] + local[2] * axis_w[1],
        local[0] * axis_u[2] + local[1] * axis_v[2] + local[2] * axis_w[2],
    ]
}

pub(super) fn point_from_frame(
    center: &[f64; 3],
    axis_u: &[f64; 3],
    axis_v: &[f64; 3],
    axis_w: &[f64; 3],
    local: [f64; 3],
) -> [f64; 3] {
    let offset = combine_frame(axis_u, axis_v, axis_w, local);
    [
        center[0] + offset[0],
        center[1] + offset[1],
        center[2] + offset[2],
    ]
}

pub(super) fn frame_max_delta(
    u1: &[f64; 3],
    v1: &[f64; 3],
    w1: &[f64; 3],
    u2: &[f64; 3],
    v2: &[f64; 3],
    w2: &[f64; 3],
) -> f64 {
    axis_delta(u1, u2)
        .max(axis_delta(v1, v2))
        .max(axis_delta(w1, w2))
}

fn axis_delta(a: &[f64; 3], b: &[f64; 3]) -> f64 {
    norm([a[0] - b[0], a[1] - b[1], a[2] - b[2]])
}

pub(super) fn norm(v: [f64; 3]) -> f64 {
    (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt()
}

pub(super) fn cross(a: &[f64; 3], b: &[f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

pub(super) fn normalize(v: [f64; 3]) -> [f64; 3] {
    let len = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    if len < 1e-30 {
        return [0.0, 0.0, 0.0];
    }
    [v[0] / len, v[1] / len, v[2] / len]
}
