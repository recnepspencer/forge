use crate::topology_operators::declaration_entry::TopologyCreateInnerLoopOnExistingFaceDeclaration;

use super::super::super::{
    TopologyDeclaredMutationArtifact, TopologyMutationApplicationError,
    TopologyMutationApplicationMode, TopologyMutationApplicationRunner, TopologyMutationFamily,
    TopologyQueryBindingIndex,
};
use super::super::mutation_payload::TopologyDeclarationMutationPayload;
use super::super::orchestration_boundary::orchestrate_topology_declaration_entry;

impl<'workspace, 'surfaces> TopologyMutationApplicationRunner<'workspace, 'surfaces> {
    pub(crate) fn apply_create_inner_loop_on_existing_face_declaration(
        &mut self,
        declaration: TopologyCreateInnerLoopOnExistingFaceDeclaration,
        bindings: &TopologyQueryBindingIndex,
        mode: TopologyMutationApplicationMode,
    ) -> Result<TopologyDeclaredMutationArtifact, TopologyMutationApplicationError> {
        let retained_handoff = orchestrate_topology_declaration_entry(
            TopologyMutationFamily::AttachBoundaryMembership,
            declaration.clone(),
            mode.clone(),
        )?;

        let sequence = declaration.clone().into_mutation_sequence();
        self.compose_face_inner_loop_program(
            retained_handoff,
            mode,
            TopologyCreateInnerLoopOnExistingFaceDeclaration::SEMANTIC_FAMILY_KEY,
            &sequence,
            bindings,
        )
    }
}
