//! Face normal orientation classification.
//!
//! DOMAIN: Classifies whether a face normal points outward by probing
//! both sides of the face with `classify_point_in_solid`.
//!
//! ALGORITHM:
//!   1. Compute face normal `n` via Newell's method (continuity module)
//!   2. Compute face interior point `p` via polygon centroid (forge-geom)
//!   3. Classify `p + ε·n` and `p - ε·n` via point-in-solid
//!   4. Outward iff (p + εn) is Outside and (p - εn) is Inside
//!
//! ASSUMPTION(convex_faces): The face interior point is the vertex-average
//! centroid, which is only guaranteed inside the polygon for convex faces
//! (all BSP primitives). For concave faces, use a proper polygon interior point.
//!
//! EPSILON CONSTRAINT: `ε` must satisfy:
//!   `boundary_tolerance < ε < min_feature_size / 2`
//!
//! DEPENDENCIES: forge-topo (arena, handles), forge-geom (polygon centroid),
//!               forge-core (ToleranceProvider).

use forge_core::{KernelError, ToleranceProvider};
use forge_geom::facade::compute_polygon_centroid;
use forge_topo::b_rep::TopologyArena;
use forge_topo::handles::{FaceId, VertexId};

use super::point_in_solid::classify_point_in_solid;
use super::schema::{NormalClassification, PointClassification};
use crate::operations::continuity::face_normal_from_outer_loop;

/// Classify whether a face normal points outward by probing both sides.
///
/// `position_fn`: maps raw vertex slot index → 3D position (same signature
///                as `classify_point_in_solid`)
/// `face_position_fn`: maps `VertexId` → position (for Newell normal computation,
///                     which uses typed handles)
/// `face_vertices`: ordered vertex positions for the face (for centroid)
/// `epsilon`: probe offset distance. See EPSILON CONSTRAINT in module docs.
/// `tolerance`: geometry-derived tolerance for point classification.
pub fn classify_face_normal_orientation(
    arena: &TopologyArena,
    position_fn: &dyn Fn(u32) -> Result<[f64; 3], KernelError>,
    face_position_fn: &dyn Fn(VertexId) -> Option<[f64; 3]>,
    face_vertices: &[[f64; 3]],
    face_id: FaceId,
    epsilon: f64,
    tolerance: &dyn ToleranceProvider,
) -> Result<NormalClassification, KernelError> {
    // Step 1: Compute face normal via Newell's method
    let normal = match face_normal_from_outer_loop(arena, face_position_fn, face_id)? {
        Some(n) => n,
        None => return Ok(NormalClassification::Degenerate {
            reason: "face normal undefined (collinear/degenerate vertices)",
        }),
    };

    // Step 2: Compute face interior point via polygon centroid (forge-geom)
    let face_center = match compute_polygon_centroid(face_vertices) {
        Some(c) => c,
        None => return Ok(NormalClassification::Degenerate {
            reason: "face has no vertices, cannot compute interior point",
        }),
    };

    // Step 3: Construct probe points p ± ε·n
    let p_plus = [
        face_center[0] + epsilon * normal[0],
        face_center[1] + epsilon * normal[1],
        face_center[2] + epsilon * normal[2],
    ];
    let p_minus = [
        face_center[0] - epsilon * normal[0],
        face_center[1] - epsilon * normal[1],
        face_center[2] - epsilon * normal[2],
    ];

    // Step 4: Classify both probe points
    let class_plus = classify_point_in_solid(
        arena, position_fn, None, &p_plus, tolerance,
    )?;
    let class_minus = classify_point_in_solid(
        arena, position_fn, None, &p_minus, tolerance,
    )?;

    match (&class_plus, &class_minus) {
        (PointClassification::Outside { .. }, PointClassification::Inside { .. }) => {
            Ok(NormalClassification::OutwardConfirmed)
        }
        (PointClassification::Inside { .. }, PointClassification::Outside { .. }) => {
            Ok(NormalClassification::InwardDetected)
        }
        _ => Ok(NormalClassification::Degenerate {
            reason: "probe classification ambiguous (boundary or same-side)",
        }),
    }
}
