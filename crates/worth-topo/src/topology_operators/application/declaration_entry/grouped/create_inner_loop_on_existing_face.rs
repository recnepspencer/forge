use crate::topology_operators::declaration_entry::TopologyCreateInnerLoopOnExistingFaceDeclaration;

use super::super::super::{
    TopologyDeclaredMutationArtifact, TopologyEditApplicationMode, TopologyEditFamily,
    TopologyOperatorExecutionError, TopologyOperatorRunner, TopologyQueryBindingIndex,
};
use super::super::contract_payload::TopologyDeclarationContractPayload;
use super::super::orchestration_boundary::orchestrate_topology_declaration_entry;

impl<'workspace, 'surfaces> TopologyOperatorRunner<'workspace, 'surfaces> {
    pub(crate) fn apply_create_inner_loop_on_existing_face_declaration(
        &mut self,
        declaration: TopologyCreateInnerLoopOnExistingFaceDeclaration,
        bindings: &TopologyQueryBindingIndex,
        mode: TopologyEditApplicationMode,
    ) -> Result<TopologyDeclaredMutationArtifact, TopologyOperatorExecutionError> {
        orchestrate_topology_declaration_entry(
            TopologyEditFamily::AttachBoundaryMembership,
            declaration.clone(),
        )?;

        let contracts = declaration.clone().into_contracts();
        let receipt = self.compose_face_inner_loop_program(&contracts, bindings)?;
        self.finish_composed_membership_execution(
            mode,
            TopologyCreateInnerLoopOnExistingFaceDeclaration::SEMANTIC_FAMILY_KEY,
            declaration.semantic_families(),
            declaration.topology_edit_digest(),
            declaration.naming_continuity_matrix(),
            declaration.naming_report(),
            receipt,
        )
    }
}
