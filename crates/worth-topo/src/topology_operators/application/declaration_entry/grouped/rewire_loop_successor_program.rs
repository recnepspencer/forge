use crate::topology_operators::declaration_entry::TopologyRewireLoopSuccessorProgramDeclaration;

use super::super::super::{
    TopologyEditApplicationMode, TopologyEditFamily, TopologyOperatorExecution,
    TopologyOperatorExecutionError, TopologyOperatorExecutionPath, TopologyOperatorRunner,
    TopologyQueryBindingIndex,
};
use super::super::contract_payload::TopologyDeclarationContractPayload;
use super::super::orchestration_boundary::orchestrate_topology_declaration_entry;

impl<'workspace, 'surfaces> TopologyOperatorRunner<'workspace, 'surfaces> {
    pub(crate) fn apply_rewire_loop_successor_program_declaration(
        &mut self,
        declaration: TopologyRewireLoopSuccessorProgramDeclaration,
        bindings: &TopologyQueryBindingIndex,
        mode: TopologyEditApplicationMode,
    ) -> Result<TopologyOperatorExecution, TopologyOperatorExecutionError> {
        orchestrate_topology_declaration_entry(
            TopologyEditFamily::RewireLoopSuccessor,
            declaration.clone(),
        )?;

        self.execute_composed_loop_successor_program(
            TopologyOperatorExecutionPath::DeclarationEntry {
                semantic_family_key:
                    TopologyRewireLoopSuccessorProgramDeclaration::SEMANTIC_FAMILY_KEY,
            },
            mode,
            declaration.semantic_families(),
            declaration.topology_edit_digest(),
            declaration.naming_continuity_matrix(),
            declaration.naming_report(),
            declaration.into_contracts(),
            bindings,
        )
    }
}
