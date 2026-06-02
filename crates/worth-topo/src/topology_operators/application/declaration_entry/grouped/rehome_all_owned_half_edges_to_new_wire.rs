use crate::topology_operators::declaration_entry::TopologyRehomeAllOwnedHalfEdgesToNewWireDeclaration;
use crate::topology_operators::local_rewrites::sheet_wire_laminar::parse_wire_rehome_program;

use super::super::super::{
    TopologyDeclaredMutationArtifact, TopologyMutationApplicationError,
    TopologyMutationApplicationMode, TopologyMutationApplicationRunner, TopologyMutationFamily,
    TopologyQueryBindingIndex,
};
use super::super::mutation_payload::TopologyDeclarationMutationPayload;
use super::super::orchestration_boundary::orchestrate_topology_declaration_entry;

impl<'workspace, 'surfaces> TopologyMutationApplicationRunner<'workspace, 'surfaces> {
    pub(crate) fn apply_rehome_all_owned_half_edges_to_new_wire_declaration(
        &mut self,
        declaration: TopologyRehomeAllOwnedHalfEdgesToNewWireDeclaration,
        bindings: &TopologyQueryBindingIndex,
        mode: TopologyMutationApplicationMode,
    ) -> Result<TopologyDeclaredMutationArtifact, TopologyMutationApplicationError> {
        let retained_handoff = orchestrate_topology_declaration_entry(
            TopologyMutationFamily::AttachShellOrWireMembership,
            declaration.clone(),
        )?;

        let sequence = declaration.clone().into_mutation_sequence();
        let program = parse_wire_rehome_program(&sequence).expect(
            "canonical wire rehome declaration should lower to a parseable composed wire program",
        );
        let receipt = self.compose_wire_rehome_program(program, &sequence, bindings)?;
        self.finish_composed_membership_execution(
            mode,
            retained_handoff,
            TopologyRehomeAllOwnedHalfEdgesToNewWireDeclaration::SEMANTIC_FAMILY_KEY,
            &sequence,
            receipt,
        )
    }
}
