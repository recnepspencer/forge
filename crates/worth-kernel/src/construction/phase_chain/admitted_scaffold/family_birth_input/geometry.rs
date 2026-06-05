use worth_geom::facade::Plane;
use worth_primitives::{
    canonical_orthotope_vertices, canonical_prism_vertices, canonical_pyramid_vertices,
    canonical_simplex_vertices, canonical_wire_body_vertices, derive_shell_with_hole_layout,
    shell_with_hole_vertices_from_layout, ShellWithHoleWitnessLayoutError,
    ShellWithHoleWitnessLayoutPolicy,
};

pub fn simplex_vertices(scale: f64, auxiliary_altitude_component: f64) -> Vec<[f64; 3]> {
    canonical_simplex_vertices(scale, auxiliary_altitude_component)
        .local_vertices()
        .to_vec()
}

pub fn orthotope_vertices(half_extents: [f64; 3]) -> Vec<[f64; 3]> {
    canonical_orthotope_vertices(half_extents)
        .local_vertices()
        .to_vec()
}

pub fn prism_vertices(sides: u32, radius: f64, height: f64) -> Vec<[f64; 3]> {
    canonical_prism_vertices(sides, radius, height)
        .local_vertices()
        .to_vec()
}

pub fn pyramid_vertices(sides: u32, radius: f64, height: f64) -> Vec<[f64; 3]> {
    canonical_pyramid_vertices(sides, radius, height)
        .local_vertices()
        .to_vec()
}

pub fn wire_body_vertices(edge_count: u32, radius: f64) -> Vec<[f64; 3]> {
    if (radius - 1.5).abs() <= f64::EPSILON {
        canonical_wire_body_vertices(edge_count)
            .local_vertices()
            .to_vec()
    } else {
        regular_polygon_vertices([0.0, 0.0, 0.0], edge_count, radius)
    }
}

pub fn shell_with_hole_vertices(
    outer_loop_edge_count: u32,
    hole_loop_edge_counts: &[u32],
) -> Result<Vec<[f64; 3]>, ShellWithHoleWitnessLayoutError> {
    let (layout, _) = derive_shell_with_hole_layout(
        outer_loop_edge_count,
        hole_loop_edge_counts,
        ShellWithHoleWitnessLayoutPolicy::default(),
    )?;
    Ok(
        shell_with_hole_vertices_from_layout(outer_loop_edge_count, hole_loop_edge_counts, &layout)
            .local_vertices()
            .to_vec(),
    )
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
