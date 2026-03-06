//! MakeEdgeKillLoop (MEKL) — merge two loops by inserting an edge.
//!
//! DOMAIN: Given a face with an outer loop and at least one inner loop,
//! insert an edge between a vertex on the outer loop and a vertex on
//! an inner loop, absorbing the inner loop into the outer boundary.
//!
//! INVARIANTS:
//! - `he_a` must originate from a vertex on the face's outer OR inner loop
//! - `he_b` must originate from a vertex on one of the face's inner loops
//! - Both halfedges must belong to the same face but different loops
//! - Creates: 2 halfedges, 1 edge.  Kills: 1 loop.
//! - Euler delta: V=0, HE+2, E+1, L-1, F=0
//!
//! DEPENDENCIES: `arena`, `handles`, `lineage`, `operator`

use forge_core::{KernelError, TopologyError};

use crate::b_rep::{EdgeData, HalfEdgeData};
use crate::handles::{HalfEdgeId, LoopId};
use crate::operator::{EulerDelta, ExecutionResult};
use crate::transactions::MutableDraft;
use crate::operator::TopoOperator;
use crate::validators::invariant_id::InvariantContract;


/// Merge two loops on the same face by inserting an edge between them.
///
/// `he_a` is the anchor on any loop (outer or inner). The new edge's halfedge
/// `he_ab` will be spliced immediately before `he_a` in the loop order.
///
/// `he_b` is the anchor on an **inner** loop. The new edge's halfedge
/// `he_ba` will be spliced immediately before `he_b` in the loop order.
///
/// After execution, the inner loop `he_b` belonged to is killed and its
/// halfedges become part of the loop containing `he_a`.
#[derive(Debug)]
pub struct MakeEdgeKillLoop {
    /// Half-edge on the surviving loop (anchor for the outer/first splice point).
    pub he_a: HalfEdgeId,
    /// Half-edge on an inner loop (anchor for the second splice point, loop will be killed).
    pub he_b: HalfEdgeId,
}

/// Output of the MEKL operator.
pub struct MeklOutput {
    /// Half-edge from outer vertex into the inner loop (origin = he_a.origin).
    pub he_ab: HalfEdgeId,
    /// Half-edge from inner vertex back to the outer loop (origin = he_b.origin).
    pub he_ba: HalfEdgeId,
    /// The newly created edge entity.
    pub edge: crate::handles::EdgeId,
    /// The loop that was killed (now removed from the arena).
    pub killed_loop: LoopId,
}

impl TopoOperator for MakeEdgeKillLoop {
    type Output = MeklOutput;

    const NAME: &'static str = "make_edge_kill_loop";

    const INVARIANT_CONTRACT: InvariantContract = crate::validators::contract_registry::FULL_TOPO_WIRING;

    fn semantic_summary(&self) -> String {
        format!(
            "Bridge outer halfedge {} to inner halfedge {}, killing inner loop",
            self.he_a.index(), self.he_b.index()
        )
    }

    fn execute(&self, draft: &mut MutableDraft, _recorder: &mut crate::provenance::LineageRecorder) -> Result<ExecutionResult<Self::Output>, KernelError> {
        let face_a = draft.arena().get_half_edge(self.he_a)?.face();
        let face_b = draft.arena().get_half_edge(self.he_b)?.face();

        if face_a != face_b {
            return Err(KernelError::InvalidInput {
                message: format!(
                    "MEKL: he_a (face {}) and he_b (face {}) must be on the same face",
                    face_a.index(),
                    face_b.index()
                ),
                context: None,
            });
        }

        let face = face_a;
        let outer_loop = draft.arena().get_face(face)?.outer_loop();
        let loop_a_id = find_loop_containing(draft, face, self.he_a)?;
        let loop_b_id = find_loop_containing(draft, face, self.he_b)?;

        if loop_a_id == loop_b_id {
            return Err(KernelError::InvalidInput {
                message: "MEKL: he_a and he_b must be on DIFFERENT loops of the same face. (Use MakeEdgeFace for splitting a single loop)".into(),
                context: None,
            });
        }

        if loop_b_id == outer_loop {
            return Err(KernelError::InvalidInput {
                message: "MEKL: he_b must be on an inner loop, so it can be safely killed.".into(),
                context: None,
            });
        }

        // ── Read splice points ──────────────────────────────────────
        let prev_a = draft.arena().get_half_edge(self.he_a)?.prev();
        let prev_b = draft.arena().get_half_edge(self.he_b)?.prev();
        let vertex_a = draft.arena().get_half_edge(self.he_a)?.origin();
        let vertex_b = draft.arena().get_half_edge(self.he_b)?.origin();

        // ── Derive lineage ──────────────────────────────────────────
        // ── Create edge + halfedge pair ─────────────────────────────
        let placeholder_he = HalfEdgeId::DANGLING;

        let new_edge =
            draft.insert_edge(EdgeData::new(placeholder_he));

        let (he_ab, he_ba) = draft.insert_radial_pair(
            HalfEdgeData::new(
                placeholder_he, // twin (fixed below by insert_half_edge_pair)
                self.he_b,      // next → into inner loop
                prev_a,         // prev → was before he_a
                face,
                vertex_a,
                new_edge,
            ),
            HalfEdgeData::new(
                placeholder_he,
                self.he_a, // next → back to outer loop
                prev_b,    // prev → was before he_b
                face,
                vertex_b,
                new_edge,
            ),
        );

        // ── Splice into loop ────────────────────────────────────────
        {
            let arena = draft.arena_mut();

            arena.get_half_edge_mut(prev_a)?.set_next(he_ab);
            arena.get_half_edge_mut(self.he_b)?.set_prev(he_ab);
            arena.get_half_edge_mut(prev_b)?.set_next(he_ba);
            arena.get_half_edge_mut(self.he_a)?.set_prev(he_ba);

            // ── Update edge representative ──────────────────────────────
            arena.get_edge_mut(new_edge)?.set_half_edge(he_ab);

            // ── Update surviving loop entry point (may have been prev_a) ────
            arena.get_loop_mut(loop_a_id)?.set_half_edge(he_ab);

            // ── Kill inner loop ─────────────────────────────────────────
            arena.get_face_mut(face)?.remove_inner_loop(loop_b_id);
        }
        draft.remove_loop(loop_b_id)?;

        Ok(ExecutionResult {
            value: MeklOutput {
                he_ab,
                he_ba,
                edge: new_edge,
                killed_loop: loop_b_id,
            },
            declared_delta: EulerDelta {
                vertices: 0,
                half_edges: 2,
                faces: 0,
                loops: -1,
                edges: 1,
                shells: 0,
                solids: 0,
                lumps: 0,
                regions: 0,
            },
        })
    }


}

/// Walk a loop to check if a specific halfedge belongs to it.
fn is_halfedge_on_loop(
    draft: &MutableDraft,
    loop_id: LoopId,
    target_he: HalfEdgeId,
) -> Result<bool, KernelError> {
    let start = draft.arena().get_loop(loop_id)?.half_edge();
    let bound = draft.arena().half_edge_count();
    let mut current = start;

    for step in 0..=bound {
        if current == target_he {
            return Ok(true);
        }
        current = draft.arena().get_half_edge(current)?.next();
        if current == start {
            return Ok(false);
        }
        if step == bound {
            return Err(KernelError::TopologyViolation {
                err: TopologyError::LoopCorruption {
                    walk_kind: "mekl_loop_check".into(),
                    seed_index: start.index(),
                    last_visited_index: current.index(),
                    steps_taken: step,
                    entity_bound: bound,
                },
                context: None,
            });
        }
    }

    Ok(false)
}

/// Find which loop (outer or inner) of `face` contains `target_he`.
fn find_loop_containing(
    draft: &MutableDraft,
    face: crate::handles::FaceId,
    target_he: HalfEdgeId,
) -> Result<LoopId, KernelError> {
    let outer_loop = draft.arena().get_face(face)?.outer_loop();
    if is_halfedge_on_loop(draft, outer_loop, target_he)? {
        return Ok(outer_loop);
    }

    let inner_loops: Vec<LoopId> = draft.arena().get_face(face)?.inner_loops().to_vec();

    for loop_id in inner_loops {
        if is_halfedge_on_loop(draft, loop_id, target_he)? {
            return Ok(loop_id);
        }
    }

    Err(KernelError::InvalidInput {
        message: format!(
            "MEKL: halfedge {} not found in any loop of face {}",
            target_he.index(),
            face.index()
        ),
        context: None,
    })
}
