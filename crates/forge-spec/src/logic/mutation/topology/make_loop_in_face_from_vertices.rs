use crate::data::error::SpecError;
use crate::data::identity::SpecNodeId;
use crate::data::schema::{RelationKind, SpecNodeKind};
use crate::logic::mutation::{MutationResult, SpecLineageRecorder, SpecMutation, TouchedDomain};
use crate::logic::transaction::SpecDraft;

use super::wire_loop_cycle::create_loop_cycle;

#[derive(Debug, Clone)]
pub struct MakeLoopInFaceFromVerticesMutation {
    pub face: SpecNodeId,
    pub vertices: Vec<SpecNodeId>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MakeLoopInFaceFromVerticesOutput {
    pub loop_id: SpecNodeId,
    pub half_edges: Vec<SpecNodeId>,
    pub edges: Vec<SpecNodeId>,
}

impl SpecMutation for MakeLoopInFaceFromVerticesMutation {
    type Output = MakeLoopInFaceFromVerticesOutput;

    const NAME: &'static str = "make_loop_in_face_from_vertices";

    fn execute(
        &self,
        draft: &mut SpecDraft,
        _recorder: &mut SpecLineageRecorder,
    ) -> Result<MutationResult<Self::Output>, SpecError> {
        if draft.node_kind(self.face)? != SpecNodeKind::Face {
            return Err(SpecError::invalid(format!(
                "MakeLoopInFaceFromVerticesMutation requires Face input, got {:?}",
                draft.node_kind(self.face)?
            )));
        }

        let ordinal = draft
            .outgoing_targets_of_kind(self.face, RelationKind::FaceInnerLoop)
            .len() as u32;
        let wired = create_loop_cycle(
            draft,
            self.face,
            &self.vertices,
            RelationKind::FaceInnerLoop,
            ordinal,
            "loop-in-face",
        )?;

        Ok(MutationResult {
            value: MakeLoopInFaceFromVerticesOutput {
                loop_id: wired.loop_id,
                half_edges: wired.half_edges,
                edges: wired.edges,
            },
            touched_domains: vec![TouchedDomain::Topology],
            mutation_trace: vec![
                format!(
                    "create {}-vertex inner loop in face {}",
                    self.vertices.len(),
                    self.face
                ),
                "wire inner loop halfedge cycle from existing vertices".to_string(),
            ],
        })
    }

    fn semantic_summary(&self) -> String {
        format!(
            "Create {}-vertex inner loop in face {}",
            self.vertices.len(),
            self.face
        )
    }
}
