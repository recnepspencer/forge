//! MakeEdgeKillLoop (MEKL) — merge two loops by inserting an edge.
//!
//! DOMAIN: Given a face with an outer loop and at least one inner loop,
//! insert an edge between a vertex on the outer loop and a vertex on
//! an inner loop, absorbing the inner loop into the outer boundary.
//!
//! INVARIANTS:
//! - `he_a` must originate from a vertex on the face's outer loop
//! - `he_b` must originate from a vertex on one of the face's inner loops
//! - Both halfedges must belong to the same face
//! - Creates: 2 halfedges, 1 edge.  Kills: 1 loop.
//! - Euler delta: V=0, HE+2, E+1, L-1, F=0
//!
//! DEPENDENCIES: `arena`, `handles`, `lineage`, `operator`

use forge_core::{KernelError, TopologyError};

use crate::arena::{HalfEdgeData, EdgeData};
use crate::handles::{HalfEdgeId, LoopId};
use crate::lineage::{Lineage, OpSignature};
use crate::state::MutableDraft;
use crate::EulerOperator;
use crate::operator::{ExecutionResult, EulerDelta};

/// Merge two loops on the same face by inserting an edge between them.
///
/// `he_a` is the anchor on the **outer** loop: the new edge's halfedge
/// `he_ab` will be spliced immediately before `he_a` in the loop order.
///
/// `he_b` is the anchor on an **inner** loop: the new edge's halfedge
/// `he_ba` will be spliced immediately before `he_b` in the loop order.
///
/// After execution, the inner loop is killed and its halfedges become
/// part of the outer loop.
#[derive(Debug)]
pub struct MakeEdgeKillLoop {
    /// Half-edge on the outer loop (anchor for the outer splice point).
    pub he_a: HalfEdgeId,
    /// Half-edge on an inner loop (anchor for the inner splice point).
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

impl EulerOperator for MakeEdgeKillLoop {
    type Output = MeklOutput;

    fn execute(&self, draft: &mut MutableDraft, sig: &OpSignature) -> Result<ExecutionResult<Self::Output>, KernelError> {
        let face_a = draft.arena().get_half_edge(self.he_a)?.face();
        let face_b = draft.arena().get_half_edge(self.he_b)?.face();

        if face_a != face_b {
            return Err(KernelError::InvalidInput {
                message: format!(
                    "MEKL: he_a (face {}) and he_b (face {}) must be on the same face",
                    face_a.index(), face_b.index()
                ),
                context: None,
            });
        }

        let face = face_a;
        let outer_loop = draft.arena().get_face(face)?.outer_loop();
        let inner_loop_id = find_loop_containing(draft, face, self.he_b)?;

        if inner_loop_id == outer_loop {
            return Err(KernelError::InvalidInput {
                message: "MEKL: he_b must be on an inner loop, not the outer loop".into(),
                context: None,
            });
        }

        let he_a_on_outer = is_halfedge_on_loop(draft, outer_loop, self.he_a)?;
        if !he_a_on_outer {
            return Err(KernelError::InvalidInput {
                message: format!(
                    "MEKL: he_a ({}) must be on the outer loop of face {}",
                    self.he_a.index(), face.index()
                ),
                context: None,
            });
        }

        // ── Read splice points ──────────────────────────────────────
        let prev_a = draft.arena().get_half_edge(self.he_a)?.prev();
        let prev_b = draft.arena().get_half_edge(self.he_b)?.prev();
        let vertex_a = draft.arena().get_half_edge(self.he_a)?.origin();
        let vertex_b = draft.arena().get_half_edge(self.he_b)?.origin();

        // ── Derive lineage ──────────────────────────────────────────
        let face_lineage = draft.arena().get_face(face)?.lineage().cloned();
        let he_ab_lineage = Lineage::derive_from(&face_lineage, sig.clone());
        let he_ba_lineage = Lineage::derive_from(&face_lineage, sig.clone());
        let edge_lineage = Lineage::derive_from(&face_lineage, sig.clone());

        // ── Create edge + halfedge pair ─────────────────────────────
        let placeholder_he = HalfEdgeId::new(u32::MAX, 0);

        let new_edge = draft.insert_edge(EdgeData::with_lineage(
            placeholder_he,
            Some(edge_lineage),
        ));

        let (he_ab, he_ba) = draft.insert_radial_pair(
            HalfEdgeData::with_lineage(
                placeholder_he,  // twin (fixed below by insert_half_edge_pair)
                self.he_b,       // next → into inner loop
                prev_a,          // prev → was before he_a
                face,
                vertex_a,
                new_edge,
                Some(he_ab_lineage),
            ),
            HalfEdgeData::with_lineage(
                placeholder_he,
                self.he_a,       // next → back to outer loop
                prev_b,          // prev → was before he_b
                face,
                vertex_b,
                new_edge,
                Some(he_ba_lineage),
            ),
        );

        // ── Splice into loop ────────────────────────────────────────
        let arena = draft.arena_mut();

        arena.get_half_edge_mut(prev_a)?.set_next(he_ab);
        arena.get_half_edge_mut(self.he_b)?.set_prev(he_ab);
        arena.get_half_edge_mut(prev_b)?.set_next(he_ba);
        arena.get_half_edge_mut(self.he_a)?.set_prev(he_ba);

        // ── Update edge representative ──────────────────────────────
        arena.get_edge_mut(new_edge)?.set_half_edge(he_ab);

        // ── Update outer loop entry point (may have been prev_a) ────
        arena.get_loop_mut(outer_loop)?.set_half_edge(he_ab);

        // ── Kill inner loop ─────────────────────────────────────────
        arena.get_face_mut(face)?.remove_inner_loop(inner_loop_id);
        drop(arena);
        draft.remove_loop(inner_loop_id)?;

        Ok(ExecutionResult {
            value: MeklOutput {
                he_ab,
                he_ba,
                edge: new_edge,
                killed_loop: inner_loop_id,
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

    fn signature(&self) -> OpSignature {
        OpSignature::new("make_edge_kill_loop")
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

    let inner_loops: Vec<LoopId> = draft.arena().get_face(face)?
        .inner_loops().to_vec();

    for loop_id in inner_loops {
        if is_halfedge_on_loop(draft, loop_id, target_he)? {
            return Ok(loop_id);
        }
    }

    Err(KernelError::InvalidInput {
        message: format!(
            "MEKL: halfedge {} not found in any loop of face {}",
            target_he.index(), face.index()
        ),
        context: None,
    })
}
