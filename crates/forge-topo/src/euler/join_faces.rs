//! JoinFaces — merge two faces by removing a shared edge.
//!
//! DOMAIN: Given a halfedge whose two sides border different faces,
//! remove the edge and merge the two faces into one.
//!
//! INVARIANTS:
//! - The two faces must be distinct
//! - Removes 2 halfedges, 1 face, 1 loop
//! - Euler formula: E-1, F-1 (net: same V-E+F)
//!
//! DEPENDENCIES: `arena` (entity storage), `lineage` (provenance)

use forge_core::KernelError;

use crate::handles::HalfEdgeId;
use crate::lineage::{Lineage, OpSignature};
use crate::operator::EulerOperator;
use crate::state::MutableDraft;

/// Merge two faces by removing a shared edge.
///
/// `edge` is a halfedge on the shared edge. Its face and
/// `edge.twin`'s face must be distinct. The twin's face is removed;
/// the edge's face survives with merged lineage.
#[derive(Debug)]
pub struct JoinFaces {
    /// A halfedge on the edge to remove. This halfedge's face survives.
    pub edge: HalfEdgeId,
}

/// Output of the JoinFaces operator.
pub struct JfOutput {
    /// The surviving face.
    pub surviving_face: crate::handles::FaceId,
}

impl EulerOperator for JoinFaces {
    type Output = JfOutput;

    fn execute(&self, draft: &mut MutableDraft, sig: &OpSignature) -> Result<Self::Output, KernelError> {
        let he = self.edge;
        let he_data = draft.arena().get_half_edge(he)?;
        let he_twin = he_data.twin;
        let he_next = he_data.next;
        let he_prev = he_data.prev;
        let face_survive = he_data.face;

        let twin_data = draft.arena().get_half_edge(he_twin)?;
        let twin_next = twin_data.next;
        let twin_prev = twin_data.prev;
        let face_remove = twin_data.face;

        if face_survive == face_remove {
            return Err(KernelError::InvalidInput {
                message: "JoinFaces: both sides of edge belong to the same face".to_string(),
                context: None,
            });
        }

        let survive_lineage = draft.arena().get_face(face_survive)?.lineage.clone();
        let remove_lineage = draft.arena().get_face(face_remove)?.lineage.clone();
        let merged_lineage = Lineage::merge(&survive_lineage, &remove_lineage, sig);

        let arena = draft.arena_mut();
        arena.get_half_edge_mut(he_prev)?.next = twin_next;
        arena.get_half_edge_mut(twin_next)?.prev = he_prev;
        arena.get_half_edge_mut(twin_prev)?.next = he_next;
        arena.get_half_edge_mut(he_next)?.prev = twin_prev;

        reassign_face(draft, twin_next, face_survive, he)?;

        let arena = draft.arena_mut();
        let loop_id = arena.get_face(face_survive)?.outer_loop;
        arena.get_loop_mut(loop_id)?.half_edge = he_next;
        arena.get_face_mut(face_survive)?.lineage = Some(merged_lineage);

        let remove_loop = draft.arena().get_face(face_remove)?.outer_loop;
        draft.arena_mut().remove_half_edge(he)?;
        draft.arena_mut().remove_half_edge(he_twin)?;
        draft.arena_mut().remove_loop(remove_loop)?;
        draft.arena_mut().remove_face(face_remove)?;

        Ok(JfOutput {
            surviving_face: face_survive,
        })
    }

    fn signature(&self) -> OpSignature {
        OpSignature::new("join_faces")
    }
}

/// Reassign all halfedges starting from `start` to `new_face`, stopping
/// when we reach `stop` (exclusive).
fn reassign_face(
    draft: &mut MutableDraft,
    start: HalfEdgeId,
    new_face: crate::handles::FaceId,
    stop: HalfEdgeId,
) -> Result<(), KernelError> {
    let mut current = start;
    loop {
        let arena = draft.arena_mut();
        arena.get_half_edge_mut(current)?.face = new_face;
        let next = arena.get_half_edge(current)?.next;
        current = next;
        if current == start || current == stop {
            break;
        }
    }
    Ok(())
}
