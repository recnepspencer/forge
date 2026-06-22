use crate::topology_operators::declaration_entry::TopologyCreateTopologyEntityDeclaration;

use super::super::{
    TopologyDeclaredMutationArtifact, TopologyMutationApplicationError,
    TopologyMutationApplicationMode, TopologyMutationApplicationRunner,
};
use super::execution_finalize::{finalize_lowered_mutations, lower_mutation_sequence};
use super::mutation_payload::TopologyDeclarationMutationPayload;
use super::orchestration_boundary::orchestrate_topology_declaration_entry;

impl<'workspace, 'surfaces> TopologyMutationApplicationRunner<'workspace, 'surfaces> {
    pub(crate) fn apply_create_topology_entity_declaration(
        &mut self,
        declaration: TopologyCreateTopologyEntityDeclaration,
        mode: TopologyMutationApplicationMode,
    ) -> Result<TopologyDeclaredMutationArtifact, TopologyMutationApplicationError> {
        let retained_handoff = orchestrate_topology_declaration_entry(
            crate::topology_operators::TopologyMutationFamily::CreateTopologyEntity,
            declaration.clone(),
            mode.clone(),
        )?;

        let sequence = declaration.clone().into_mutation_sequence();
        let lowered_mutations = lower_mutation_sequence(
            self,
            &sequence,
            &Default::default(),
            sequence.created_entity_kinds(),
        )?;

        finalize_lowered_mutations(
            self,
            retained_handoff,
            lowered_mutations,
            TopologyCreateTopologyEntityDeclaration::SEMANTIC_FAMILY_KEY,
            mode,
            &sequence,
        )
    }
}
