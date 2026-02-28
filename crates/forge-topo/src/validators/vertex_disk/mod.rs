//! Vertex-disk and umbrella invariant validators.
//!
//! DOMAIN: Vertex disk partition correctness, disk closure,
//! ordering determinism, cross-disk coedge detection, and
//! pinch-point consistency for NMT states.
//!
//! VALIDATORS (from validators.md §4):
//! - ValidateVertexDiskPartition
//! - ValidateDiskClosure
//! - ValidateDiskOrderingDeterminism
//! - ValidateNoCrossDiskCoedges
//! - ValidatePinchPointConsistency
//!
//! DEPENDENCIES: `arena`, `handles`

use crate::b_rep::TopologyArena;
use forge_core::KernelError;

/// Validate that every vertex's outgoing halfedge is valid and points back to the vertex.
pub(crate) fn validate_vertex_outgoing(arena: &TopologyArena) -> Result<(), KernelError> {
    for (vid, v_data) in arena.iter_vertices() {
        let out = v_data.outgoing();

        let out_data =
            arena
                .get_half_edge(out)
                .map_err(|_| KernelError::TopologyViolation {
                    err: forge_core::TopologyError::BrokenLoop {
                        starting_halfedge: out.index(),
                        face_index: 0,
                    },
                    context: Some(forge_core::ErrorContext {
                        scope: forge_core::ErrorScope::Entity {
                            entity_kind: "Vertex".to_string(),
                            index: vid.index(),
                        },
                        suggested_fixes: Vec::new(),
                        detail: format!(
                            "Vertex {} outgoing halfedge {}(gen{}) is stale/deleted",
                            vid.index(),
                            out.index(),
                            out.generation()
                        ),
                    }),
                })?;

        if out_data.origin() != vid {
            return Err(KernelError::TopologyViolation {
                err: forge_core::TopologyError::BrokenLoop {
                    starting_halfedge: out.index(),
                    face_index: 0,
                },
                context: Some(forge_core::ErrorContext {
                    scope: forge_core::ErrorScope::Entity {
                        entity_kind: "Vertex".to_string(),
                        index: vid.index(),
                    },
                    suggested_fixes: Vec::new(),
                    detail: format!(
                        "Vertex {} outgoing halfedge {} has origin {} (should be {})",
                        vid.index(),
                        out.index(),
                        out_data.origin().index(),
                        vid.index()
                    ),
                }),
            });
        }
    }
    Ok(())
}
