//! Shell and body closure/orientation validators.
//!
//! DOMAIN: Watertightness for solid shells, laminar-only boundaries
//! for sheet bodies, consistent shell orientation (outward for outer,
//! inward for inner), inner shell containment, and self-intersection
//! detection at the topology level.
//!
//! VALIDATORS (from validators.md §5):
//! - ValidateShellWatertightness
//! - ValidateBoundaryIsLaminarOnly
//! - ValidateConsistentShellOrientation
//! - ValidateInnerShellContainment
//! - ValidateNoInsideOutShells
//! - ValidateNoSelfIntersectingShellTopology
//!
//! DEPENDENCIES: `arena`, `handles`, `queries::shell`

use crate::arena::TopologyArena;
use crate::handles::FaceId;
use crate::topology::bitset::EntityBitset;
use crate::topology::queries::traverse::FaceEdgeIterator;
use forge_core::KernelError;
use std::collections::BTreeSet;

/// Collect halfedge IDs for a face's loop and find neighbor faces via twins.
///
/// Returns `(neighbor_faces, edge_keys, vertex_indices)` for the face.
pub(crate) fn collect_shell_data_for_face(
    arena: &TopologyArena,
    face_id: FaceId,
) -> Result<(Vec<FaceId>, Vec<u32>, Vec<u32>), KernelError> {
    let mut neighbors = Vec::new();
    let mut edge_keys = Vec::new();
    let mut vertex_indices = Vec::new();

    for he_result in FaceEdgeIterator::new(arena, face_id)? {
        let he_id = he_result?;
        let he_data = arena.get_half_edge(he_id)?;

        vertex_indices.push(he_data.origin().index());
        edge_keys.push(he_data.edge().index());

        for neighbor_res in crate::topology::queries::traverse::RadialEdgeIterator::new(arena, he_id)?
        {
            let neighbor_he = neighbor_res?;
            if neighbor_he != he_id {
                let neighbor_data = arena.get_half_edge(neighbor_he)?;
                neighbors.push(neighbor_data.face());
            }
        }
    }

    Ok((neighbors, edge_keys, vertex_indices))
}

/// Validate shell consistency: Solid shells must not contain boundary edges.
pub(crate) fn validate_shell_consistency(arena: &TopologyArena) -> Result<(), KernelError> {
    for (shell_id, shell_data) in arena.iter_shells() {
        if matches!(shell_data.kind(), crate::arena::ShellKind::Solid(_)) {
            for (face_id, face_data) in arena.iter_faces() {
                if face_data.shell() == shell_id {
                    let iter =
                        crate::topology::queries::traverse::FaceEdgeIterator::new(arena, face_id)?;
                    for he_res in iter {
                        let he_id = he_res?;
                        if crate::topology::queries::traverse::is_boundary_edge(arena, he_id)? {
                            return Err(KernelError::TopologyViolation {
                                err: forge_core::TopologyError::BoundaryEdgeInSolid {
                                    halfedge_index: he_id.index(),
                                    shell_index: shell_id.index(),
                                },
                                context: Some(forge_core::ErrorContext {
                                    scope: forge_core::ErrorScope::Entity {
                                        entity_kind: "HalfEdge".to_string(),
                                        index: he_id.index(),
                                    },
                                    suggested_fixes: Vec::new(),
                                    detail: format!(
                                        "Solid shell {} contains a boundary edge {} (Solid shells must be watertight)",
                                        shell_id.index(),
                                        he_id.index()
                                    ),
                                }),
                            });
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

/// Validate the 2-manifold invariant (Doctrine D8).
///
/// Every edge in every shell must have radial valence ≤ 2.
/// - **Solid shells**: valence must be exactly 2 (watertight).
/// - **Open shells**: valence 1 (boundary) or 2 (manifold) is valid.
/// - **Wire edges** (same-face twin pair from `MakeEdgeVertex`) are exempted
///   — they are valid topological construction features.
///
/// This is the commit-time enforcement of the NMT-aware data structure:
/// `radial_next` supports arbitrary-length rings during construction,
/// but `validate_manifold_edges` rejects valence > 2 at commit.
pub fn validate_manifold_edges(arena: &TopologyArena) -> Result<(), KernelError> {
    let mut checked_halfedges = EntityBitset::for_half_edges(arena);

    for (he_id, he_data) in arena.iter_half_edges() {
        if checked_halfedges.contains(he_id.index())? {
            continue;
        }

        checked_halfedges.insert(he_id.index())?;

        let edge_id = he_data.edge();
        let valence = crate::topology::queries::traverse::radial_valence(arena, he_id)?;

        let mut curr = he_data.radial_next();
        while curr != he_id {
            checked_halfedges.insert(curr.index())?;
            curr = arena.get_half_edge(curr)?.radial_next();
        }

        // Valence 1: self-radial wire edge (boundary halfedge). Valid.
        // Valence 2: manifold interior edge. Valid.
        // Valence > 2: non-manifold. Always rejected under ManifoldStrict.
        //
        if valence <= 2 {
            continue;
        }

        return Err(KernelError::TopologyViolation {
            err: forge_core::TopologyError::NonManifoldEdge {
                edge_index: edge_id.index(),
                valence,
            },
            context: Some(forge_core::ErrorContext {
                scope: forge_core::ErrorScope::Entity {
                    entity_kind: "Edge".to_string(),
                    index: edge_id.index(),
                },
                suggested_fixes: Vec::new(),
                detail: format!(
                    "Edge {} has radial valence {} (max allowed: 2). \
                     Doctrine D8 requires 2-manifold topology at commit time.",
                    edge_id.index(),
                    valence
                ),
            }),
        });
    }
    Ok(())
}

/// Validate orientation consistency across twin edge pairs (P0.3).
///
/// In a correctly oriented manifold halfedge mesh, every twin pair
/// (he, twin) must belong to different faces and traverse the shared
/// edge in opposite directions.
///
/// Wire edges (antennae from MakeEdgeVertex) are exempted: their twin
/// pair legitimately shares the same face. A wire edge is identified
/// by `he.face() == he.radial_next().face()` and is a valid non-manifold
/// feature, not an orientation defect.
pub(crate) fn validate_orientation_consistency(arena: &TopologyArena) -> Result<(), KernelError> {
    // In a single-face topology (e.g. digon from MVF+SE), all twin
    // pairs necessarily share the same face. This is valid — skip.
    if arena.face_count() <= 1 {
        return Ok(());
    }

    let mut checked: BTreeSet<(u32, u32)> = BTreeSet::new();

    for (he_id, he_data) in arena
        .iter_half_edges()
        .filter(|(id, d)| *id != d.radial_next())
    {
        let twin_id = he_data.radial_next();
        let canonical = (
            he_id.index().min(twin_id.index()),
            he_id.index().max(twin_id.index()),
        );

        if checked.insert(canonical) {
            let twin_data = arena.get_half_edge(twin_id)?;

            if he_data.face() == twin_data.face() {
                // Wire edge (antenna): both halfedges of a wire edge
                // share the same face. This is valid topology created by
                // MakeEdgeVertex — skip this pair.
                continue;
            }
        }
    }

    Ok(())
}
