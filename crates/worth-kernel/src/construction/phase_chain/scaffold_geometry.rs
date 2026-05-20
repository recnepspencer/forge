use worth_geom::facade::Plane;

pub fn simplex_vertices(scale: f64) -> Vec<[f64; 3]> {
    vec![
        [0.0, 0.0, scale],
        [0.0, scale, -scale],
        [-scale * 0.7071, -scale * 0.5, -scale],
        [scale * 0.7071, -scale * 0.5, -scale],
    ]
}

pub fn orthotope_vertices(half_extents: [f64; 3]) -> Vec<[f64; 3]> {
    let [hx, hy, hz] = half_extents;
    vec![
        [-hx, -hy, -hz],
        [-hx, -hy, hz],
        [-hx, hy, -hz],
        [-hx, hy, hz],
        [hx, -hy, -hz],
        [hx, -hy, hz],
        [hx, hy, -hz],
        [hx, hy, hz],
    ]
}

pub fn prism_vertices(sides: u32, radius: f64, height: f64) -> Vec<[f64; 3]> {
    let half_height = height / 2.0;
    (0..sides)
        .flat_map(|index| {
            let angle = std::f64::consts::TAU * index as f64 / sides as f64;
            let x = angle.cos() * radius;
            let y = angle.sin() * radius;
            [[x, y, -half_height], [x, y, half_height]]
        })
        .collect()
}

pub fn pyramid_vertices(sides: u32, radius: f64, height: f64) -> Vec<[f64; 3]> {
    let mut vertices = regular_polygon_vertices([0.0, 0.0, 0.0], sides, radius);
    vertices.push([0.0, 0.0, height]);
    vertices
}

pub fn wire_body_vertices(edge_count: u32, radius: f64) -> Vec<[f64; 3]> {
    regular_polygon_vertices([0.0, 0.0, 0.0], edge_count, radius)
}

pub fn shell_with_hole_vertices(
    outer_loop_edge_count: u32,
    hole_loop_edge_counts: &[u32],
) -> Vec<[f64; 3]> {
    let mut vertices = regular_polygon_vertices([0.0, 0.0, 0.0], outer_loop_edge_count, 3.0);
    let hole_centers = hole_loop_centers(hole_loop_edge_counts.len());
    for (index, edge_count) in hole_loop_edge_counts.iter().copied().enumerate() {
        vertices.extend(regular_polygon_vertices(
            [hole_centers[index][0], hole_centers[index][1], 0.0],
            edge_count,
            0.4,
        ));
    }
    vertices
}

pub fn planar_support_plane() -> Result<Plane, String> {
    Plane::from_point_normal([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]).map_err(|error| error.to_string())
}

fn regular_polygon_vertices(center: [f64; 3], sides: u32, radius: f64) -> Vec<[f64; 3]> {
    (0..sides)
        .map(|index| {
            let angle = std::f64::consts::TAU * index as f64 / sides as f64;
            [
                center[0] + angle.cos() * radius,
                center[1] + angle.sin() * radius,
                center[2],
            ]
        })
        .collect()
}

fn hole_loop_centers(count: usize) -> Vec<[f64; 2]> {
    if count == 1 {
        return vec![[0.0, 0.0]];
    }
    (0..count)
        .map(|index| {
            let angle = std::f64::consts::TAU * index as f64 / count as f64;
            [angle.cos() * 1.2, angle.sin() * 1.2]
        })
        .collect()
}
