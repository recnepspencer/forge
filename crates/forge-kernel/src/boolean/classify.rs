//! Face classification for Boolean operations.
//!
//! Classifies each face of a solid relative to the other solid by
//! sampling a point on the face interior and testing point-in-solid.

use forge_core::KernelError;
use forge_topo::arena::TopologyArena;
use forge_topo::handles::FaceId;
use forge_topo::classify::classify_point_in_solid;

use crate::geometry_store::GeometryStore;
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
    origin: FaceOrigin,
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

        let ray_extent = 1e6;
        let classification = classify_point_in_solid(
            other_arena,
            &vertex_lookup,
            &sample,
            ray_extent,
        )?;

        let face_class = match classification {
            forge_topo::classify::PointClassification::Inside => FaceClassification::Inside,
            forge_topo::classify::PointClassification::Outside => FaceClassification::Outside,
            forge_topo::classify::PointClassification::OnBoundary(_) => FaceClassification::OnBoundary,
        };

        classified.push(ClassifiedFace::new(face_id, origin, face_class));
    }

    Ok(classified)
}

/// Compute the centroid of a face by averaging its vertex positions.
///
/// Walks the face's outer loop and averages all vertex positions to
/// produce a representative interior point for classification.
fn compute_face_centroid(
    arena: &TopologyArena,
    geometry: &GeometryStore,
    face: FaceId,
) -> Result<[f64; 3], KernelError> {
    let face_data = arena.get_face(face)?;
    let loop_data = arena.get_loop(face_data.outer_loop)?;
    let start_he = loop_data.half_edge;

    let mut sum = [0.0_f64; 3];
    let mut count = 0u32;
    let mut current = start_he;
    let max_iterations: usize = 1000;

    for _ in 0..max_iterations {
        let he_data = arena.get_half_edge(current)?;
        let vertex = he_data.origin;
        let pos = geometry.get_vertex_position(vertex).ok_or_else(|| {
            KernelError::InvalidInput {
                message: format!("No position for vertex {} during centroid computation", vertex),
                context: None,
            }
        })?;

        sum[0] += pos[0];
        sum[1] += pos[1];
        sum[2] += pos[2];
        count += 1;

        current = he_data.next;
        if current == start_he {
            let inv = 1.0 / f64::from(count);
            return Ok([sum[0] * inv, sum[1] * inv, sum[2] * inv]);
        }
    }

    Err(KernelError::InternalError {
        message: "Loop limit exceeded in compute_face_centroid".to_string(),
        context: None,
    })
}
