//! Shell, region, lump, and solid distance and scale queries.
//!
//! DOMAIN: Compute center points and overall scale characteristic dimensions
//!         for entire solids or sub-components.

use forge_core::KernelError;
use forge_topo::arena::TopologyArena;
use forge_topo::handles::VertexId;

/// Compute the centroid of a solid by averaging all of its vertex positions.
pub fn compute_solid_centroid(
    arena: &TopologyArena,
    position_fn: &dyn Fn(VertexId) -> Option<[f64; 3]>,
) -> Result<[f64; 3], KernelError> {
    let vertices: Vec<[f64; 3]> = arena
        .iter_vertices()
        .filter_map(|(vid, _)| position_fn(vid))
        .collect();

    if vertices.is_empty() {
        return Err(KernelError::InvalidInput {
            message: "Cannot compute solid centroid: no valid vertex positions found".to_string(),
            context: None,
        });
    }

    let coords =
        forge_geom::primitives::polygon::compute_polygon_centroid(&vertices).unwrap_or(vertices[0]);

    Ok(coords)
}

/// Compute the characteristic scale (max bounding box diagonal) across one or two arenas.
///
/// Useful for determining spatial hashing tolerances or disjoint placement logic.
pub fn combined_solid_scale(
    primary_arena: &TopologyArena,
    primary_pos: &dyn Fn(VertexId) -> Option<[f64; 3]>,
    secondary: Option<(&TopologyArena, &dyn Fn(VertexId) -> Option<[f64; 3]>)>,
) -> f64 {
    let mut min_pos = [f64::INFINITY; 3];
    let mut max_pos = [f64::NEG_INFINITY; 3];

    for (vid, _) in primary_arena.iter_vertices() {
        if let Some(pos) = primary_pos(vid) {
            min_pos = forge_math::linalg::component_min(min_pos, pos);
            max_pos = forge_math::linalg::component_max(max_pos, pos);
        }
    }

    if let Some((sec_arena, sec_pos)) = secondary {
        for (vid, _) in sec_arena.iter_vertices() {
            if let Some(pos) = sec_pos(vid) {
                min_pos = forge_math::linalg::component_min(min_pos, pos);
                max_pos = forge_math::linalg::component_max(max_pos, pos);
            }
        }
    }

    if min_pos[0] == f64::INFINITY {
        return 1e-15; // Fallback for completely empty arenas
    }

    forge_math::linalg::norm(forge_math::linalg::sub(max_pos, min_pos)).max(1e-15)
}

/// Compute a scale-aware ray extent for point-in-solid classification.
///
/// Walks all vertex positions in `arena` to compute the bounding box diagonal,
/// then multiplies by `scale_factor` (typically 10.0 for a safe ray overshoot).
/// The `scale_factor` is always supplied by the kernel via `ToleranceConfig` — no
/// magic numbers live below `forge-kernel`.
///
/// Returns `default_extent` when the arena is empty or unbounded.
pub fn compute_solid_ray_extent(
    arena: &forge_topo::arena::TopologyArena,
    position_fn: &dyn Fn(forge_topo::handles::VertexId) -> Option<[f64; 3]>,
    scale_factor: f64,
    default_extent: f64,
) -> f64 {
    let mut min_pos = [f64::INFINITY; 3];
    let mut max_pos = [f64::NEG_INFINITY; 3];

    for (vid, _) in arena.iter_vertices() {
        if let Some(pos) = position_fn(vid) {
            min_pos = forge_math::linalg::component_min(min_pos, pos);
            max_pos = forge_math::linalg::component_max(max_pos, pos);
        }
    }

    if min_pos[0] == f64::INFINITY {
        return default_extent;
    }

    let diagonal = forge_math::linalg::norm(forge_math::linalg::sub(max_pos, min_pos));
    (diagonal * scale_factor).max(default_extent)
}
