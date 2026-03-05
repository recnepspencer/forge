//! Shell orientation consistency validation — adjacent face normals must agree.
//!
//! DOMAIN: Validates that across every shared (non-boundary) edge in a shell,
//! the two half-edges run in opposite directions. This is the topological
//! signature of compatible face orientations in an orientable 2-manifold.
//!
//! ALGORITHM: For each non-self-radial half-edge pair `(he, radial_next(he))`,
//! the half-edges must have opposite vertex endpoints. If `he` goes A→B,
//! its radial neighbor must go B→A. If both go A→B, the faces have
//! incompatible normals (one is flipped relative to the other).
//!
//! This is a pure combinatorial check augmented with position data only to
//! resolve vertex identity. It catches the class of bugs where MFKRH or
//! boolean operators produce faces with inconsistent winding.
//!
//! DEPENDENCIES: forge-topo (arena, handles, traversal), forge-core (KernelError).

use forge_core::KernelError;
use forge_topo::b_rep::TopologyArena;
use forge_topo::handles::VertexId;
use forge_topo::traverse::edge_endpoint_ids;

/// Validate that all adjacent faces across shared edges have compatible orientations.
///
/// For each non-boundary edge (radial_next ≠ self), checks that the two
/// half-edges sharing the geometric edge have opposite winding:
/// if `he_a` goes vertex A→B, then `he_b` must go B→A.
///
/// Same-direction half-edges indicate an orientation flip between adjacent faces.
pub fn validate_shell_orientation(
    arena: &TopologyArena,
    position_fn: &dyn Fn(VertexId) -> Option<[f64; 3]>,
) -> Result<(), KernelError> {
    // Iterate all half-edges. For manifold edges, check each pair once
    // (only check when he_id < radial_next).
    for (he_id, he_data) in arena.iter_half_edges() {
        let radial = he_data.radial_next();

        // Skip self-radial (boundary) edges — no neighbor to compare.
        if radial == he_id {
            continue;
        }

        // Only check each edge pair once (avoid double-checking).
        if he_id.index() >= radial.index() {
            continue;
        }

        // Get endpoints for both half-edges.
        let (origin_a, dest_a) = edge_endpoint_ids(arena, he_id)?;
        let (origin_b, dest_b) = edge_endpoint_ids(arena, radial)?;

        // For compatible orientation, the half-edges must run in opposite directions:
        // he_a: A→B, he_b: B→A (the standard orientable manifold relationship).
        //
        // We compare vertex positions rather than IDs because non-manifold
        // vertices could share a geometric position with different IDs.
        let pos_origin_a = position_fn(origin_a);
        let pos_dest_a = position_fn(dest_a);
        let pos_origin_b = position_fn(origin_b);
        let pos_dest_b = position_fn(dest_b);

        // If any position is missing, skip this edge (handled by completeness validator).
        let (pa_o, pa_d, pb_o, pb_d) = match (pos_origin_a, pos_dest_a, pos_origin_b, pos_dest_b) {
            (Some(a), Some(b), Some(c), Some(d)) => (a, b, c, d),
            _ => continue,
        };

        // Check if both half-edges run in the SAME direction (both A→B).
        // This indicates incompatible face orientations.
        let same_direction = positions_match(&pa_o, &pb_o) && positions_match(&pa_d, &pb_d);

        if same_direction {
            return Err(KernelError::TopologyViolation {
                err: forge_core::TopologyError::ValidatorFailure {
                    validator: "ShellOrientationConsistency".to_string(),
                    detail: format!(
                        "Half-edges {} and {} share a geometric edge but run in the same direction \
                         (both origin at vertex {}, dest at vertex {}). \
                         Adjacent faces {} and {} have incompatible normals.",
                        he_id.index(), radial.index(),
                        origin_a.index(), dest_a.index(),
                        he_data.face().index(),
                        arena.get_half_edge(radial).map(|r| r.face().index()).unwrap_or(u32::MAX),
                    ),
                },
                context: Some(forge_core::ErrorContext {
                    scope: forge_core::ErrorScope::Entity {
                        entity_kind: "Edge".to_string(),
                        index: he_data.edge().index(),
                    },
                    suggested_fixes: Vec::new(),
                    detail: format!(
                        "Reverse the winding of face {} or {} to restore orientation consistency",
                        he_data.face().index(),
                        arena.get_half_edge(radial).map(|r| r.face().index()).unwrap_or(u32::MAX),
                    ),
                }),
            });
        }
    }
    Ok(())
}

/// Check if two positions are the same point (within floating-point tolerance).
fn positions_match(a: &[f64; 3], b: &[f64; 3]) -> bool {
    let dx = a[0] - b[0];
    let dy = a[1] - b[1];
    let dz = a[2] - b[2];
    dx * dx + dy * dy + dz * dz < 1e-20
}
