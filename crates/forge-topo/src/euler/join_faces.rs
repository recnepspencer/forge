//! JF — Join Faces.
//!
//! DOMAIN: Merges two adjacent faces by removing the shared edge between them.
//!
//! The surviving face absorbs the halfedges from the removed face.
//! The shared edge's halfedge pair and the removed face's loop are deleted.
//!
//! Lineage: The surviving face's lineage is updated to reflect the merge,
//! derived from the smaller-ID face (deterministic dominant parent).

use forge_core::KernelError;
use crate::handles::{FaceId, HalfEdgeId};
use crate::lineage::{Lineage, OpSignature};
use crate::operator::EulerOperator;
use crate::state::MutableDraft;

/// Maximum loop iterations for reassigning face ownership.
const MAX_LOOP_REASSIGN_ITERATIONS: usize = 10_000;

/// Merge two faces by removing the shared edge.
///
/// Given the halfedge `edge` that lies on the boundary between two faces,
/// this operator removes `edge` and its twin, merging the two faces into one.
/// The face on the `edge` side survives; the face on the twin side is deleted.
///
/// # Preconditions
/// - `edge` and its twin must border different faces
/// - Both faces must have more than one edge (can't remove the last edge)
#[derive(Debug)]
pub struct JoinFaces {
    /// The halfedge on the shared boundary. Its face survives.
    pub edge: HalfEdgeId,
}

impl EulerOperator for JoinFaces {
    type Output = FaceId;

    fn execute(&self, draft: &mut MutableDraft, sig: &OpSignature) -> Result<Self::Output, KernelError> {
        let he = self.edge;
        let he_data = draft.arena().get_half_edge(he)?.clone();
        let twin = he_data.twin;
        let twin_data = draft.arena().get_half_edge(twin)?.clone();

        let surviving_face = he_data.face;
        let removed_face = twin_data.face;

        if surviving_face == removed_face {
            return Err(KernelError::InvalidInput {
                message: "JoinFaces: edge and twin are on the same face".to_string(),
                context: None,
            });
        }

        let surviving_lineage = draft.arena().get_face(surviving_face)?.lineage.clone();
        let removed_lineage = draft.arena().get_face(removed_face)?.lineage.clone();
        let merged_lineage = merge_lineage(&surviving_lineage, &removed_lineage, sig);

        let he_prev = he_data.prev;
        let he_next = he_data.next;
        let twin_prev = twin_data.prev;
        let twin_next = twin_data.next;

        let arena = draft.arena_mut();

        arena.get_half_edge_mut(he_prev)?.next = twin_next;
        arena.get_half_edge_mut(twin_next)?.prev = he_prev;

        arena.get_half_edge_mut(twin_prev)?.next = he_next;
        arena.get_half_edge_mut(he_next)?.prev = twin_prev;

        reassign_face(arena, twin_next, surviving_face, twin)?;

        arena.get_face_mut(surviving_face)?.lineage = Some(merged_lineage);

        let surviving_loop = arena.get_face(surviving_face)?.outer_loop;
        arena.get_loop_mut(surviving_loop)?.half_edge = he_next;

        let removed_loop = arena.get_face(removed_face)?.outer_loop;
        arena.remove_loop(removed_loop)?;
        arena.remove_face(removed_face)?;

        arena.remove_half_edge(he)?;
        arena.remove_half_edge(twin)?;

        Ok(surviving_face)
    }

    fn signature(&self) -> OpSignature {
        OpSignature::new("join_faces")
    }
}

/// Merge lineage from two faces, using the deterministic dominant parent
/// (the one with the smaller ancestry_hash, for D1 determinism).
fn merge_lineage(a: &Option<Lineage>, b: &Option<Lineage>, sig: &OpSignature) -> Lineage {
    let dominant = match (a, b) {
        (Some(la), Some(lb)) => {
            if la.ancestry_hash <= lb.ancestry_hash { la } else { lb }
        }
        (Some(la), None) => la,
        (None, Some(lb)) => lb,
        (None, None) => return Lineage::root(0, sig.clone()),
    };
    Lineage::derive(dominant, sig.clone())
}

/// Walk the loop starting from `start_he` and set all halfedges' face to `face`.
/// Stops when we return to `start_he` or encounter `stop_he` (the removed edge).
fn reassign_face(
    arena: &mut crate::arena::TopologyArena,
    start_he: HalfEdgeId,
    face: FaceId,
    stop_he: HalfEdgeId,
) -> Result<(), KernelError> {
    let mut current = start_he;

    for _ in 0..MAX_LOOP_REASSIGN_ITERATIONS {
        arena.get_half_edge_mut(current)?.face = face;
        let next = arena.get_half_edge(current)?.next;
        if next == start_he || next == stop_he {
            return Ok(());
        }
        current = next;
    }

    Err(KernelError::InternalError {
        message: "Loop limit exceeded in reassign_face".to_string(),
        context: None,
    })
}
