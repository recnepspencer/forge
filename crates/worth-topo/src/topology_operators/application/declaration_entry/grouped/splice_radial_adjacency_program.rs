use crate::topology_operators::declaration_entry::TopologySpliceRadialAdjacencyProgramDeclaration;

use super::super::super::{
    TopologyEditApplicationMode, TopologyEditFamily, TopologyOperatorExecution,
    TopologyOperatorExecutionError, TopologyOperatorExecutionPath, TopologyOperatorRunner,
    TopologyQueryBindingIndex,
};
use super::super::contract_payload::TopologyDeclarationContractPayload;
use super::super::execution_finalize::{finalize_lowered_batch, lower_contracts};
use super::super::orchestration_boundary::orchestrate_topology_declaration_entry;

impl<'workspace, 'surfaces> TopologyOperatorRunner<'workspace, 'surfaces> {
    pub(crate) fn apply_splice_radial_adjacency_program_declaration(
        &mut self,
        declaration: TopologySpliceRadialAdjacencyProgramDeclaration,
        bindings: &TopologyQueryBindingIndex,
        mode: TopologyEditApplicationMode,
    ) -> Result<TopologyOperatorExecution, TopologyOperatorExecutionError> {
        orchestrate_topology_declaration_entry(
            TopologyEditFamily::SpliceRadialAdjacency,
            declaration.clone(),
        )?;

        let contracts = declaration.clone().into_contracts();
        let lowered_batch = lower_contracts(self, &contracts, bindings, &Default::default())?;
        finalize_lowered_batch(
            self,
            lowered_batch,
            TopologyOperatorExecutionPath::DeclarationEntry {
                semantic_family_key:
                    TopologySpliceRadialAdjacencyProgramDeclaration::SEMANTIC_FAMILY_KEY,
            },
            mode,
            declaration.semantic_families(),
            declaration.topology_edit_digest(),
            declaration.naming_continuity_matrix(),
            declaration.naming_report(),
        )
    }
}
