use crate::data::error::SpecError;
use crate::data::identity::SpecNodeId;
use crate::data::schema::{RelationKind, SpecNodeKind};
use crate::logic::mutation::{MutationResult, SpecLineageRecorder, SpecMutation, TouchedDomain};
use crate::logic::transaction::SpecDraft;

/// Split a simple outer loop face by connecting two boundary vertices.
///
/// This implementation is intentionally restricted to the smallest currently
/// proven migration case:
/// - one outer loop
/// - no inner loops
/// - exactly one boundary halfedge from each input vertex on the face
pub struct MakeEdgeFaceMutation {
    pub face: SpecNodeId,
    pub vertex_a: SpecNodeId,
    pub vertex_b: SpecNodeId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MakeEdgeFaceOutput {
    pub half_edge_ab: SpecNodeId,
    pub half_edge_ba: SpecNodeId,
    pub new_face: SpecNodeId,
    pub new_loop: SpecNodeId,
    pub edge: SpecNodeId,
}

impl std::fmt::Debug for MakeEdgeFaceMutation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MakeEdgeFaceMutation")
            .field("face", &self.face)
            .field("vertex_a", &self.vertex_a)
            .field("vertex_b", &self.vertex_b)
            .finish()
    }
}

impl SpecMutation for MakeEdgeFaceMutation {
    type Output = MakeEdgeFaceOutput;

    const NAME: &'static str = "make_edge_face";

    fn execute(
        &self,
        draft: &mut SpecDraft,
        _recorder: &mut SpecLineageRecorder,
    ) -> Result<MutationResult<Self::Output>, SpecError> {
        if self.vertex_a == self.vertex_b {
            return Err(SpecError::invalid(
                "MakeEdgeFaceMutation does not support closed-edge splitting".to_string(),
            ));
        }

        if !draft
            .outgoing_targets_of_kind(self.face, RelationKind::FaceInnerLoop)
            .is_empty()
        {
            return Err(SpecError::invalid(
                "MakeEdgeFaceMutation currently supports only faces without inner loops"
                    .to_string(),
            ));
        }

        let shell = draft.single_incoming_source(self.face, RelationKind::ShellOwnsFace)?;
        let original_loop = draft.single_outgoing_target(self.face, RelationKind::FaceOuterLoop)?;
        let face_halfedges = draft.incoming_sources_of_kind(self.face, RelationKind::HalfEdgeBoundsFace);

        let candidates_a = face_halfedges_from_vertex(draft, &face_halfedges, self.vertex_a)?;
        let candidates_b = face_halfedges_from_vertex(draft, &face_halfedges, self.vertex_b)?;

        if candidates_a.len() != 1 || candidates_b.len() != 1 {
            return Err(SpecError::invalid(
                "MakeEdgeFaceMutation currently requires exactly one boundary occurrence of each vertex on the face"
                    .to_string(),
            ));
        }

        let he_from_a = candidates_a[0];
        let he_from_b = candidates_b[0];
        let next_a = draft.single_outgoing_target(he_from_a, RelationKind::HalfEdgeNext)?;
        let next_b = draft.single_outgoing_target(he_from_b, RelationKind::HalfEdgeNext)?;
        if !(next_a == he_from_b && next_b == he_from_a) {
            return Err(SpecError::invalid(
                "MakeEdgeFaceMutation currently supports only the simple two-halfedge split case"
                    .to_string(),
            ));
        }

        let prev_a = draft.single_incoming_source(he_from_a, RelationKind::HalfEdgeNext)?;
        let prev_b = draft.single_incoming_source(he_from_b, RelationKind::HalfEdgeNext)?;

        let new_face = draft.create_node(SpecNodeKind::Face, None, "face")?;
        let new_loop = draft.create_node(SpecNodeKind::Loop, None, "loop")?;
        let edge = draft.create_node(SpecNodeKind::Edge, None, "edge")?;
        let half_edge_ab = draft.create_node(SpecNodeKind::HalfEdge, None, "half_edge_ab")?;
        let half_edge_ba = draft.create_node(SpecNodeKind::HalfEdge, None, "half_edge_ba")?;

        draft.replace_single_relation(RelationKind::HalfEdgeNext, prev_a, half_edge_ab, "prev-a-next")?;
        add(draft, RelationKind::HalfEdgeNext, half_edge_ab, he_from_b, 0, "ab-next")?;
        draft.replace_single_relation(RelationKind::HalfEdgeNext, prev_b, half_edge_ba, "prev-b-next")?;
        add(draft, RelationKind::HalfEdgeNext, half_edge_ba, he_from_a, 0, "ba-next")?;

        add(
            draft,
            RelationKind::HalfEdgeRadialNext,
            half_edge_ab,
            half_edge_ba,
            0,
            "ab-radial",
        )?;
        add(
            draft,
            RelationKind::HalfEdgeRadialNext,
            half_edge_ba,
            half_edge_ab,
            0,
            "ba-radial",
        )?;
        add(draft, RelationKind::HalfEdgeUsesEdge, half_edge_ab, edge, 0, "ab-edge")?;
        add(draft, RelationKind::HalfEdgeUsesEdge, half_edge_ba, edge, 0, "ba-edge")?;
        add(
            draft,
            RelationKind::HalfEdgeOriginVertex,
            half_edge_ab,
            self.vertex_a,
            0,
            "ab-origin",
        )?;
        add(
            draft,
            RelationKind::HalfEdgeOriginVertex,
            half_edge_ba,
            self.vertex_b,
            0,
            "ba-origin",
        )?;
        add(
            draft,
            RelationKind::HalfEdgeBoundsFace,
            half_edge_ab,
            self.face,
            0,
            "ab-face",
        )?;
        add(
            draft,
            RelationKind::HalfEdgeBoundsFace,
            half_edge_ba,
            new_face,
            0,
            "ba-face",
        )?;

        draft.replace_single_relation(
            RelationKind::HalfEdgeBoundsFace,
            he_from_a,
            new_face,
            "reassign-face",
        )?;
        draft.replace_single_relation(
            RelationKind::LoopEntryHalfEdge,
            original_loop,
            half_edge_ab,
            "old-loop-entry",
        )?;
        add(
            draft,
            RelationKind::ShellOwnsFace,
            shell,
            new_face,
            0,
            "shell-face",
        )?;
        add(
            draft,
            RelationKind::FaceOuterLoop,
            new_face,
            new_loop,
            0,
            "new-face-loop",
        )?;
        add(
            draft,
            RelationKind::LoopEntryHalfEdge,
            new_loop,
            half_edge_ba,
            0,
            "new-loop-entry",
        )?;

        Ok(MutationResult {
            value: MakeEdgeFaceOutput {
                half_edge_ab,
                half_edge_ba,
                new_face,
                new_loop,
                edge,
            },
            touched_domains: vec![TouchedDomain::Topology],
            mutation_trace: vec![
                format!(
                    "split face {} between vertices {} and {}",
                    self.face, self.vertex_a, self.vertex_b
                ),
                "create new face, loop, and split edge pair".to_string(),
            ],
        })
    }

    fn semantic_summary(&self) -> String {
        format!(
            "Split face {} between vertices {} and {}",
            self.face, self.vertex_a, self.vertex_b
        )
    }
}

fn face_halfedges_from_vertex(
    draft: &SpecDraft,
    halfedges: &[SpecNodeId],
    vertex: SpecNodeId,
) -> Result<Vec<SpecNodeId>, SpecError> {
    let mut result = Vec::new();
    for halfedge in halfedges {
        let origin = draft.single_outgoing_target(*halfedge, RelationKind::HalfEdgeOriginVertex)?;
        if origin == vertex {
            result.push(*halfedge);
        }
    }
    result.sort_unstable();
    Ok(result)
}

fn add(
    draft: &mut SpecDraft,
    kind: RelationKind,
    source: SpecNodeId,
    target: SpecNodeId,
    ordinal: u32,
    role: &str,
) -> Result<(), SpecError> {
    draft.add_relation(kind, source, target, ordinal, role)?;
    Ok(())
}
