//! Face classification for Boolean operations.
//!
//! Classifies each face of a solid relative to the other solid by
//! sampling a point on the face interior and testing point-in-solid.

use forge_core::KernelError;
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
pub fn classify_faces(
    source_arena: &TopologyArena,
    source_geometry: &GeometryStore,
    other_arena: &TopologyArena,
    other_geometry: &GeometryStore,
    _origin: FaceOrigin,
    config: &ToleranceConfig,
) -> Result<Vec<ClassifiedFace>, KernelError> {
    let mut classified = Vec::new();

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
        )?;

        let face_class = match classification {
            forge_topo::classify::PointClassification::Inside => FaceClassification::Inside,
            forge_topo::classify::PointClassification::Outside => FaceClassification::Outside,
            forge_topo::classify::PointClassification::OnBoundary(_) => FaceClassification::OnBoundary,
        };

        classified.push(ClassifiedFace::new(face_id, face_class));
    }

    Ok(classified)
}

