//! Zero-area face detection.
//!
//! DOMAIN: Validate that no planar face has area below its per-vertex tolerance.

use forge_core::{KernelError, ToleranceProvider};
use forge_geom::primitives::polygon::compute_polygon_area;
use forge_topo::arena::TopologyArena;
use forge_topo::handles::{FaceId, VertexId};
use forge_topo::traverse::FaceEdgeIterator;

/// Validate that no planar face has area below its per-vertex tolerance squared.
pub fn validate_zero_area_faces(
    arena: &TopologyArena,
    position_fn: &dyn Fn(VertexId) -> Option<[f64; 3]>,
    is_planar: &dyn Fn(FaceId) -> bool,
    tolerance_provider: &dyn ToleranceProvider,
) -> Result<(), KernelError> {
    for (face_id, _) in arena.iter_faces() {
        if !is_planar(face_id) {
            continue;
        }

        let positions = collect_face_positions(arena, face_id, position_fn)?;
        if positions.len() < 3 {
            continue;
        }

        let area_threshold = FaceEdgeIterator::new(arena, face_id)?
            .filter_map(|r| r.ok())
            .filter_map(|he_id| arena.get_half_edge(he_id).ok())
            .map(|he| tolerance_provider.vertex_tolerance(he.origin().index(), he.origin().generation()))
            .fold(0.0_f64, f64::max)
            .powi(2);

        let area = compute_polygon_area(&positions);

        if area < area_threshold {
            return Err(KernelError::TopologyViolation {
                err: forge_core::TopologyError::ZeroAreaFace {
                    face_index: face_id.index(),
                    computed_area: area,
                    threshold: area_threshold,
                },
                context: Some(forge_core::ErrorContext {
                    scope: forge_core::ErrorScope::Entity {
                        entity_kind: "Face".to_string(),
                        index: face_id.index(),
                    },
                    suggested_fixes: Vec::new(),
                    detail: format!(
                        "Face {} area {:.2e} below threshold {:.2e}",
                        face_id.index(), area, area_threshold
                    ),
                }),
            });
        }
    }
    Ok(())
}

fn collect_face_positions(
    arena: &TopologyArena,
    face_id: FaceId,
    position_fn: &dyn Fn(VertexId) -> Option<[f64; 3]>,
) -> Result<Vec<[f64; 3]>, KernelError> {
    let mut positions = Vec::new();
    for he_res in FaceEdgeIterator::new(arena, face_id)? {
        let he_id = he_res?;
        let he = arena.get_half_edge(he_id)?;
        let v = he.origin();
        let pos = position_fn(v).ok_or_else(|| KernelError::TopologyViolation {
            err: forge_core::TopologyError::MissingVertexPosition {
                vertex_index: v.index(),
                face_index: face_id.index(),
            },
            context: None,
        })?;
        positions.push(pos);
    }
    Ok(positions)
}
