//! Zero-area face detection.
//!
//! DOMAIN: Validate that no planar face has area below its per-vertex tolerance.

use forge_core::{KernelError, ToleranceProvider};
use worth_geom::compute_polygon_area;
use forge_topo::b_rep::TopologyArena;
use forge_topo::handles::{FaceId, VertexId};
use forge_topo::traverse::FaceEdgeIterator;

use super::utils;

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

        let positions = utils::collect_face_positions(arena, face_id, position_fn)?;
        if positions.len() < 3 {
            continue;
        }

        let area_threshold = FaceEdgeIterator::new(arena, face_id)?
            .filter_map(|r| r.ok())
            .filter_map(|he_id| arena.get_half_edge(he_id).ok())
            .map(|he| {
                tolerance_provider.vertex_tolerance(he.origin().index(), he.origin().generation())
            })
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
                        face_id.index(),
                        area,
                        area_threshold
                    ),
                }),
            });
        }
    }
    Ok(())
}
