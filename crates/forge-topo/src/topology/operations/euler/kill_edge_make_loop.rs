//! KillEdgeMakeLoop (KEML) — split a loop by removing an edge.
//!
//! DOMAIN: Given an edge whose twin halfedges are on the same face
//! (a "same-face edge" created by MEKL), remove the edge and split
//! the surrounding loop into two separate loops. The original loop
//! survives; a new inner loop is created from the partitioned portion.
//!
//! INVARIANTS:
//! - The edge's halfedges must be on the same face
//! - The edge must separate two loop components (not a wire edge from MEV)
//! - Removes: 2 halfedges, 1 edge.  Creates: 1 loop.
//! - Euler delta: V=0, HE-2, E-1, L+1, F=0
//!
//! DEPENDENCIES: `arena`, `handles`, `lineage`, `operator`

use forge_core::{KernelError, TopologyError};

use crate::arena::LoopData;
use crate::handles::{HalfEdgeId, LoopId};
use crate::lineage::OpSignature;
use crate::state::MutableDraft;
use crate::EulerOperator;
use crate::operator::{ExecutionResult, EulerDelta};

/// Remove an edge to split a loop into two loops (outer + new inner).
///
/// `edge` is one halfedge of the edge to remove. Its twin must be on
/// the same face. After removal, the halfedges that were reachable
/// from `edge.next` (skipping the removed edge) form a new inner loop,
/// while the halfedges reachable from `twin.next` remain as the
/// outer loop.
#[derive(Debug)]
pub struct KillEdgeMakeLoop {
    /// One halfedge of the edge to remove.
    pub edge: HalfEdgeId,
}

/// Output of the KEML operator.
pub struct KemlOutput {
    /// The newly created inner loop.
    pub new_loop: LoopId,
}

impl EulerOperator for KillEdgeMakeLoop {
    type Output = KemlOutput;

    fn execute(&self, draft: &mut MutableDraft, sig: &OpSignature) -> Result<ExecutionResult<Self::Output>, KernelError> {
        let he_data = draft.arena().get_half_edge(self.edge)?;
        let twin_id = he_data.radial_next();
        let face = he_data.face();
        let he_prev = he_data.prev();
        let he_next = he_data.next();
        let edge_id = he_data.edge();

        let twin_data = draft.arena().get_half_edge(twin_id)?;
        let twin_face = twin_data.face();
        let twin_prev = twin_data.prev();
        let twin_next = twin_data.next();

        let valence = crate::topology::queries::traverse::radial_valence(draft.arena(), self.edge)?;
        if valence != 2 {
            return Err(KernelError::InvalidInput {
                message: format!("KEML: edge has radial valence {}, must be exactly 2", valence),
                context: None,
            });
        }

        // ── Validate: both halfedges on the same face ───────────────
        if face != twin_face {
            return Err(KernelError::InvalidInput {
                message: format!(
                    "KEML: edge halfedges must be on the same face. \
                     he ({}) on face {}, twin ({}) on face {}",
                    self.edge.index(), face.index(),
                    twin_id.index(), twin_face.index()
                ),
                context: None,
            });
        }

        // ── Validate: not a wire edge (self-loop twins from MEV) ────
        //
        // A wire edge from MEV has he.next == twin (the antenna pattern).
        // KEML targets MEKL bridge edges, not MEV antenna edges.
        // Wire edges should be removed with KEV, not KEML.
        if he_next == twin_id && twin_next == self.edge {
            return Err(KernelError::InvalidInput {
                message: format!(
                    "KEML: edge ({}, {}) is a wire edge (antenna). \
                     Use KillEdgeVertex (KEV) to remove wire edges.",
                    self.edge.index(), twin_id.index()
                ),
                context: None,
            });
        }

        // ── Unsplice the edge ───────────────────────────────────────
        //
        // Before: ...→ he_prev → [he] → he_next → ...
        //         ...→ twin_prev → [twin] → twin_next → ...
        //
        // After:  ...→ he_prev → twin_next → ...  (chain A: contains twin_next)
        //         ...→ twin_prev → he_next → ...  (chain B: contains he_next)
        let arena = draft.arena_mut();
        arena.get_half_edge_mut(he_prev)?.set_next(twin_next);
        arena.get_half_edge_mut(twin_next)?.set_prev(he_prev);
        arena.get_half_edge_mut(twin_prev)?.set_next(he_next);
        arena.get_half_edge_mut(he_next)?.set_prev(twin_prev);

        // ── Determine which chain becomes the new inner loop ────────
        //
        // Convention: chain A (containing twin_next, starting from he_prev)
        // stays as the outer loop. Chain B (containing he_next, starting
        // from twin_prev) becomes the new inner loop.
        //
        // This matches the MEKL inverse: MEKL splices he_ab before he_a
        // and he_ba before he_b. Removing he_ab(=self.edge) and
        // he_ba(=twin_id) reverses that splice. The chain starting at
        // he_next (which was he_b, the inner loop anchor in MEKL)
        // becomes the restored inner loop.

        // ── Create new inner loop ───────────────────────────────────
        let new_loop = draft.insert_loop(LoopData::new(he_next, face));

        // ── Update outer loop entry point ───────────────────────────
        let outer_loop = draft.arena().get_face(face)?.outer_loop();
        draft.arena_mut().get_loop_mut(outer_loop)?.set_half_edge(twin_next);

        // ── Register inner loop on face ─────────────────────────────
        draft.arena_mut().get_face_mut(face)?.add_inner_loop(new_loop);

        // ── Remove edge and halfedges ───────────────────────────────
        draft.remove_half_edge(self.edge)?;
        draft.remove_half_edge(twin_id)?;
        draft.remove_edge(edge_id)?;

        Ok(ExecutionResult {
            value: KemlOutput { new_loop },
            declared_delta: EulerDelta {
                vertices: 0,
                half_edges: -2,
                faces: 0,
                loops: 1,
                edges: -1,
                shells: 0,
                solids: 0,
                lumps: 0,
                regions: 0,
            },
        })
    }

    fn signature(&self) -> OpSignature {
        OpSignature::new("kill_edge_make_loop")
    }
}
