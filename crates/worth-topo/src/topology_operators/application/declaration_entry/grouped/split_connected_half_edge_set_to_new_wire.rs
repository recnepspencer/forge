use crate::topology_operators::declaration_entry::TopologySplitConnectedHalfEdgeSetToNewWireDeclaration;
use crate::topology_operators::local_rewrites::sheet_wire_laminar::resolve_wire_split_program;

use super::super::super::{
    TopologyDeclaredMutationArtifact, TopologyMutationApplicationError,
    TopologyMutationApplicationMode, TopologyMutationApplicationRunner, TopologyMutationFamily,
    TopologyQueryBindingIndex,
};
use super::super::mutation_payload::TopologyDeclarationMutationPayload;
use super::super::orchestration_boundary::orchestrate_topology_declaration_entry;

impl<'workspace, 'surfaces> TopologyMutationApplicationRunner<'workspace, 'surfaces> {
    pub(crate) fn apply_split_connected_half_edge_set_to_new_wire_declaration(
        &mut self,
        declaration: TopologySplitConnectedHalfEdgeSetToNewWireDeclaration,
        bindings: &TopologyQueryBindingIndex,
        mode: TopologyMutationApplicationMode,
    ) -> Result<TopologyDeclaredMutationArtifact, TopologyMutationApplicationError> {
        let retained_handoff = orchestrate_topology_declaration_entry(
            TopologyMutationFamily::AttachShellOrWireMembership,
            declaration.clone(),
        )?;

        let sequence = declaration.clone().into_mutation_sequence();
        let program = resolve_wire_split_program(bindings, &sequence)
            .expect("canonical wire split declaration should lower to an admitted split program");
        let receipt = self.compose_wire_split_program(program, bindings)?;
        self.finish_composed_membership_execution(
            mode,
            retained_handoff,
            TopologySplitConnectedHalfEdgeSetToNewWireDeclaration::SEMANTIC_FAMILY_KEY,
            &sequence,
            receipt,
        )
    }
}
