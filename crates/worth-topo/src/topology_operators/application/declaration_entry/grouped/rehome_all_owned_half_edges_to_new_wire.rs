use crate::topology_operators::declaration_entry::TopologyRehomeAllOwnedHalfEdgesToNewWireDeclaration;
use crate::topology_operators::local_rewrites::sheet_wire_laminar::parse_wire_rehome_program;

use super::super::super::{
    TopologyDeclaredMutationArtifact, TopologyEditApplicationMode, TopologyEditFamily,
    TopologyOperatorExecutionError, TopologyOperatorRunner, TopologyQueryBindingIndex,
};
use super::super::contract_payload::TopologyDeclarationContractPayload;
use super::super::orchestration_boundary::orchestrate_topology_declaration_entry;

impl<'workspace, 'surfaces> TopologyOperatorRunner<'workspace, 'surfaces> {
    pub(crate) fn apply_rehome_all_owned_half_edges_to_new_wire_declaration(
        &mut self,
        declaration: TopologyRehomeAllOwnedHalfEdgesToNewWireDeclaration,
        bindings: &TopologyQueryBindingIndex,
        mode: TopologyEditApplicationMode,
    ) -> Result<TopologyDeclaredMutationArtifact, TopologyOperatorExecutionError> {
        orchestrate_topology_declaration_entry(
            TopologyEditFamily::AttachShellOrWireMembership,
            declaration.clone(),
        )?;

        let contracts = declaration.clone().into_contracts();
        let program = parse_wire_rehome_program(&contracts).expect(
            "canonical wire rehome declaration should lower to a parseable composed wire program",
        );
        let receipt = self.compose_wire_rehome_program(program, &contracts, bindings)?;
        self.finish_composed_membership_execution(
            mode,
            TopologyRehomeAllOwnedHalfEdgesToNewWireDeclaration::SEMANTIC_FAMILY_KEY,
            declaration.semantic_families(),
            declaration.topology_edit_digest(),
            declaration.naming_continuity_matrix(),
            declaration.naming_report(),
            receipt,
        )
    }
}
