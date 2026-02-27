//! Shared utility for computing face centroids.

use forge_topo::arena::TopologyArena;
use forge_topo::handles::FaceId;
use crate::geometry_state::GeometryState;

/// Compute the centroid of a face using its vertices.
pub fn compute_face_centroid(
    arena: &TopologyArena,
    geom: &GeometryState,
    face: FaceId,
) -> Option<[f64; 3]> {
    let loops = forge_topo::polygon::face_loop_vertices(arena, face).ok()?;
    let vertices = loops.first()?;
    if vertices.is_empty() {
        return None;
    }

    let mut sum = [0.0, 0.0, 0.0];
    for &vid in vertices {
        let coords = geom.get_vertex_position(vid)?;
        sum[0] += coords[0];
        sum[1] += coords[1];
        sum[2] += coords[2];
    }

    let count = vertices.len() as f64;
    Some([sum[0] / count, sum[1] / count, sum[2] / count])
}
