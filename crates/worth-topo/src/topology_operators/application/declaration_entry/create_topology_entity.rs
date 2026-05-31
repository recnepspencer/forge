use crate::topology_operators::declaration_entry::TopologyCreateTopologyEntityDeclaration;

use super::super::{
    TopologyDeclaredMutationArtifact, TopologyEditApplicationMode, TopologyOperatorExecutionError,
    TopologyOperatorRunner,
};
use super::contract_payload::TopologyDeclarationContractPayload;
use super::execution_finalize::{finalize_lowered_batch, lower_contracts};
use super::orchestration_boundary::orchestrate_topology_declaration_entry;

impl<'workspace, 'surfaces> TopologyOperatorRunner<'workspace, 'surfaces> {
    pub(crate) fn apply_create_topology_entity_declaration(
        &mut self,
        declaration: TopologyCreateTopologyEntityDeclaration,
        mode: TopologyEditApplicationMode,
    ) -> Result<TopologyDeclaredMutationArtifact, TopologyOperatorExecutionError> {
        orchestrate_topology_declaration_entry(
            crate::topology_operators::TopologyEditFamily::CreateTopologyEntity,
            declaration.clone(),
        )?;

        let contracts = declaration.clone().into_contracts();
        let lowered_batch = lower_contracts(
            self,
            &contracts,
            &Default::default(),
            &declaration.created_entity_kinds(),
        )?;

        finalize_lowered_batch(
            self,
            lowered_batch,
            TopologyCreateTopologyEntityDeclaration::SEMANTIC_FAMILY_KEY,
            mode,
            declaration.semantic_families(),
            declaration.topology_edit_digest(),
            declaration.naming_continuity_matrix(),
            declaration.naming_report(),
        )
    }
}
