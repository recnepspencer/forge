//! Equivalence testing for solid bodies.
//!
//! DOMAIN: Determine spatial equivalence or inclusion without slicing geometry.
//! Used by boolean fast-paths to detect `A - A = Ø` or touching contact.

use forge_core::{KernelError, ToleranceProvider};
use forge_topo::b_rep::TopologyArena;
use forge_topo::handles::VertexId;

use crate::geometry_state::GeometryState;
use crate::spatial::{classify_point_in_solid, PointClassification};

/// Check whether two solids are coincident (all test faces lie on the target boundary).
///
/// This is used to detect identical solids (for A−A=∅) or full containment.
/// It checks if every face centroid from `test_arena` classifies as `OnBoundary`
/// against `target_arena`.
pub fn are_solids_coincident(
    target_arena: &TopologyArena,
    target_geom: &GeometryState,
    test_arena: &TopologyArena,
    test_geom: &GeometryState,
) -> Result<bool, KernelError> {
    if target_arena.face_count() != test_arena.face_count() {
        return Ok(false);
    }

    for (face_id, _) in test_arena.iter_faces() {
        let centroid = crate::shared_ops::vertex::centroid::compute_face_centroid(
            test_arena, test_geom, face_id,
        )
        .ok_or_else(|| KernelError::InvalidInput {
            message: format!("Face {:?} has degenerate geometry", face_id),
            context: None,
        })?;

        let class = classify_point_in_solid(
            target_arena,
            &|index| lookup_vertex(target_arena, target_geom, index),
            None,
            &centroid,
            target_geom as &dyn ToleranceProvider,
        )?;

        if !is_on_boundary(&class) {
            return Ok(false);
        }
    }

    Ok(true)
}

/// Check if any face centroid from the test solid lies on the target's boundary.
///
/// Used to quickly check for "Touching" contact between disjoint shells.
pub fn has_boundary_centroid(
    target_arena: &TopologyArena,
    target_geom: &GeometryState,
    test_arena: &TopologyArena,
    test_geom: &GeometryState,
) -> Result<bool, KernelError> {
    for (face_id, _) in test_arena.iter_faces() {
        let centroid = crate::shared_ops::vertex::centroid::compute_face_centroid(
            test_arena, test_geom, face_id,
        )
        .ok_or_else(|| KernelError::InvalidInput {
            message: format!("Face {:?} has degenerate geometry", face_id),
            context: None,
        })?;

        let class = classify_point_in_solid(
            target_arena,
            &|index| lookup_vertex(target_arena, target_geom, index),
            None,
            &centroid,
            target_geom as &dyn ToleranceProvider,
        )?;

        if is_on_boundary(&class) {
            return Ok(true);
        }
    }
    Ok(false)
}

/// Helper to wrap vertex lookups for point classification.
fn lookup_vertex(
    arena: &TopologyArena,
    geom: &GeometryState,
    index: u32,
) -> Result<[f64; 3], KernelError> {
    let gen = arena.vertex_generation(index as usize).ok_or_else(|| {
        KernelError::InvalidInput {
            message: format!("No active vertex at slot index {}", index),
            context: None,
        }
    })?;
    let vid = VertexId::new(index, gen);
    geom.get_vertex_position(vid).copied().ok_or_else(|| {
        KernelError::InvalidInput {
            message: format!("No position for vertex {}", index),
            context: None,
        }
    })
}

fn is_on_boundary(cls: &PointClassification) -> bool {
    matches!(cls, PointClassification::OnBoundary(_))
}
