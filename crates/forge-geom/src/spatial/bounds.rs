pub fn compute_characteristic_scale(positions: impl Iterator<Item = [f64; 3]>) -> f64 {
    let mut min_pos = [f64::INFINITY; 3];
    let mut max_pos = [f64::NEG_INFINITY; 3];

    for pos in positions {
        min_pos = forge_math::linalg::component_min(min_pos, pos);
        max_pos = forge_math::linalg::component_max(max_pos, pos);
    }

    let diagonal = forge_math::linalg::norm(forge_math::linalg::sub(max_pos, min_pos));

    diagonal.max(1e-15)
}
