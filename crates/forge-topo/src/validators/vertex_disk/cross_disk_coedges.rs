//! Validate No Cross-Disk Co-Edges
//!
//! INVARIANT: At a non-manifold vertex with multiple disks, a single face
//! must not have co-edges that span across different disks. If a face visits
//! a vertex multiple times, each visit must be fully contained within one disk
//! or belong to entirely separate topological structures, but a twin pair
//! must not straddle a pinch point.

use crate::b_rep::TopologyArena;
use crate::handles::HalfEdgeId;
use forge_core::KernelError;
use std::collections::BTreeSet;

pub(crate) fn validate_no_cross_disk_coedges(arena: &TopologyArena) -> Result<(), KernelError> {
    let mut visited: BTreeSet<HalfEdgeId> = BTreeSet::new();

    for (vid, _) in arena.iter_vertices() {
        // Find all disks at this vertex
        let mut disks: Vec<BTreeSet<HalfEdgeId>> = Vec::new();

        for (he_id, he_data) in arena.iter_half_edges() {
            if he_data.origin() == vid && !visited.contains(&he_id) {
                let (disk, _) = super::disk_walker::collect_disk(arena, he_id)?;
                visited.extend(disk.iter().copied());
                disks.push(disk);
            }
        }

        // If a vertex has only one disk, there can be no cross-disk co-edges.
        if disks.len() <= 1 {
            continue;
        }

        // For each disk, gather the pairs of consecutive half-edges (he, next(he))
        // that belong to the same face. Wait, a co-edge is (he, next) on the face loop.
        // If a face passes through a vertex, it uses two half-edges: an incoming
        // and an outgoing. The incoming half-edge's NEXT is the outgoing half-edge.
        // We want to ensure that incoming and outgoing are in the SAME disk!

        // Let's check every halfedge in the arena whose origin is `vid`.
        // This is the "outgoing" halfedge. Its incoming is `prev()`.
        for (he_id, he_data) in arena.iter_half_edges() {
            if he_data.origin() == vid {
                let prev_id = he_data.prev();
                let incoming = arena.get_half_edge(prev_id)?;

                // Which disk is `he_id` (outgoing) in?
                let out_idx = disks
                    .iter()
                    .position(|d| d.contains(&he_id))
                    .unwrap_or(usize::MAX);

                // Which disk is `twin(incoming)` in?
                // The incoming halfedge has origin at some other vertex, but its twin
                // (radial_next of incoming... wait, no. We just check if they are in the
                // same disk logic).
                // Actually, the disk around a vertex is formed by all OUTGOING halfedges.
                // The cross-disk co-edge check asserts that `he` and `twin(he.prev())`
                // must be in the SAME vertex disk.
                // Because `twin(he.prev())` is an outgoing halfedge at the SAME vertex!
                let incoming_twin_candidate = incoming.radial_next();
                let incoming_twin_data = arena.get_half_edge(incoming_twin_candidate)?;

                if incoming_twin_data.origin() != vid {
                    // It might not be the exact origin if the face is corrupted, but
                    // other validators catch that. If it matches, check disks.
                    continue;
                }

                let in_idx = disks
                    .iter()
                    .position(|d| d.contains(&incoming_twin_candidate))
                    .unwrap_or(usize::MAX);

                if out_idx != usize::MAX && in_idx != usize::MAX && out_idx != in_idx {
                    return Err(KernelError::TopologyViolation {
                        err: forge_core::TopologyError::BrokenLoop {
                            starting_halfedge: he_id.index(),
                            face_index: 0,
                        },
                        context: Some(forge_core::ErrorContext {
                            scope: forge_core::ErrorScope::Entity {
                                entity_kind: "Vertex".to_string(),
                                index: vid.index(),
                            },
                            suggested_fixes: Vec::new(),
                            detail: format!(
                                "Cross-disk co-edge detected at vertex {}. \
                                Halfedge {} and its incoming co-edge's twin {} are in different disks.",
                                vid.index(), he_id.index(), incoming_twin_candidate.index()
                            ),
                        }),
                    });
                }
            }
        }
    }

    Ok(())
}
