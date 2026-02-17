//! Face classification for Boolean operations.
//!
//! Classifies each face of a solid relative to the other solid by
//! sampling a point on the face interior and testing point-in-solid.
//! When a face is coplanar with the other solid's boundary, we check
//! normal alignment to distinguish same-direction overlap (OnBoundary)
//! from opposite-direction overlap (OppositeBoundary).

use forge_core::KernelError;
use forge_core::result::{DecisionLog, TracedDecision, DecisionId, DecisionKind, DecisionContext, EntityRef};
use forge_topo::arena::TopologyArena;
use forge_topo::classify::classify_point_in_solid;

use crate::core::ToleranceConfig;
use crate::geometry_store::GeometryStore;
use super::eval::compute_face_centroid;
use super::schema::{FaceClassification, ClassifiedFace, FaceOrigin};

/// Classify all faces of one solid relative to the other solid.
///
/// For each face, samples a point on the face interior and classifies
/// it against the other solid using ray-casting point-in-solid.
///
/// When the point lands OnBoundary, we compare the source face normal
/// with the boundary face normal to determine whether they're aligned
/// (same-direction coplanar overlap) or opposed (opposite normals).
pub fn classify_faces(
    source_arena: &TopologyArena,
    source_geometry: &GeometryStore,
    other_arena: &TopologyArena,
    other_geometry: &GeometryStore,
    origin: FaceOrigin,
    config: &ToleranceConfig,
) -> Result<(Vec<ClassifiedFace>, DecisionLog), KernelError> {
    let mut classified = Vec::new();
    let mut log = DecisionLog::new();
    let origin_label = match origin {
        FaceOrigin::Target => "Target",
        FaceOrigin::Tool => "Tool",
    };

    for (face_id, _face_data) in source_arena.iter_faces() {
        let sample = compute_face_centroid(source_arena, source_geometry, face_id)?;

        let vertex_lookup = |index: u32| -> Result<[f64; 3], KernelError> {
            let gen = other_arena.vertex_generation(index as usize).ok_or_else(|| {
                KernelError::InvalidInput {
                    message: format!("No active vertex at slot index {}", index),
                    context: None,
                }
            })?;
            let vid = forge_topo::handles::VertexId::new(index, gen);
            other_geometry.get_vertex_position(vid).copied().ok_or_else(|| {
                KernelError::InvalidInput {
                    message: format!("No position for vertex {}", index),
                    context: None,
                }
            })
        };

        let classification = classify_point_in_solid(
            other_arena,
            &vertex_lookup,
            &sample,
            config.get_ray_extent(),
            config.get_edge_split_degeneracy(),
        )?;

        let (face_class, class_label) = match classification {
            forge_topo::classify::PointClassification::Inside => (FaceClassification::Inside, "Inside"),
            forge_topo::classify::PointClassification::Outside => (FaceClassification::Outside, "Outside"),
            forge_topo::classify::PointClassification::OnBoundary(boundary_face_id) => {
                // Coplanar resolution: check normal alignment between
                // the source face and the boundary face it landed on.
                let normals_align = check_normal_alignment(
                    source_geometry, face_id,
                    other_geometry, boundary_face_id,
                );
                if normals_align {
                    (FaceClassification::OnBoundary, "OnBoundary(aligned)")
                } else {
                    (FaceClassification::OppositeBoundary, "OppositeBoundary(opposed)")
                }
            }
        };

        let mut decision = TracedDecision::new(
            DecisionId(face_id.index() as u64),
            DecisionKind::Exact,
            1.0,
            DecisionContext::Classification {
                point: sample,
                result: format!("{}:Face#{} → {}", origin_label, face_id.index(), class_label),
            },
        );
        decision.set_entity_scope(EntityRef::new("Face", face_id.index()));
        log.record(decision);

        classified.push(ClassifiedFace::new(face_id, face_class));
    }

    Ok((classified, log))
}

/// Check whether two faces have aligned normals (same direction).
///
/// Compares the face plane normals via dot product:
/// - Positive dot product → same direction (aligned)
/// - Negative dot product → opposite direction (opposed)
/// - Zero → perpendicular (shouldn't happen for coplanar faces; default to aligned)
fn check_normal_alignment(
    source_geom: &GeometryStore,
    source_face: forge_topo::handles::FaceId,
    other_geom: &GeometryStore,
    other_face: forge_topo::handles::FaceId,
) -> bool {
    let source_plane = source_geom.get_face_plane(source_face);
    let other_plane = other_geom.get_face_plane(other_face);

    match (source_plane, other_plane) {
        (Some(sp), Some(op)) => {
            let sn = sp.raw_normal();
            let on = op.raw_normal();
            let dot = sn[0] * on[0] + sn[1] * on[1] + sn[2] * on[2];
            dot > 0.0
        }
        _ => true, // Default to aligned if planes unavailable
    }
}
