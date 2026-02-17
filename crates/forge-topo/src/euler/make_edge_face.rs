//! MEF — Make Edge Face.
//!
//! DOMAIN: Splits an existing face by connecting two vertices with a new edge,
//! creating a new face on one side.
//!
//! This is the primary face-building operator. Given two vertices that lie on
//! the boundary of the same face, it inserts an edge between them, splitting
//! the face into two.

use forge_core::KernelError;
use crate::arena::{FaceData, HalfEdgeData, LoopData};
use crate::handles::{FaceId, HalfEdgeId, VertexId, LoopId};
use crate::lineage::{Lineage, OpSignature};
use crate::operator::EulerOperator;
use crate::state::MutableDraft;

/// Split a face by connecting two vertices with a new edge.
///
/// Given `vertex_a` and `vertex_b` on the boundary of `face`, this operator:
/// 1. Creates a pair of twin halfedges connecting the two vertices
/// 2. Creates a new face on one side of the new edge
/// 3. Creates a new loop for the new face
/// 4. Rewires the existing loops so both faces have closed boundaries
///
/// # Preconditions
/// - Both vertices must lie on the boundary halfedges of `face`
/// - The face must have at least one edge (not the degenerate MVF seed)
#[derive(Debug)]
pub struct MakeEdgeFace {
    /// First vertex (origin of the new halfedge going to vertex_b).
    pub vertex_a: VertexId,
    /// Second vertex (origin of the twin halfedge going back to vertex_a).
    pub vertex_b: VertexId,
    /// The face to split.
    pub face: FaceId,
}

/// Output of the MakeEdgeFace operator.
pub struct MefOutput {
    /// The new halfedge from vertex_a toward vertex_b.
    pub half_edge_ab: HalfEdgeId,
    /// The twin halfedge from vertex_b toward vertex_a.
    pub half_edge_ba: HalfEdgeId,
    /// The newly created face (on the vertex_b side).
    pub new_face: FaceId,
    /// The loop of the new face.
    pub new_loop: LoopId,
}

impl EulerOperator for MakeEdgeFace {
    type Output = MefOutput;

    fn execute(&self, draft: &mut MutableDraft, sig: &OpSignature) -> Result<Self::Output, KernelError> {
        let he_from_a = find_outgoing_in_face(draft, self.vertex_a, self.face)?;
        let he_from_b = find_outgoing_in_face(draft, self.vertex_b, self.face)?;

        let prev_a = draft.arena().get_half_edge(he_from_a)?.prev;
        let prev_b = draft.arena().get_half_edge(he_from_b)?.prev;

        let parent_lineage = draft.arena().get_face(self.face)?.lineage.clone();
        let child_lineage = derive_or_root(&parent_lineage, sig);

        let placeholder_he = HalfEdgeId::new(u32::MAX, 0);
        let placeholder_loop = LoopId::new(u32::MAX, 0);

        let arena = draft.arena_mut();

        let new_face = arena.insert_face(FaceData {
            outer_loop: placeholder_loop,
            lineage: Some(child_lineage.clone()),
        });

        let new_loop = arena.insert_loop(LoopData {
            half_edge: placeholder_he,
            face: new_face,
        });

        let he_ab = arena.insert_half_edge(HalfEdgeData {
            twin: placeholder_he,
            next: he_from_b,
            prev: prev_a,
            face: self.face,
            origin: self.vertex_a,
            lineage: Some(child_lineage.clone()),
        });

        let he_ba = arena.insert_half_edge(HalfEdgeData {
            twin: he_ab,
            next: he_from_a,
            prev: prev_b,
            face: new_face,
            origin: self.vertex_b,
            lineage: Some(child_lineage),
        });

        arena.get_half_edge_mut(he_ab)?.twin = he_ba;

        arena.get_half_edge_mut(prev_a)?.next = he_ab;
        arena.get_half_edge_mut(prev_b)?.next = he_ba;
        arena.get_half_edge_mut(he_from_a)?.prev = he_ba;
        arena.get_half_edge_mut(he_from_b)?.prev = he_ab;

        reassign_loop_faces(arena, he_ba, new_face)?;

        arena.get_face_mut(new_face)?.outer_loop = new_loop;
        arena.get_loop_mut(new_loop)?.half_edge = he_ba;

        let old_loop = arena.get_face(self.face)?.outer_loop;
        arena.get_loop_mut(old_loop)?.half_edge = he_ab;

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

/// Derive a child lineage from a parent, or create root if parent has none.
fn derive_or_root(parent: &Option<Lineage>, sig: &OpSignature) -> Lineage {
    match parent {
        Some(p) => Lineage::derive(p, sig.clone()),
        None => Lineage::root(0, sig.clone()),
    }
}

/// Find the halfedge originating from `vertex` that lies in `face`.
fn find_outgoing_in_face(
    draft: &MutableDraft,
    vertex: VertexId,
    face: FaceId,
) -> Result<HalfEdgeId, KernelError> {
    let start = draft.arena().get_vertex(vertex)?.outgoing;
    let mut current = start;
    let max_iterations: usize = 1000;

    for _ in 0..max_iterations {
        let he_data = draft.arena().get_half_edge(current)?;
        if he_data.face == face {
            return Ok(current);
        }
        let twin = he_data.twin;
        let next_of_twin = draft.arena().get_half_edge(twin)?.next;
        current = next_of_twin;
        if current == start {
            return Err(KernelError::InvalidInput {
                message: format!("Vertex {} is not on the boundary of Face {}", vertex, face),
                context: None,
            });
        }
    }

    Err(KernelError::InternalError {
        message: "Loop limit exceeded in find_outgoing_in_face".to_string(),
        context: None,
    })
}

/// Walk the halfedge loop starting from `start_he` and set all halfedges' face to `face`.
fn reassign_loop_faces(
    arena: &mut crate::arena::TopologyArena,
    start_he: HalfEdgeId,
    face: FaceId,
) -> Result<(), KernelError> {
    let mut current = start_he;
    let max_iterations: usize = 10000;

    for _ in 0..max_iterations {
        arena.get_half_edge_mut(current)?.face = face;
        let next = arena.get_half_edge(current)?.next;
        current = next;
        if current == start_he {
            return Ok(());
        }
    }

    Err(KernelError::InternalError {
        message: "Loop limit exceeded in reassign_loop_faces".to_string(),
        context: None,
    })
}
