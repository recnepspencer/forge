//! FlipEdge — flip the diagonal of a quad formed by two adjacent triangles.
//!
//! DOMAIN: Given an edge shared by two triangle faces, remove the edge
//! (JoinFaces) to form a quad, then re-split across the other diagonal
//! (MakeEdgeFace).
//!
//! This is a compound algorithm: JoinFaces + MakeEdgeFace.
//!
//! DEPENDENCIES: `boundary_editing::join_faces`, `entity_lifecycle::make_edge_face`

use crate::handles::{HalfEdgeId, VertexId};
use crate::state::MutableDraft;
use crate::topology::operations::boundary_editing::join_faces::JoinFaces;
use crate::topology::operations::entity_lifecycle::make_edge_face::MakeEdgeFace;
use crate::topology::queries::traverse::FaceEdgeIterator;
use forge_core::KernelError;

/// Output of the flip_edge algorithm.
pub struct FlipEdgeOutput {
    /// The new halfedge A→B on the flipped diagonal.
    pub he_ab: HalfEdgeId,
    /// The new halfedge B→A on the flipped diagonal.
    pub he_ba: HalfEdgeId,
    /// The vertex at one end of the new diagonal.
    pub vertex_a: VertexId,
    /// The vertex at the other end of the new diagonal.
    pub vertex_b: VertexId,
}

/// Flip the diagonal of two adjacent triangles sharing `edge`.
///
/// The two faces adjacent to `edge` must each have exactly 3 edges
/// (triangles). After flipping, the shared edge connects the two
/// vertices that were previously NOT connected.
pub fn flip_edge(
    draft: &mut MutableDraft,
    edge: HalfEdgeId,
) -> Result<FlipEdgeOutput, KernelError> {
    let he_data = draft.arena().get_half_edge(edge)?;
    let twin = he_data.radial_next();
    let face_a = he_data.face();

    let twin_data = draft.arena().get_half_edge(twin)?;
    let face_b = twin_data.face();

    if face_a == face_b {
        return Err(KernelError::InvalidInput {
            message: "flip_edge: edge is not shared between two distinct faces".into(),
            context: None,
        });
    }

    if twin == edge {
        return Err(KernelError::InvalidInput {
            message: "flip_edge: edge is a boundary edge (self-radial)".into(),
            context: None,
        });
    }

    let count_a = face_edge_count(draft, face_a)?;
    let count_b = face_edge_count(draft, face_b)?;

    if count_a != 3 || count_b != 3 {
        return Err(KernelError::InvalidInput {
            message: format!(
                "flip_edge: both faces must be triangles, got {} and {} edges",
                count_a, count_b
            ),
            context: None,
        });
    }

    let vertex_a = find_opposite_vertex(draft, face_a, edge)?;
    let vertex_b = find_opposite_vertex(draft, face_b, twin)?;

    let jf = draft.execute(JoinFaces { edge })?.into_value();
    let merged_face = jf.surviving_face;

    let mef = draft.execute(
        MakeEdgeFace {
            face: merged_face,
            vertex_a,
            vertex_b,
        },
    )?
    .into_value();

    Ok(FlipEdgeOutput {
        he_ab: mef.half_edge_ab,
        he_ba: mef.half_edge_ba,
        vertex_a,
        vertex_b,
    })
}

/// Count edges in a face loop.
fn face_edge_count(
    draft: &MutableDraft,
    face: crate::handles::FaceId,
) -> Result<usize, KernelError> {
    let mut count = 0usize;
    for he_result in FaceEdgeIterator::new(draft.arena(), face)? {
        let _he = he_result?;
        count += 1;
    }
    Ok(count)
}

/// Find the vertex in a triangle face that is opposite to the given halfedge.
fn find_opposite_vertex(
    draft: &MutableDraft,
    face: crate::handles::FaceId,
    he: HalfEdgeId,
) -> Result<VertexId, KernelError> {
    let he_origin = draft.arena().get_half_edge(he)?.origin();
    let he_next = draft.arena().get_half_edge(he)?.next();
    let he_next_origin = draft.arena().get_half_edge(he_next)?.origin();

    for he_result in FaceEdgeIterator::new(draft.arena(), face)? {
        let loop_he = he_result?;
        let v = draft.arena().get_half_edge(loop_he)?.origin();
        if v != he_origin && v != he_next_origin {
            return Ok(v);
        }
    }

    Err(KernelError::InvalidInput {
        message: format!(
            "flip_edge: could not find opposite vertex in face {}",
            face.index()
        ),
        context: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity_lifecycle::make_edge_face::MakeEdgeFace;
    use crate::entity_lifecycle::make_vertex_face::MakeVertexFace;
    use crate::entity_lifecycle::split_edge::SplitEdge;
    use crate::state::TopologyState;
    use crate::topology::queries::traverse::FaceEdgeIterator;

    #[test]
    fn flip_edge_swaps_diagonal() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();

        let mvf = draft.execute(MakeVertexFace).unwrap().into_value();
        let se1 = draft.execute(
            SplitEdge {
                edge: mvf.half_edge,
                parameter: 0.25,
            },
        )
        .unwrap()
        .into_value();
        let se2 = draft.execute(
            SplitEdge {
                edge: se1.he_mb,
                parameter: 0.5,
            },
        )
        .unwrap()
        .into_value();
        let _se3 = draft.execute(
            SplitEdge {
                edge: se2.he_mb,
                parameter: 0.75,
            },
        )
        .unwrap()
        .into_value();

        let edges: Vec<_> = FaceEdgeIterator::new(draft.arena(), mvf.face)
            .unwrap()
            .map(|r| r.unwrap())
            .collect();
        assert_eq!(edges.len(), 4);

        let v1 = draft.arena().get_half_edge(edges[1]).unwrap().origin();
        let v3 = draft.arena().get_half_edge(edges[3]).unwrap().origin();

        let mef = draft.execute(
            MakeEdgeFace {
                face: mvf.face,
                vertex_a: v1,
                vertex_b: v3,
            },
        )
        .unwrap()
        .into_value();

        assert_eq!(draft.arena().face_count(), 2);
        assert_eq!(draft.arena().vertex_count(), 4);

        let flip = flip_edge(&mut draft, mef.half_edge_ab).unwrap();

        assert_eq!(
            draft.arena().face_count(),
            2,
            "Flip must preserve face count"
        );
        assert_eq!(
            draft.arena().vertex_count(),
            4,
            "Flip must preserve vertex count"
        );

        assert_ne!(flip.vertex_a, v1);
        assert_ne!(flip.vertex_a, v3);
    }
}
