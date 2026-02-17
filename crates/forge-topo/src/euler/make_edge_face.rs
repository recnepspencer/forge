//! MakeEdgeFace — split a face by inserting an edge between two vertices.
//!
//! DOMAIN: Given an existing face with two vertices on its boundary,
//! insert a new edge connecting them and split the face into two.
//!
//! INVARIANTS:
//! - Both vertices must lie on the same face loop
//! - Creates 2 new halfedges, 1 new face, 1 new loop
//! - Euler formula: E+1, F+1 (net: same V-E+F)
//!
//! DEPENDENCIES: `arena` (entity storage), `lineage` (provenance)

use forge_core::KernelError;

use crate::arena::{FaceData, HalfEdgeData, LoopData};
use crate::handles::{FaceId, HalfEdgeId, LoopId, VertexId};
use crate::lineage::{Lineage, OpSignature};
use crate::operator::EulerOperator;
use crate::state::MutableDraft;

/// Split a face by inserting a new edge between two of its vertices.
///
/// Given face `face` and two vertices `vertex_a` and `vertex_b` on its
/// boundary, inserts edge (A→B) and splits the face. The original face
/// keeps the loop containing the new edge A→B. A new face gets the
/// loop containing B→A.
#[derive(Debug)]
pub struct MakeEdgeFace {
    /// The face to split.
    pub face: FaceId,
    /// First vertex of the new edge (must be on the face boundary).
    pub vertex_a: VertexId,
    /// Second vertex of the new edge (must be on the face boundary).
    pub vertex_b: VertexId,
}

/// Output of the MakeEdgeFace operator.
pub struct MefOutput {
    /// The new halfedge A→B (on the original face).
    pub half_edge_ab: HalfEdgeId,
    /// The new halfedge B→A (on the new face).
    pub half_edge_ba: HalfEdgeId,
    /// The newly created face (gets the B→A side).
    pub new_face: FaceId,
    /// The newly created loop for the new face.
    pub new_loop: LoopId,
}

impl EulerOperator for MakeEdgeFace {
    type Output = MefOutput;

    fn execute(&self, draft: &mut MutableDraft, sig: &OpSignature) -> Result<Self::Output, KernelError> {
        let he_from_a = find_halfedge_from_vertex(draft, self.face, self.vertex_a)?;
        let he_from_b = find_halfedge_from_vertex(draft, self.face, self.vertex_b)?;

        let prev_a = draft.arena().get_half_edge(he_from_a)?.prev;
        let prev_b = draft.arena().get_half_edge(he_from_b)?.prev;

        let face_lineage = draft.arena().get_face(self.face)?.lineage.clone();
        let he_ab_lineage = Lineage::derive_from(&face_lineage, sig.clone());
        let he_ba_lineage = Lineage::derive_from(&face_lineage, sig.clone());
        let new_face_lineage = Lineage::derive_from(&face_lineage, sig.clone());

        let placeholder_he = HalfEdgeId::new(u32::MAX, 0);
        let placeholder_loop = LoopId::new(u32::MAX, 0);

        let new_face = draft.arena_mut().insert_face(FaceData {
            outer_loop: placeholder_loop,
            lineage: Some(new_face_lineage),
        });

        let new_loop = draft.arena_mut().insert_loop(LoopData {
            half_edge: placeholder_he,
            face: new_face,
        });

        let (he_ab, he_ba) = draft.arena_mut().insert_half_edge_pair(
            HalfEdgeData {
                twin: placeholder_he,
                next: he_from_b,
                prev: prev_a,
                face: self.face,
                origin: self.vertex_a,
                lineage: Some(he_ab_lineage),
            },
            HalfEdgeData {
                twin: placeholder_he,
                next: he_from_a,
                prev: prev_b,
                face: new_face,
                origin: self.vertex_b,
                lineage: Some(he_ba_lineage),
            },
        );

        let arena = draft.arena_mut();
        arena.get_half_edge_mut(prev_a)?.next = he_ab;
        arena.get_half_edge_mut(he_from_b)?.prev = he_ab;
        arena.get_half_edge_mut(prev_b)?.next = he_ba;
        arena.get_half_edge_mut(he_from_a)?.prev = he_ba;

        reassign_face_loop(draft, he_ba, new_face)?;

        let arena = draft.arena_mut();
        let original_loop = arena.get_face(self.face)?.outer_loop;
        arena.get_loop_mut(original_loop)?.half_edge = he_ab;
        arena.get_face_mut(new_face)?.outer_loop = new_loop;
        arena.get_loop_mut(new_loop)?.half_edge = he_ba;

        Ok(MefOutput {
            half_edge_ab: he_ab,
            half_edge_ba: he_ba,
            new_face,
            new_loop,
        })
    }

    fn signature(&self) -> OpSignature {
        OpSignature::new("make_edge_face")
    }
}

/// Find the halfedge originating from `vertex` on `face`.
fn find_halfedge_from_vertex(
    draft: &MutableDraft,
    face: FaceId,
    vertex: VertexId,
) -> Result<HalfEdgeId, KernelError> {
    let face_data = draft.arena().get_face(face)?;
    let loop_data = draft.arena().get_loop(face_data.outer_loop)?;
    let start = loop_data.half_edge;
    let mut current = start;

    loop {
        let he_data = draft.arena().get_half_edge(current)?;
        if he_data.origin == vertex {
            return Ok(current);
        }
        current = he_data.next;
        if current == start {
            return Err(KernelError::InvalidInput {
                message: format!("Vertex {} not found on face {}", vertex, face),
                context: None,
            });
        }
    }
}

/// Reassign all halfedges in a loop (starting from `start`) to `new_face`.
fn reassign_face_loop(
    draft: &mut MutableDraft,
    start: HalfEdgeId,
    new_face: FaceId,
) -> Result<(), KernelError> {
    let mut current = start;
    loop {
        let arena = draft.arena_mut();
        arena.get_half_edge_mut(current)?.face = new_face;
        let next = arena.get_half_edge(current)?.next;
        current = next;
        if current == start {
            break;
        }
    }
    Ok(())
}
