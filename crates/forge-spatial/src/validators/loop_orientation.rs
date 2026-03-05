//! Loop orientation validation — outer loops CCW, inner loops CW.
//!
//! DOMAIN: Validates that face loops have consistent winding relative to
//! the face normal. Outer loops must wind counter-clockwise (positive signed
//! area) and inner loops must wind clockwise (negative signed area) when
//! projected onto the Newell normal of the face.
//!
//! ALGORITHM: Newell method for face normal, then signed-area projection
//! of each loop onto that normal to determine winding sense.
//!
//! DEPENDENCIES: forge-topo (arena, handles, traversal), forge-core (KernelError).

use forge_core::KernelError;
use forge_topo::b_rep::TopologyArena;
use forge_topo::handles::{FaceId, VertexId};
use forge_topo::traverse::{FaceEdgeIterator, FaceLoopsIterator, LoopEdgeIterator};

/// Validate that all face loops have consistent orientation.
///
/// For each face with geometry (non-degenerate position data):
/// - The outer loop must wind CCW relative to the face Newell normal (positive projected area).
/// - Each inner loop must wind CW (negative projected area).
///
/// Faces with fewer than 3 vertices are skipped (degenerate).
pub fn validate_loop_orientation(
    arena: &TopologyArena,
    position_fn: &dyn Fn(VertexId) -> Option<[f64; 3]>,
    is_planar: &dyn Fn(FaceId) -> bool,
) -> Result<(), KernelError> {
    for (face_id, _face_data) in arena.iter_faces() {
        if !is_planar(face_id) {
            continue;
        }

        // Collect outer loop positions and compute Newell normal.
        let outer_positions = collect_loop_positions(arena, face_id, position_fn, true)?;
        if outer_positions.len() < 3 {
            continue;
        }

        let face_normal = newell_normal(&outer_positions);
        let normal_mag_sq = dot(&face_normal, &face_normal);
        if normal_mag_sq < 1e-30 {
            // Degenerate face (all vertices collinear) — skip.
            continue;
        }

        // Check outer loop: signed area projected onto face normal must be positive.
        let outer_signed_area = projected_signed_area(&outer_positions, &face_normal);
        if outer_signed_area < 0.0 {
            return Err(KernelError::TopologyViolation {
                err: forge_core::TopologyError::OrientationInconsistency {
                    face_index: face_id.index(),
                },
                context: Some(forge_core::ErrorContext {
                    scope: forge_core::ErrorScope::Entity {
                        entity_kind: "Face".to_string(),
                        index: face_id.index(),
                    },
                    suggested_fixes: Vec::new(),
                    detail: format!(
                        "Outer loop of face {} winds CW (projected signed area {:.6e}), expected CCW",
                        face_id.index(), outer_signed_area
                    ),
                }),
            });
        }

        // Check inner loops: each must have negative projected signed area (CW).
        let loops = FaceLoopsIterator::new(arena, face_id)?;
        let mut loop_index: u32 = 0;
        for loop_id in loops {
            if loop_index == 0 {
                // Skip outer loop (already checked).
                loop_index += 1;
                continue;
            }

            let inner_positions = collect_loop_positions_for_loop(arena, loop_id, position_fn)?;
            if inner_positions.len() < 3 {
                loop_index += 1;
                continue;
            }

            let inner_signed_area = projected_signed_area(&inner_positions, &face_normal);
            if inner_signed_area > 0.0 {
                return Err(KernelError::TopologyViolation {
                    err: forge_core::TopologyError::OrientationInconsistency {
                        face_index: face_id.index(),
                    },
                    context: Some(forge_core::ErrorContext {
                        scope: forge_core::ErrorScope::Entity {
                            entity_kind: "Face".to_string(),
                            index: face_id.index(),
                        },
                        suggested_fixes: Vec::new(),
                        detail: format!(
                            "Inner loop {} of face {} winds CCW (projected signed area {:.6e}), expected CW",
                            loop_index, face_id.index(), inner_signed_area
                        ),
                    }),
                });
            }

            loop_index += 1;
        }
    }
    Ok(())
}

// ── Geometry helpers ────────────────────────────────────────────────────

/// Compute the Newell normal for a polygon via summing cross products.
fn newell_normal(positions: &[[f64; 3]]) -> [f64; 3] {
    let n = positions.len();
    let mut normal = [0.0; 3];

    for i in 0..n {
        let curr = positions[i];
        let next = positions[(i + 1) % n];

        normal[0] += (curr[1] - next[1]) * (curr[2] + next[2]);
        normal[1] += (curr[2] - next[2]) * (curr[0] + next[0]);
        normal[2] += (curr[0] - next[0]) * (curr[1] + next[1]);
    }

    normal
}

/// Compute the signed area of a polygon projected onto a given normal.
///
/// Result > 0 means CCW winding relative to the normal.
/// Result < 0 means CW winding.
fn projected_signed_area(positions: &[[f64; 3]], normal: &[f64; 3]) -> f64 {
    let polygon_normal = newell_normal(positions);
    // The dot product of the polygon's Newell normal with the reference normal
    // gives 2× the signed area projected onto the reference normal plane.
    dot(&polygon_normal, normal)
}

fn dot(a: &[f64; 3], b: &[f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

/// Collect vertex positions for the outer loop of a face.
fn collect_loop_positions(
    arena: &TopologyArena,
    face_id: FaceId,
    position_fn: &dyn Fn(VertexId) -> Option<[f64; 3]>,
    _outer: bool,
) -> Result<Vec<[f64; 3]>, KernelError> {
    let mut positions = Vec::new();
    for he_res in FaceEdgeIterator::new(arena, face_id)? {
        let he_id = he_res?;
        let he = arena.get_half_edge(he_id)?;
        if let Some(pos) = position_fn(he.origin()) {
            positions.push(pos);
        }
    }
    Ok(positions)
}

/// Collect vertex positions for a specific loop.
fn collect_loop_positions_for_loop(
    arena: &TopologyArena,
    loop_id: forge_topo::handles::LoopId,
    position_fn: &dyn Fn(VertexId) -> Option<[f64; 3]>,
) -> Result<Vec<[f64; 3]>, KernelError> {
    let mut positions = Vec::new();
    for he_res in LoopEdgeIterator::new(arena, loop_id)? {
        let he_id = he_res?;
        let he = arena.get_half_edge(he_id)?;
        if let Some(pos) = position_fn(he.origin()) {
            positions.push(pos);
        }
    }
    Ok(positions)
}
