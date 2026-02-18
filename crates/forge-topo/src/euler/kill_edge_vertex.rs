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
        let vertex_a = he_data.origin;

        let twin_data = draft.arena().get_half_edge(he_twin)?;
        let twin_next = twin_data.next;
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
        let is_last_edge = he_next == he_twin && twin_next == he;

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

        // 1. Gather all edges around B (excluding he_twin) that need their origin updated.
        // We MUST do this traversal before we alter the mesh connections.
        let mut edges_from_b = Vec::new();
        let he_next_initial = draft.arena().get_half_edge(he)?.next; 
        let mut curr = he_next_initial;
        let max_iter = 100_000;
        for _ in 0..max_iter {
            if curr == he_twin {
                break;
            }
            edges_from_b.push(curr);
            let curr_twin = draft.arena().get_half_edge(curr)?.twin;
            curr = draft.arena().get_half_edge(curr_twin)?.next;
        }

        let arena = draft.arena_mut();

        // 2. Update origins to A
        for edge_id in edges_from_b {
            arena.get_half_edge_mut(edge_id)?.origin = vertex_a;
        }

        // 3. Sequential Bypass
        // Remove `he` from its loop
        let he_prev = arena.get_half_edge(he)?.prev;
        let he_next = arena.get_half_edge(he)?.next;
        arena.get_half_edge_mut(he_prev)?.next = he_next;
        arena.get_half_edge_mut(he_next)?.prev = he_prev;

        // Remove `he_twin` from its loop (re-reading pointers accommodates spurs perfectly)
        let twin_prev = arena.get_half_edge(he_twin)?.prev;
        let twin_next = arena.get_half_edge(he_twin)?.next;
        arena.get_half_edge_mut(twin_prev)?.next = twin_next;
        arena.get_half_edge_mut(twin_next)?.prev = twin_prev;

        // 4. Update vertex A outgoing 
        // Because of the sequential bypass, `twin_next` is universally safe and not deleted.
        arena.get_vertex_mut(vertex_a)?.outgoing = twin_next;
        arena.get_vertex_mut(vertex_a)?.lineage = Some(merged_lineage);

        // 5. Update face loop pointers
        let he_face = arena.get_half_edge(he)?.face;
        let twin_face = arena.get_half_edge(he_twin)?.face;

        let he_loop_id = arena.get_face(he_face)?.outer_loop;
        let twin_loop_id = arena.get_face(twin_face)?.outer_loop;

        if he_loop_id == twin_loop_id {
            // Same loop (spur logic)
            let loop_he = arena.get_loop(he_loop_id)?.half_edge;
            if loop_he == he || loop_he == he_twin {
                arena.get_loop_mut(he_loop_id)?.half_edge = twin_next;
            }
        } else {
            // Different loops (standard face splitting logic)
            let loop_he = arena.get_loop(he_loop_id)?.half_edge;
            if loop_he == he || loop_he == he_twin {
                arena.get_loop_mut(he_loop_id)?.half_edge = he_next;
            }
            let loop_twin = arena.get_loop(twin_loop_id)?.half_edge;
            if loop_twin == he || loop_twin == he_twin {
                arena.get_loop_mut(twin_loop_id)?.half_edge = twin_next;
            }
        }

        // 6. Mark incident faces as dirty so the diff engine detects boundary changes
        arena.bump_face_version(he_face)?;
        if twin_face != he_face {
            arena.bump_face_version(twin_face)?;
        }

        // 7. Delete entities
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
