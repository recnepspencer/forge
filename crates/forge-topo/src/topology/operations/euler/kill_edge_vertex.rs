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

use forge_core::{KernelError, TopologyError};

use crate::arena::{HalfEdgeData, EdgeData};
use crate::handles::{HalfEdgeId, EdgeId};
use crate::lineage::{Lineage, OpSignature};
use crate::EulerOperator;
use crate::operator::{ExecutionResult, EulerDelta};
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
    /// Whether this collapse produced a degenerate self-loop halfedge.
    ///
    /// When `true`, the surviving vertex's outgoing halfedge has
    /// `twin == next == prev == self`. This is the same degenerate state
    /// as `MakeVertexFace`'s initial seed. Traverse code must handle
    /// `he.twin() == he` to avoid infinite loops.
    pub is_degenerate: bool,
}

impl EulerOperator for KillEdgeVertex {
    type Output = KevOutput;

    fn execute(&self, draft: &mut MutableDraft, sig: &OpSignature) -> Result<ExecutionResult<Self::Output>, KernelError> {
        let he = self.edge;
        let he_data = draft.arena().get_half_edge(he)?;
        let he_twin = he_data.twin();
        let he_next = he_data.next();
        let vertex_a = he_data.origin();

        let twin_data = draft.arena().get_half_edge(he_twin)?;
        let twin_next = twin_data.next();
        let vertex_b = twin_data.origin();

        let is_self_loop = he == he_twin;
        let killed_edge = he_data.edge();

        if is_self_loop {
            return Err(KernelError::InvalidInput {
                message: "Cannot KillEdgeVertex on a self-loop halfedge".into(),
                context: None,
            });
        }

        let v_a_lineage = draft.arena().get_vertex(vertex_a)?.lineage().cloned();
        let v_b_lineage = draft.arena().get_vertex(vertex_b)?.lineage().cloned();
        let merged_lineage = Lineage::merge(&v_a_lineage, &v_b_lineage, sig);

        // ── Last-edge collapse early return ─────────────────────────
        let is_last_edge = he_next == he_twin && twin_next == he;

        if is_last_edge {
            let he_face = draft.arena().get_half_edge(he)?.face();

            // Reuse `he` as the surviving self-loop. Wire it to point to itself
            // on all three pointer fields, and update origin/lineage to vertex_a.
            // Then remove `he_twin` and `vertex_b`; `killed_edge` is reused.
            {
                let arena = draft.arena_mut();
                arena.get_half_edge_mut(he)?.set_twin(he);
                arena.get_half_edge_mut(he)?.set_next(he);
                arena.get_half_edge_mut(he)?.set_prev(he);
                arena.get_half_edge_mut(he)?.set_origin(vertex_a);
                arena.get_half_edge_mut(he)?.set_lineage(Some(merged_lineage.clone()));
                arena.get_edge_mut(killed_edge)?.set_half_edge(he);
                arena.get_vertex_mut(vertex_a)?.set_outgoing(he);
                arena.get_vertex_mut(vertex_a)?.set_lineage(Some(merged_lineage));
                let loop_id = arena.get_face(he_face)?.outer_loop();
                arena.get_loop_mut(loop_id)?.set_half_edge(he);
            }

            draft.remove_half_edge(he_twin)?;
            draft.remove_vertex(vertex_b)?;

            return Ok(ExecutionResult {
                value: KevOutput {
                    surviving_vertex: vertex_a,
                    is_degenerate: true,
                },
                // Net: -1 HE (removed twin), -1 vertex, 0 edges (reused)
                declared_delta: EulerDelta { vertices: -1, half_edges: -1, faces: 0, loops: 0, edges: 0, shells: 0 },
            });
        }

        // ── General case ────────────────────────────────────────────

        let mut edges_from_b = Vec::new();
        let he_next_initial = draft.arena().get_half_edge(he)?.next(); 
        let mut curr = he_next_initial;
        let bound = draft.arena().half_edge_count();
        for step in 0..bound {
            if curr == he_twin {
                break;
            }
            edges_from_b.push(curr);
            let curr_twin = draft.arena().get_half_edge(curr)?.twin();
            curr = draft.arena().get_half_edge(curr_twin)?.next();
            if step + 1 == bound {
                return Err(KernelError::TopologyViolation {
                    err: TopologyError::LoopCorruption {
                        walk_kind: "kill_edge_vertex_orbit".into(),
                        seed_index: he_next_initial.index(),
                        last_visited_index: curr.index(),
                        steps_taken: step + 1,
                        entity_bound: bound,
                    },
                    context: None,
                });
            }
        }

        for edge_id in edges_from_b {
            draft.arena_mut().get_half_edge_mut(edge_id)?.set_origin(vertex_a);
        }

        let he_prev = draft.arena().get_half_edge(he)?.prev();
        let he_next = draft.arena().get_half_edge(he)?.next();
        draft.arena_mut().get_half_edge_mut(he_prev)?.set_next(he_next);
        draft.arena_mut().get_half_edge_mut(he_next)?.set_prev(he_prev);

        let twin_prev = draft.arena().get_half_edge(he_twin)?.prev();
        let twin_next = draft.arena().get_half_edge(he_twin)?.next();
        draft.arena_mut().get_half_edge_mut(twin_prev)?.set_next(twin_next);
        draft.arena_mut().get_half_edge_mut(twin_next)?.set_prev(twin_prev);

        draft.arena_mut().get_vertex_mut(vertex_a)?.set_outgoing(twin_next);
        draft.arena_mut().get_vertex_mut(vertex_a)?.set_lineage(Some(merged_lineage));

        let he_face = draft.arena().get_half_edge(he)?.face();
        let twin_face = draft.arena().get_half_edge(he_twin)?.face();

        let he_loop_id = draft.arena().get_face(he_face)?.outer_loop();
        let twin_loop_id = draft.arena().get_face(twin_face)?.outer_loop();

        if he_loop_id == twin_loop_id {
            let loop_he = draft.arena().get_loop(he_loop_id)?.half_edge();
            if loop_he == he || loop_he == he_twin {
                draft.arena_mut().get_loop_mut(he_loop_id)?.set_half_edge(twin_next);
            }
        } else {
            let loop_he = draft.arena().get_loop(he_loop_id)?.half_edge();
            if loop_he == he || loop_he == he_twin {
                draft.arena_mut().get_loop_mut(he_loop_id)?.set_half_edge(he_next);
            }
            let loop_twin = draft.arena().get_loop(twin_loop_id)?.half_edge();
            if loop_twin == he || loop_twin == he_twin {
                draft.arena_mut().get_loop_mut(twin_loop_id)?.set_half_edge(twin_next);
            }
        }

        draft.arena_mut().bump_face_version(he_face)?;
        if twin_face != he_face {
            draft.arena_mut().bump_face_version(twin_face)?;
        }

        draft.remove_half_edge(he)?;
        draft.remove_half_edge(he_twin)?;
        draft.remove_vertex(vertex_b)?;
        draft.remove_edge(killed_edge)?;

        Ok(ExecutionResult {
            value: KevOutput {
                surviving_vertex: vertex_a,
                is_degenerate: false,
            },
            declared_delta: EulerDelta { vertices: -1, half_edges: -2, faces: 0, loops: 0, edges: -1, shells: 0 },
        })
    }

    fn signature(&self) -> OpSignature {
        OpSignature::new("kill_edge_vertex")
    }
}
