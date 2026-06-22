use crate::topology_operators::declaration_entry::TopologyRewireLoopSuccessorProgramDeclaration;

use super::super::super::{
    TopologyDeclaredMutationArtifact, TopologyMutationApplicationError,
    TopologyMutationApplicationMode, TopologyMutationApplicationRunner, TopologyMutationFamily,
    TopologyQueryBindingIndex,
};
use super::super::mutation_payload::TopologyDeclarationMutationPayload;
use super::super::orchestration_boundary::orchestrate_topology_declaration_entry;

impl<'workspace, 'surfaces> TopologyMutationApplicationRunner<'workspace, 'surfaces> {
    pub(crate) fn apply_rewire_loop_successor_program_declaration(
        &mut self,
        declaration: TopologyRewireLoopSuccessorProgramDeclaration,
        bindings: &TopologyQueryBindingIndex,
        mode: TopologyMutationApplicationMode,
    ) -> Result<TopologyDeclaredMutationArtifact, TopologyMutationApplicationError> {
        let retained_handoff = orchestrate_topology_declaration_entry(
            TopologyMutationFamily::RewireLoopSuccessor,
            declaration.clone(),
            mode.clone(),
        )?;

        let sequence = declaration.into_mutation_sequence();
        self.execute_composed_loop_successor_program(
            TopologyRewireLoopSuccessorProgramDeclaration::SEMANTIC_FAMILY_KEY,
            retained_handoff,
            mode,
            sequence,
            bindings,
        )
    }
}
