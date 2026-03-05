//! ExtractShell — isolate a subset of faces by unsewing boundary edges.
//!
//! DOMAIN: Given a set of face IDs, find all edges on the boundary of
//! the set (edges whose radial twin belongs to a face NOT in the set)
//! and unsew them via `UnsewEdge`.
//!
//! After extraction, the face subset is topologically disconnected from
//! the rest of the mesh.
//!
//! This is a compound algorithm: walk + repeated UnsewEdge.
//!
//! DEPENDENCIES: `non_manifold::unsew_edge`

use std::collections::BTreeSet;

use crate::handles::{FaceId, HalfEdgeId};
use crate::transactions::MutableDraft;
use crate::b_rep::EntityBitset;
use crate::operations::algorithms::region_extraction::is_face_group_boundary_half_edge;
use crate::operations::non_manifold::unsew_edge::UnsewEdge;
use crate::queries::traverse::FaceAllEdgesIterator;
use forge_core::KernelError;

/// Output of the extract_shell algorithm.
pub struct ExtractShellOutput {
    /// The boundary half-edge pairs that were unsewn.
    pub unsewn_pairs: Vec<(HalfEdgeId, HalfEdgeId)>,
}

/// Isolate a subset of faces by unsewing all boundary edges.
///
/// Walks every face in `faces`, finds halfedges whose radial twin
/// belongs to a face NOT in the set, and unsews them. After this call,
/// the face subset is topologically disconnected.
pub fn extract_shell(
    draft: &mut MutableDraft,
    faces: &EntityBitset,
) -> Result<ExtractShellOutput, KernelError> {
    if faces.is_empty() {
        return Ok(ExtractShellOutput {
            unsewn_pairs: Vec::new(),
        });
    }

    let boundary_pairs = find_boundary_pairs(draft, faces)?;

    let mut unsewn_pairs = Vec::new();
    for (he_inside, he_outside) in boundary_pairs {
        draft.execute(
            UnsewEdge {
                he_a: he_inside,
                he_b: he_outside,
            },
        )?
        .into_value();
        unsewn_pairs.push((he_inside, he_outside));
    }

    Ok(ExtractShellOutput { unsewn_pairs })
}

/// Find all (inside_he, outside_he) pairs on the boundary of the face set.
///
/// A boundary edge is one where `he` belongs to a face in `faces` but
/// `he.radial_next` belongs to a face NOT in `faces`.
fn find_boundary_pairs(
    draft: &MutableDraft,
    faces: &EntityBitset,
) -> Result<Vec<(HalfEdgeId, HalfEdgeId)>, KernelError> {
    let mut pairs = Vec::new();
    let mut seen_edges = BTreeSet::new();

    for face_idx in faces.iter_ones() {
        let face_id = FaceId::new(face_idx, 0);
        for he_result in FaceAllEdgesIterator::new(draft.arena(), face_id)? {
            let he_id = he_result?;
            let he_data = draft.arena().get_half_edge(he_id)?;
            let twin_id = he_data.radial_next();

            if twin_id == he_id {
                continue;
            }

            if !is_face_group_boundary_half_edge(draft.arena(), faces, he_id)? {
                continue;
            }

            let canonical = (
                he_id.index().min(twin_id.index()),
                he_id.index().max(twin_id.index()),
            );
            if seen_edges.contains(&canonical) {
                continue;
            }

            pairs.push((he_id, twin_id));
            seen_edges.insert(canonical);
        }
    }

    Ok(pairs)
}

#[cfg(test)]
mod tests {
    use crate::b_rep::ShellKind;
    use super::*;
    use crate::entity_lifecycle::make_edge_face::MakeEdgeFace;
    use crate::entity_lifecycle::make_vertex_face::MakeVertexFace;
    use crate::entity_lifecycle::split_edge::SplitEdge;
    use crate::transactions::TopologyState;
    use crate::queries::traverse::FaceEdgeIterator;

    #[test]
    fn extract_shell_isolates_one_triangle() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();

        let mvf = draft.execute(MakeVertexFace { shell_kind: ShellKind::Sheet }).unwrap().into_value();
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

        let edges: Vec<_> = FaceEdgeIterator::new(draft.arena(), mvf.face)
            .unwrap()
            .map(|r| r.unwrap())
            .collect();
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

        let mut subset = EntityBitset::for_faces(draft.arena());
        subset
            .insert(mef.new_face.index())
            .expect("bitset capacity must cover fixture faces");

        let result = extract_shell(&mut draft, &subset).unwrap();

        assert!(
            !result.unsewn_pairs.is_empty(),
            "Must have unsewn at least one edge"
        );

        for &(he_in, _) in &result.unsewn_pairs {
            let he_data = draft.arena().get_half_edge(he_in).unwrap();
            assert_eq!(
                he_data.radial_next(),
                he_in,
                "Unsewn halfedge must be self-radial"
            );
        }
    }

    #[test]
    fn extract_empty_set_is_noop() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();

        let _mvf = draft.execute(MakeVertexFace { shell_kind: ShellKind::Sheet }).unwrap().into_value();

        let subset = EntityBitset::for_faces(draft.arena());
        let result = extract_shell(&mut draft, &subset).unwrap();
        assert!(result.unsewn_pairs.is_empty());
    }
}
