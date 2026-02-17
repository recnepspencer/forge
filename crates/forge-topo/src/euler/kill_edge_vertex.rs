//! KillEdgeVertex — collapse an edge by merging its target vertex into its origin.
//!
//! DOMAIN: Given a halfedge (A→B), removes the edge and vertex B.
//! All halfedges that used B as origin are rewired to A.
//!
//! INVARIANTS:
//! - Removes 1 vertex, 2 halfedges (the edge pair)
//! - Euler formula: V-1, E-1 (net: same V-E+F)
//! - Surviving vertex gets merged lineage
//!
//! DEPENDENCIES: `arena` (entity storage), `lineage` (provenance)

use forge_core::KernelError;

use crate::handles::HalfEdgeId;
use crate::lineage::{Lineage, OpSignature};
use crate::operator::EulerOperator;
use crate::state::MutableDraft;

/// Collapse an edge by removing it and merging its target vertex into the origin.
///
/// `edge` is a halfedge A→B. Vertex B is removed; all references to B
/// become references to A. The edge (both halfedges) is removed.
#[derive(Debug)]
pub struct KillEdgeVertex {
    /// The halfedge to kill. Its target vertex (twin's origin) is collapsed.
    pub edge: HalfEdgeId,
}

/// Output of the KillEdgeVertex operator.
pub struct KevOutput {
    /// The surviving vertex (the origin of `edge`).
    pub surviving_vertex: crate::handles::VertexId,
}

impl EulerOperator for KillEdgeVertex {
    type Output = KevOutput;

    fn execute(&self, draft: &mut MutableDraft, sig: &OpSignature) -> Result<Self::Output, KernelError> {
        let he = self.edge;
        let he_data = draft.arena().get_half_edge(he)?;
        let he_twin = he_data.twin;
        let he_next = he_data.next;
        let he_prev = he_data.prev;
        let vertex_a = he_data.origin;

        let twin_data = draft.arena().get_half_edge(he_twin)?;
        let twin_next = twin_data.next;
        let twin_prev = twin_data.prev;
        let vertex_b = twin_data.origin;

        let is_self_loop = he == he_twin;

        if is_self_loop {
            return Err(KernelError::InvalidInput {
                message: "Cannot KillEdgeVertex on a self-loop halfedge".into(),
                context: None,
            });
        }

        let v_a_lineage = draft.arena().get_vertex(vertex_a)?.lineage.clone();
        let v_b_lineage = draft.arena().get_vertex(vertex_b)?.lineage.clone();
        let merged_lineage = Lineage::merge(&v_a_lineage, &v_b_lineage, sig);

        // ── Last-edge collapse early return ─────────────────────────
        // When he_next == he_twin, this is the only edge pair around vertex_a.
        // Removing it must restore the seed topology: a self-loop halfedge.
        // This is the exact inverse of SplitEdge on a self-loop.
        let is_last_edge = he_next == he_twin;

        if is_last_edge {
            use crate::arena::HalfEdgeData;

            let he_face = draft.arena().get_half_edge(he)?.face;
            let arena = draft.arena_mut();

            let self_loop = arena.insert_half_edge(HalfEdgeData {
                twin: crate::handles::HalfEdgeId::new(u32::MAX, 0),
                next: crate::handles::HalfEdgeId::new(u32::MAX, 0),
                prev: crate::handles::HalfEdgeId::new(u32::MAX, 0),
                face: he_face,
                origin: vertex_a,
                lineage: Some(merged_lineage),
            });

            arena.get_half_edge_mut(self_loop)?.twin = self_loop;
            arena.get_half_edge_mut(self_loop)?.next = self_loop;
            arena.get_half_edge_mut(self_loop)?.prev = self_loop;

            arena.get_vertex_mut(vertex_a)?.outgoing = self_loop;
            arena.get_vertex_mut(vertex_a)?.lineage = Some(
                Lineage::merge(&v_a_lineage, &v_b_lineage, sig),
            );

            let loop_id = arena.get_face(he_face)?.outer_loop;
            arena.get_loop_mut(loop_id)?.half_edge = self_loop;

            arena.remove_half_edge(he)?;
            arena.remove_half_edge(he_twin)?;
            arena.remove_vertex(vertex_b)?;

            return Ok(KevOutput {
                surviving_vertex: vertex_a,
            });
        }

        // ── General case ────────────────────────────────────────────
        let arena = draft.arena_mut();

        arena.get_half_edge_mut(he_prev)?.next = twin_next;
        arena.get_half_edge_mut(twin_next)?.prev = he_prev;

        arena.get_half_edge_mut(twin_prev)?.next = he_next;
        arena.get_half_edge_mut(he_next)?.prev = twin_prev;

        // Rewire any halfedges originating at vertex_b to originate at vertex_a
        let mut current = twin_next;
        let max_iter = 100_000;
        for _ in 0..max_iter {
            let cur_data = arena.get_half_edge(current)?;
            if cur_data.origin == vertex_b {
                arena.get_half_edge_mut(current)?.origin = vertex_a;
            }
            let next = arena.get_half_edge(current)?.twin;
            current = arena.get_half_edge(next)?.next;
            if current == twin_next {
                break;
            }
        }

        // Update surviving vertex outgoing (pick a valid halfedge)
        arena.get_vertex_mut(vertex_a)?.outgoing = he_next;
        arena.get_vertex_mut(vertex_a)?.lineage = Some(merged_lineage);

        // Update loop entries if they pointed to removed halfedges
        let he_face = arena.get_half_edge(he)?.face;
        let he_loop_id = arena.get_face(he_face)?.outer_loop;
        let loop_he = arena.get_loop(he_loop_id)?.half_edge;
        if loop_he == he || loop_he == he_twin {
            arena.get_loop_mut(he_loop_id)?.half_edge = he_next;
        }

        let twin_face = arena.get_half_edge(he_twin)?.face;
        if twin_face != he_face {
            let twin_loop_id = arena.get_face(twin_face)?.outer_loop;
            let twin_loop_he = arena.get_loop(twin_loop_id)?.half_edge;
            if twin_loop_he == he || twin_loop_he == he_twin {
                arena.get_loop_mut(twin_loop_id)?.half_edge = twin_next;
            }
        }

        // Remove the edge pair and the dead vertex
        arena.remove_half_edge(he)?;
        arena.remove_half_edge(he_twin)?;
        arena.remove_vertex(vertex_b)?;

        Ok(KevOutput {
            surviving_vertex: vertex_a,
        })
    }

    fn signature(&self) -> OpSignature {
        OpSignature::new("kill_edge_vertex")
    }
}
