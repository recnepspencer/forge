//! JoinFacesNmt — NMT-compatible face merge.
//!
//! DOMAIN: Given two halfedges sharing a geometric edge in a high-valence
//! radial ring (>2), merge their faces. The shared topological edge is
//! converted into a same-face "slit" (a twin pair) in the surviving face.
//! The remaining non-selected halfedges in the radial ring are re-wired
//! to bypass the slit, preserving the "protected" cyclic structure.
//!
//! INVARIANTS:
//! - Global radial valence must be > 2 (otherwise use standard JoinFaces).
//! - The two halfedges must belong to different faces but share the same edge entity.
//! - Removes 1 face, 1 loop. Unlike standard JoinFaces, it does NOT remove
//!   halfedges or the edge entity (it creates a slit instead).
//! - Euler formula is locally maintained for the surviving manifold regions,
//!   though NMT semantics apply.
//!
//! DEPENDENCIES: `arena`, `lineage`

use forge_core::{KernelError, TopologyError};

use crate::handles::{HalfEdgeId, LoopId};
use crate::operator::{EulerDelta, ExecutionResult};
use crate::state::MutableDraft;
use crate::EulerOperator;


/// NMT-compatible face merge that leaves a topological slit.
///
/// Merges `killed_face` into `surviving_face`. The boundary halfedges
/// `he_survive` and `he_kill` are retained as a twin pair (slit) in the
/// surviving face. Non-selected uses of the edge remain structurally intact.
#[derive(Debug)]
pub struct JoinFacesNmt {
    /// Halfedge belonging to the face that will survive.
    pub he_survive: HalfEdgeId,
    /// Halfedge belonging to the face that will be removed.
    pub he_kill: HalfEdgeId,
}

/// Output of the JoinFacesNmt operator.
#[derive(Debug)]
pub struct JfNmtOutput {
    /// The surviving face.
    pub surviving_face: crate::handles::FaceId,
}

impl EulerOperator for JoinFacesNmt {
    type Output = JfNmtOutput;

    const NAME: &'static str = "join_faces_nmt";

    fn execute(
        &self,
        draft: &mut MutableDraft,
    ) -> Result<ExecutionResult<Self::Output>, KernelError> {
        let he_s = self.he_survive;
        let he_k = self.he_kill;

        let s_data = draft.arena().get_half_edge(he_s)?;
        let k_data = draft.arena().get_half_edge(he_k)?;

        let face_survive = s_data.face();
        let face_kill = k_data.face();
        let edge_id = s_data.edge();

        if face_survive == face_kill {
            return Err(KernelError::InvalidInput {
                message: "JoinFacesNmt: both halfedges belong to the same face. Slit collapse is not supported by this operator.".into(),
                context: None,
            });
        }
        if edge_id != k_data.edge() {
            return Err(KernelError::InvalidInput {
                message: "JoinFacesNmt: halfedges do not share the same geometric edge.".into(),
                context: None,
            });
        }

        let valence = crate::topology::queries::traverse::radial_valence(draft.arena(), he_s)?;
        if valence <= 2 {
            return Err(KernelError::InvalidInput {
                message: format!("JoinFacesNmt: edge has radial valence {}, must be > 2. Use standard JoinFaces instead.", valence),
                context: None,
            });
        }

        // Collect the full radial ring in order (starting with he_s).
        let mut ring = Vec::new();
        let mut curr = he_s;
        loop {
            ring.push(curr);
            curr = draft.arena().get_half_edge(curr)?.radial_next();
            if curr == he_s {
                break;
            }
        }

        // Validate that he_k is actually in this ring (implied by edge_id equality and structure, but double-check).
        if !ring.contains(&he_k) {
            return Err(KernelError::InvalidInput {
                message: "JoinFacesNmt: he_kill is not in the same radial ring as he_survive."
                    .into(),
                context: None,
            });
        }

        // 1. Surgery on the protected radial ring.
        // We must remove he_s and he_k from the cyclic list, while keeping the rest intact.
        // The simplest way: filter out he_s and he_k, and wire the remaining ones in order.
        let protected: Vec<HalfEdgeId> = ring
            .into_iter()
            .filter(|&h| h != he_s && h != he_k)
            .collect();
        if !protected.is_empty() {
            for i in 0..protected.len() {
                let this = protected[i];
                let next = protected[(i + 1) % protected.len()];
                draft
                    .arena_mut()
                    .get_half_edge_mut(this)?
                    .set_radial_next(next);
            }
        }

        // 2. Wire the slit pair to each other.
        draft
            .arena_mut()
            .get_half_edge_mut(he_s)?
            .set_radial_next(he_k);
        draft
            .arena_mut()
            .get_half_edge_mut(he_k)?
            .set_radial_next(he_s);

        // 3. Lineage merge.


        // 4. Reassign killed face's outer boundary halfedges to surviving face.
        // Wait: `he_k`'s next() loop gives us the outer boundary of the killed face.
        // We must reassign EVERYTHING in that loop to `face_survive`.
        reassign_face(draft, he_k, face_survive)?;

        // P10: Transfer inner loops from face_kill to face_survive.
        let inner_loops: Vec<LoopId> = draft.arena().get_face(face_kill)?.inner_loops().to_vec();
        for il_id in inner_loops {
            let inner_start = draft.arena().get_loop(il_id)?.half_edge();
            draft
                .arena_mut()
                .get_face_mut(face_kill)?
                .remove_inner_loop(il_id);
            draft
                .arena_mut()
                .get_face_mut(face_survive)?
                .add_inner_loop(il_id);
            draft
                .arena_mut()
                .get_loop_mut(il_id)?
                .set_face(face_survive);
            reassign_face(draft, inner_start, face_survive)?;
        }

        // 5. Wire next/prev pointers to merge the two face-loops into one
        //    outer loop, while isolating the slit into a 2-element inner loop.
        //
        //    Before: FaceS loop: …→ he_s_prev → he_s → he_s_next →…
        //            FaceK loop: …→ he_k_prev → he_k → he_k_next →…
        //    After:  Outer loop: …→ he_s_prev → he_k_next →…→ he_k_prev → he_s_next →…
        //            Slit loop:  he_s ↔ he_k  (inner loop of surviving face)
        let he_s_prev = draft.arena().get_half_edge(he_s)?.prev();
        let he_s_next = draft.arena().get_half_edge(he_s)?.next();
        let he_k_prev = draft.arena().get_half_edge(he_k)?.prev();
        let he_k_next = draft.arena().get_half_edge(he_k)?.next();

        // Bypass the slit in the outer boundary.
        draft
            .arena_mut()
            .get_half_edge_mut(he_s_prev)?
            .set_next(he_k_next);
        draft
            .arena_mut()
            .get_half_edge_mut(he_k_next)?
            .set_prev(he_s_prev);

        draft
            .arena_mut()
            .get_half_edge_mut(he_k_prev)?
            .set_next(he_s_next);
        draft
            .arena_mut()
            .get_half_edge_mut(he_s_next)?
            .set_prev(he_k_prev);

        // Wire the slit as a 2-element closed loop.
        draft.arena_mut().get_half_edge_mut(he_s)?.set_next(he_k);
        draft.arena_mut().get_half_edge_mut(he_k)?.set_prev(he_s);
        draft.arena_mut().get_half_edge_mut(he_k)?.set_next(he_s);
        draft.arena_mut().get_half_edge_mut(he_s)?.set_prev(he_k);

        // 6. Fix vertex outgoing pointers away from the slit.
        //
        // vertex_s = he_s.origin(), vertex_k = he_k.origin().
        // Both he_s and he_k are now in the slit (inner loop). If a vertex's
        // outgoing pointer points to a slit halfedge, we must replace it with
        // a non-slit halfedge that has the same origin.
        //
        // Loop processes both vertices unconditionally. When vertex_s == vertex_k,
        // the second iteration is safe (no-op since the first already fixed it).
        // This avoids the asymmetry bug where the old branching code skipped
        // vertex_k fixup when vertex_s == vertex_k and outgoing == he_k.
        let vertex_s = draft.arena().get_half_edge(he_s)?.origin();
        let vertex_k = draft.arena().get_half_edge(he_k)?.origin();

        for &target_vertex in &[vertex_s, vertex_k] {
            let current_out = draft.arena().get_vertex(target_vertex)?.outgoing();
            if current_out == he_s || current_out == he_k {
                let replacement =
                    find_non_slit_outgoing(draft, target_vertex, he_s, he_k, &protected)?;
                draft
                    .arena_mut()
                    .get_vertex_mut(target_vertex)?
                    .set_outgoing(replacement);
            }
        }

        // 7. Register the slit as a new inner loop on the surviving face.
        let new_inner_loop = draft.insert_loop(crate::arena::LoopData::new(he_s, face_survive));
        draft
            .arena_mut()
            .get_face_mut(face_survive)?
            .add_inner_loop(new_inner_loop);

        // 7b. Fix EdgeData.half_edge pointer.
        // After slit creation, the EdgeData may point to he_s or he_k (now in the
        // slit ring). Code that enters the radial ring via EdgeData.half_edge()
        // (e.g. continuity queries) would see only the 2-element slit instead of
        // the protected ring. Point it to a protected-ring halfedge.
        if !protected.is_empty() {
            draft
                .arena_mut()
                .get_edge_mut(edge_id)?
                .set_half_edge(protected[0]);
        }

        // 8. Remove the killed face and its outer loop.
        let loop_kill = draft.arena().get_face(face_kill)?.outer_loop();
        draft.remove_loop(loop_kill)?;
        draft.remove_face(face_kill)?;

        // 9. Ensure the surviving face's outer loop points into the merged ring.
        let loop_survive = draft.arena().get_face(face_survive)?.outer_loop();
        draft
            .arena_mut()
            .get_loop_mut(loop_survive)?
            .set_half_edge(he_s_next);

        Ok(ExecutionResult {
            value: JfNmtOutput {
                surviving_face: face_survive,
            },
            declared_delta: EulerDelta {
                vertices: 0,
                half_edges: 0,
                faces: -1,
                loops: 0,
                edges: 0,
                shells: 0,
                solids: 0,
                lumps: 0,
                regions: 0,
            },
        })
    }


}

/// Reassign all halfedges starting from `start` to `new_face`.
///
/// Walks the loop via `next()` until returning to `start`.
fn reassign_face(
    draft: &mut MutableDraft,
    start: HalfEdgeId,
    new_face: crate::handles::FaceId,
) -> Result<(), KernelError> {
    let bound = draft.arena().half_edge_count();
    let mut current = start;
    let mut steps = 0usize;
    loop {
        draft
            .arena_mut()
            .get_half_edge_mut(current)?
            .set_face(new_face);
        let next = draft.arena().get_half_edge(current)?.next();
        current = next;
        if current == start {
            break;
        }
        steps += 1;
        if steps > bound {
            return Err(KernelError::TopologyViolation {
                err: TopologyError::LoopCorruption {
                    walk_kind: "reassign_face_nmt".into(),
                    seed_index: start.index(),
                    last_visited_index: current.index(),
                    steps_taken: steps,
                    entity_bound: bound,
                },
                context: None,
            });
        }
    }
    Ok(())
}

/// Find a non-slit halfedge originating at `target_vertex`.
///
/// Searches the protected ring first, then walks the merged outer loop.
fn find_non_slit_outgoing(
    draft: &MutableDraft,
    target_vertex: crate::handles::VertexId,
    slit_a: HalfEdgeId,
    slit_b: HalfEdgeId,
    protected: &[HalfEdgeId],
) -> Result<HalfEdgeId, KernelError> {
    for &p in protected {
        if draft.arena().get_half_edge(p)?.origin() == target_vertex {
            return Ok(p);
        }
    }

    // Fallback: walk all halfedges to find one with the correct origin.
    for (he_id, he_data) in draft.arena().iter_half_edges() {
        if he_id != slit_a && he_id != slit_b && he_data.origin() == target_vertex {
            return Ok(he_id);
        }
    }

    Err(KernelError::InvalidInput {
        message: format!(
            "JoinFacesNmt: cannot find non-slit outgoing halfedge for vertex {}",
            target_vertex.index()
        ),
        context: None,
    })
}
