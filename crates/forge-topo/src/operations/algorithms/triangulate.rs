//! Triangulate — fan-triangulate a face with N > 3 edges.
//!
//! DOMAIN: Given a face with N vertices on its outer loop (N > 3),
//! repeatedly apply MakeEdgeFace to produce N-2 triangular faces.
//! Uses a simple fan triangulation from the first vertex.
//!
//! This is a compound algorithm: repeated MakeEdgeFace.
//!
//! NOTE: This is purely topological — no geometric ear-clipping.
//! For non-convex faces, a geometry-aware triangulator in forge-kernel
//! should be used instead.
//! Curved faces are rejected here because topological fan diagonals may not
//! be valid trim curves in parametric space.
//!
//! DEPENDENCIES: `entity_lifecycle::make_edge_face`

use crate::handles::{FaceId, HalfEdgeId};
use crate::transactions::MutableDraft;
use crate::operations::entity_lifecycle::make_edge_face::MakeEdgeFace;
use crate::queries::traverse::FaceEdgeIterator;
use forge_core::KernelError;

/// Output of the triangulate_face algorithm.
pub struct TriangulateOutput {
    /// The new edges created by the triangulation (N-3 edges for an N-gon).
    pub new_edges: Vec<HalfEdgeId>,
    /// The new faces created (N-3 faces; the original face remains as one triangle).
    pub new_faces: Vec<FaceId>,
}

/// Fan-triangulate a face into triangles from its first vertex.
///
/// The face must have no inner loops (bridge them first).
/// For a face with N edges, this creates N-3 new edges and N-2 total
/// triangles (the original face becomes one of them).
pub fn triangulate_face(
    draft: &mut MutableDraft,
    face: FaceId,
) -> Result<TriangulateOutput, KernelError> {
    if draft.arena().get_face(face)?.surface_ref().is_some() {
        return Err(KernelError::InvalidInput {
            message: format!(
                "triangulate_face: face {} is curved; use kernel geometry validation/triangulation",
                face.index()
            ),
            context: None,
        });
    }

    if !draft.arena().get_face(face)?.inner_loops().is_empty() {
        return Err(KernelError::InvalidInput {
            message: format!(
                "triangulate_face: face {} has inner loops; bridge them first",
                face.index()
            ),
            context: None,
        });
    }

    let verts = collect_face_vertices(draft, face)?;
    let n = verts.len();

    if n < 3 {
        return Err(KernelError::InvalidInput {
            message: format!(
                "triangulate_face: face {} has only {} vertices",
                face.index(),
                n
            ),
            context: None,
        });
    }

    if n == 3 {
        return Ok(TriangulateOutput {
            new_edges: Vec::new(),
            new_faces: Vec::new(),
        });
    }

    let anchor = verts[0];
    let mut new_edges = Vec::new();
    let mut new_faces = Vec::new();
    let mut current_face = face;

    for i in 2..(n - 1) {
        let target = verts[i];

        let mef = draft.execute(
            MakeEdgeFace {
                face: current_face,
                vertex_a: anchor,
                vertex_b: target,
            },
        )?
        .into_value();

        new_edges.push(mef.half_edge_ab);
        new_faces.push(mef.new_face);
        current_face = face;
    }

    Ok(TriangulateOutput {
        new_edges,
        new_faces,
    })
}

/// Collect all vertex IDs around the outer loop of a face, in order.
fn collect_face_vertices(
    draft: &MutableDraft,
    face: FaceId,
) -> Result<Vec<crate::handles::VertexId>, KernelError> {
    let mut verts = Vec::new();
    for he_result in FaceEdgeIterator::new(draft.arena(), face)? {
        let he_id = he_result?;
        let v = draft.arena().get_half_edge(he_id)?.origin();
        verts.push(v);
    }
    Ok(verts)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity_lifecycle::make_vertex_face::MakeVertexFace;
    use crate::entity_lifecycle::split_edge::SplitEdge;
    use crate::transactions::TopologyState;
    use crate::queries::traverse::FaceEdgeIterator;

    #[test]
    fn triangulate_quad_produces_two_triangles() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();

        let mvf = draft.execute(MakeVertexFace).unwrap().into_value();
        let se1 = draft.execute(
            SplitEdge {
                edge: mvf.half_edge,
            },
        )
        .unwrap()
        .into_value();
        let se2 = draft.execute(
            SplitEdge {
                edge: se1.he_mb,
            },
        )
        .unwrap()
        .into_value();
        let _se3 = draft.execute(
            SplitEdge {
                edge: se2.he_mb,
            },
        )
        .unwrap()
        .into_value();

        assert_eq!(draft.arena().face_count(), 1);
        assert_eq!(draft.arena().vertex_count(), 4);

        let result = triangulate_face(&mut draft, mvf.face).unwrap();

        assert_eq!(result.new_edges.len(), 1, "Quad needs 1 diagonal");
        assert_eq!(result.new_faces.len(), 1, "Quad splits into 2 faces");
        assert_eq!(draft.arena().face_count(), 2);

        for (face_id, _) in draft.arena().iter_faces() {
            let count: usize = FaceEdgeIterator::new(draft.arena(), face_id)
                .unwrap()
                .map(|r| r.unwrap())
                .count();
            assert_eq!(count, 3, "Each face must be a triangle after triangulation");
        }
    }

    #[test]
    fn triangulate_pentagon_produces_three_triangles() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();

        let mvf = draft.execute(MakeVertexFace).unwrap().into_value();
        let se1 = draft.execute(
            SplitEdge {
                edge: mvf.half_edge,
            },
        )
        .unwrap()
        .into_value();
        let se2 = draft.execute(
            SplitEdge {
                edge: se1.he_mb,
            },
        )
        .unwrap()
        .into_value();
        let se3 = draft.execute(
            SplitEdge {
                edge: se2.he_mb,
            },
        )
        .unwrap()
        .into_value();
        let _se4 = draft.execute(
            SplitEdge {
                edge: se3.he_mb,
            },
        )
        .unwrap()
        .into_value();

        assert_eq!(draft.arena().vertex_count(), 5);

        let result = triangulate_face(&mut draft, mvf.face).unwrap();

        assert_eq!(result.new_edges.len(), 2, "Pentagon needs 2 diagonals");
        assert_eq!(result.new_faces.len(), 2, "Pentagon splits into 3 faces");
        assert_eq!(draft.arena().face_count(), 3);
    }

    #[test]
    fn triangulate_triangle_is_noop() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();

        let mvf = draft.execute(MakeVertexFace).unwrap().into_value();
        let se1 = draft.execute(
            SplitEdge {
                edge: mvf.half_edge,
            },
        )
        .unwrap()
        .into_value();
        let _se2 = draft.execute(
            SplitEdge {
                edge: se1.he_mb,
            },
        )
        .unwrap()
        .into_value();

        assert_eq!(draft.arena().vertex_count(), 3);

        let result = triangulate_face(&mut draft, mvf.face).unwrap();

        assert!(result.new_edges.is_empty());
        assert!(result.new_faces.is_empty());
        assert_eq!(draft.arena().face_count(), 1);
    }
}
