use crate::topology_operators::declaration_entry::TopologySpliceRadialAdjacencyProgramDeclaration;

use super::super::super::{
    TopologyDeclaredMutationArtifact, TopologyMutationApplicationError,
    TopologyMutationApplicationMode, TopologyMutationApplicationRunner, TopologyMutationFamily,
    TopologyQueryBindingIndex,
};
use super::super::execution_finalize::{finalize_lowered_mutations, lower_mutation_sequence};
use super::super::mutation_payload::TopologyDeclarationMutationPayload;
use super::super::orchestration_boundary::orchestrate_topology_declaration_entry;

impl<'workspace, 'surfaces> TopologyMutationApplicationRunner<'workspace, 'surfaces> {
    pub(crate) fn apply_splice_radial_adjacency_program_declaration(
        &mut self,
        declaration: TopologySpliceRadialAdjacencyProgramDeclaration,
        bindings: &TopologyQueryBindingIndex,
        mode: TopologyMutationApplicationMode,
    ) -> Result<TopologyDeclaredMutationArtifact, TopologyMutationApplicationError> {
        let retained_handoff = orchestrate_topology_declaration_entry(
            TopologyMutationFamily::SpliceRadialAdjacency,
            declaration.clone(),
            mode.clone(),
        )?;

        let sequence = declaration.clone().into_mutation_sequence();
        let lowered_mutations =
            lower_mutation_sequence(self, &sequence, bindings, &Default::default())?;
        finalize_lowered_mutations(
            self,
            retained_handoff,
            lowered_mutations,
            TopologySpliceRadialAdjacencyProgramDeclaration::SEMANTIC_FAMILY_KEY,
            mode,
            &sequence,
        )
    }
}
