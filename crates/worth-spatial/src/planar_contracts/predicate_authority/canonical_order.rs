pub(crate) fn canonical_cyclic_orient2d_points(points: [[f64; 2]; 3]) -> [[f64; 2]; 3] {
    let rotations = [
        [points[0], points[1], points[2]],
        [points[1], points[2], points[0]],
        [points[2], points[0], points[1]],
    ];
    rotations
        .into_iter()
        .min_by(compare_point_triples)
        .expect("fixed non-empty cyclic rotation set")
}

pub(crate) fn canonical_planar_coordinate_bits(value: f64) -> u64 {
    if value == 0.0 {
        0.0f64.to_bits()
    } else {
        value.to_bits()
    }
}

fn compare_point_triples(left: &[[f64; 2]; 3], right: &[[f64; 2]; 3]) -> std::cmp::Ordering {
    left.iter()
        .flat_map(|point| point.iter())
        .map(|value| canonical_planar_coordinate_bits(*value))
        .cmp(
            right
                .iter()
                .flat_map(|point| point.iter())
                .map(|value| canonical_planar_coordinate_bits(*value)),
        )
}
