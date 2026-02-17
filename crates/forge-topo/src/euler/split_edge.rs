//! SE — Split Edge.
//!
//! DOMAIN: Subdivides a halfedge pair by inserting a new vertex at the midpoint,
//! producing two new halfedge pairs and one new vertex.
//!
//! The topology around the split point is rewired so that both original faces
//! retain valid, closed loops.
//!
//! Also handles the degenerate case where `he.twin == he` (the MVF seed),
//! turning the self-loop into a proper 2-halfedge edge between two vertices.

use forge_core::KernelError;
use crate::arena::{HalfEdgeData, VertexData};
use crate::handles::{HalfEdgeId, VertexId};
use crate::lineage::{Lineage, OpSignature};
use crate::operator::EulerOperator;
use crate::state::MutableDraft;

/// Subdivide a halfedge pair, inserting a new vertex.
///
/// Given halfedge `he` (from vertex A to vertex B), this operator:
/// 1. Inserts a new vertex M
/// 2. Replaces `he` with two consecutive halfedges: A→M and M→B
/// 3. Replaces `he.twin` with two consecutive halfedges: B→M and M→A
///
/// For the degenerate case (MVF seed where he.twin == he), this creates
/// a proper edge: two halfedges A→M and M→A, each being twins.
///
/// # Returns
/// `SplitEdgeOutput` — the new vertex and the two halfedges
/// that now connect A→M→B on the original face side.
#[derive(Debug)]
pub struct SplitEdge {
    /// The halfedge to split (from origin A toward the next vertex B).
    pub edge: HalfEdgeId,
}

/// Output of the SplitEdge operator.
pub struct SplitEdgeOutput {
    /// The newly inserted midpoint vertex.
    pub new_vertex: VertexId,
    /// Halfedge from A to M (reuses the original halfedge slot conceptually).
    pub he_am: HalfEdgeId,
    /// New halfedge from M to A (the twin in the 2-vertex case).
    pub he_mb: HalfEdgeId,
}

impl EulerOperator for SplitEdge {
    type Output = SplitEdgeOutput;

    fn execute(&self, draft: &mut MutableDraft, sig: &OpSignature) -> Result<Self::Output, KernelError> {
        let he = self.edge;
        let he_data = draft.arena().get_half_edge(he)?.clone();
        let twin = he_data.twin;

        if he == twin {
            return execute_degenerate_split(draft, he, &he_data, sig);
        }

        execute_normal_split(draft, he, &he_data, sig)
    }

    fn signature(&self) -> OpSignature {
        OpSignature::new("split_edge")
    }
}

/// Derive a child lineage from a parent, or create root if parent has none.
fn derive_or_root(parent: &Option<Lineage>, sig: &OpSignature) -> Lineage {
    match parent {
        Some(p) => Lineage::derive(p, sig.clone()),
        None => Lineage::root(0, sig.clone()),
    }
}

/// Split a degenerate self-twin halfedge (from MVF seed).
///
/// Transforms the single self-referencing halfedge into a proper edge:
/// two halfedges forming a 2-edge loop around a 2-vertex face.
fn execute_degenerate_split(
    draft: &mut MutableDraft,
    he: HalfEdgeId,
    he_data: &HalfEdgeData,
    sig: &OpSignature,
) -> Result<SplitEdgeOutput, KernelError> {
    let face = he_data.face;
    let parent_lineage = he_data.lineage.clone();
    let child_lineage = derive_or_root(&parent_lineage, sig);

    let arena = draft.arena_mut();

    let new_vertex = arena.insert_vertex(VertexData {
        outgoing: HalfEdgeId::new(u32::MAX, 0),
        lineage: Some(child_lineage.clone()),
    });

    let he_ma = arena.insert_half_edge(HalfEdgeData {
        twin: he,
        next: he,
        prev: he,
        face,
        origin: new_vertex,
        lineage: Some(child_lineage.clone()),
    });

    arena.get_half_edge_mut(he)?.twin = he_ma;
    arena.get_half_edge_mut(he)?.next = he_ma;
    arena.get_half_edge_mut(he)?.prev = he_ma;
    arena.get_half_edge_mut(he)?.lineage = Some(child_lineage);

    arena.get_vertex_mut(new_vertex)?.outgoing = he_ma;

    let face_data = arena.get_face(face)?;
    let loop_id = face_data.outer_loop;
    arena.get_loop_mut(loop_id)?.half_edge = he;

    Ok(SplitEdgeOutput {
        new_vertex,
        he_am: he,
        he_mb: he_ma,
    })
}

/// Split a normal edge (he != twin) by inserting a new vertex.
fn execute_normal_split(
    draft: &mut MutableDraft,
    he: HalfEdgeId,
    he_data: &HalfEdgeData,
    sig: &OpSignature,
) -> Result<SplitEdgeOutput, KernelError> {
    let twin = he_data.twin;
    let twin_data = draft.arena().get_half_edge(twin)?.clone();

    let face_left = he_data.face;
    let face_right = twin_data.face;
    let he_next = he_data.next;
    let twin_next = twin_data.next;

    let parent_lineage = he_data.lineage.clone();
    let child_lineage = derive_or_root(&parent_lineage, sig);

    let arena = draft.arena_mut();

    let new_vertex = arena.insert_vertex(VertexData {
        outgoing: HalfEdgeId::new(u32::MAX, 0),
        lineage: Some(child_lineage.clone()),
    });

    let he_mb = arena.insert_half_edge(HalfEdgeData {
        twin: HalfEdgeId::new(u32::MAX, 0),
        next: he_next,
        prev: he,
        face: face_left,
        origin: new_vertex,
        lineage: Some(child_lineage.clone()),
    });

    let he_bm = arena.insert_half_edge(HalfEdgeData {
        twin: he,
        next: twin_next,
        prev: twin,
        face: face_right,
        origin: new_vertex,
        lineage: Some(child_lineage.clone()),
    });

    arena.get_half_edge_mut(he)?.next = he_mb;
    arena.get_half_edge_mut(he)?.twin = he_bm;
    arena.get_half_edge_mut(he)?.lineage = Some(child_lineage);

    arena.get_half_edge_mut(twin)?.next = he_bm;
    arena.get_half_edge_mut(twin)?.twin = he_mb;

    arena.get_half_edge_mut(he_mb)?.twin = twin;

    arena.get_half_edge_mut(he_next)?.prev = he_mb;
    arena.get_half_edge_mut(twin_next)?.prev = he_bm;

    arena.get_vertex_mut(new_vertex)?.outgoing = he_mb;

    Ok(SplitEdgeOutput {
        new_vertex,
        he_am: he,
        he_mb,
    })
}
