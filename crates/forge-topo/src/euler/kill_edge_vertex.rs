//! KEV — Kill Edge Vertex.
//!
//! DOMAIN: Removes an edge and one of its endpoint vertices, merging
//! the two edges around the removed vertex into one.
//!
//! This is the inverse of SplitEdge: it collapses a vertex of degree 2
//! by removing one edge and merging the connectivity.
//!
//! Lineage: The surviving vertex's lineage is updated to reflect the
//! collapse, derived from its existing lineage + the operation.

use forge_core::KernelError;
use crate::handles::HalfEdgeId;
use crate::lineage::{Lineage, OpSignature};
use crate::operator::EulerOperator;
use crate::state::MutableDraft;

/// Remove an edge and collapse one of its endpoint vertices.
///
/// Given halfedge `edge` (A → B), this operator removes the edge pair
/// and vertex B, merging the topology so that the halfedges that previously
/// originated from B now originate from A.
///
/// # Preconditions
/// - Vertex B (the target) must have degree 2 (exactly 2 outgoing halfedges)
/// - The edge must not be the last edge in the mesh
#[derive(Debug)]
pub struct KillEdgeVertex {
    /// The halfedge whose TARGET vertex (B) will be removed.
    /// The halfedge goes from A to B.
    pub edge: HalfEdgeId,
}

impl EulerOperator for KillEdgeVertex {
    type Output = ();

    fn execute(&self, draft: &mut MutableDraft, sig: &OpSignature) -> Result<Self::Output, KernelError> {
        let he = self.edge;
        let he_data = draft.arena().get_half_edge(he)?.clone();
        let twin = he_data.twin;
        let twin_data = draft.arena().get_half_edge(twin)?.clone();

        let vertex_a = he_data.origin;
        let vertex_b = twin_data.origin;

        let parent_lineage = draft.arena().get_vertex(vertex_a)?.lineage.clone();
        let updated_lineage = derive_or_root(&parent_lineage, sig);

        let he_next = he_data.next;
        let he_prev = he_data.prev;
        let twin_next = twin_data.next;
        let twin_prev = twin_data.prev;

        let arena = draft.arena_mut();

        arena.get_half_edge_mut(he_prev)?.next = he_next;
        arena.get_half_edge_mut(he_next)?.prev = he_prev;

        arena.get_half_edge_mut(twin_prev)?.next = twin_next;
        arena.get_half_edge_mut(twin_next)?.prev = twin_prev;

        retarget_vertex_origin(arena, vertex_b, vertex_a, he, twin)?;

        arena.get_vertex_mut(vertex_a)?.outgoing = he_next;
        arena.get_vertex_mut(vertex_a)?.lineage = Some(updated_lineage);

        let loop_id_he = arena.get_face(he_data.face)?.outer_loop;
        arena.get_loop_mut(loop_id_he)?.half_edge = he_next;
        let loop_id_twin = arena.get_face(twin_data.face)?.outer_loop;
        arena.get_loop_mut(loop_id_twin)?.half_edge = twin_next;

        arena.remove_half_edge(he)?;
        arena.remove_half_edge(twin)?;
        arena.remove_vertex(vertex_b)?;

        Ok(())
    }

    fn signature(&self) -> OpSignature {
        OpSignature::new("kill_edge_vertex")
    }
}

/// Derive a child lineage from a parent, or create root if parent has none.
fn derive_or_root(parent: &Option<Lineage>, sig: &OpSignature) -> Lineage {
    match parent {
        Some(p) => Lineage::derive(p, sig.clone()),
        None => Lineage::root(0, sig.clone()),
    }
}

/// Walk all halfedges originating from `old_vertex` and retarget them to `new_vertex`.
/// Skips `skip_a` and `skip_b` (the halfedges being removed).
fn retarget_vertex_origin(
    arena: &mut crate::arena::TopologyArena,
    old_vertex: crate::handles::VertexId,
    new_vertex: crate::handles::VertexId,
    skip_a: HalfEdgeId,
    skip_b: HalfEdgeId,
) -> Result<(), KernelError> {
    let mut to_retarget = Vec::new();
    for (he_id, he_data) in arena.iter_half_edges() {
        if he_data.origin == old_vertex && he_id != skip_a && he_id != skip_b {
            to_retarget.push(he_id);
        }
    }
    for he_id in to_retarget {
        arena.get_half_edge_mut(he_id)?.origin = new_vertex;
    }
    Ok(())
}
