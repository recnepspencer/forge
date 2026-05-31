use crate::topology_operators::declaration_entry::TopologySplitConnectedHalfEdgeSetToNewWireDeclaration;
use crate::topology_operators::local_rewrites::sheet_wire_laminar::resolve_wire_split_program;

use super::super::super::{
    TopologyDeclaredMutationArtifact, TopologyEditApplicationMode, TopologyEditFamily,
    TopologyOperatorExecutionError, TopologyOperatorRunner, TopologyQueryBindingIndex,
};
use super::super::contract_payload::TopologyDeclarationContractPayload;
use super::super::orchestration_boundary::orchestrate_topology_declaration_entry;

impl<'workspace, 'surfaces> TopologyOperatorRunner<'workspace, 'surfaces> {
    pub(crate) fn apply_split_connected_half_edge_set_to_new_wire_declaration(
        &mut self,
        declaration: TopologySplitConnectedHalfEdgeSetToNewWireDeclaration,
        bindings: &TopologyQueryBindingIndex,
        mode: TopologyEditApplicationMode,
    ) -> Result<TopologyDeclaredMutationArtifact, TopologyOperatorExecutionError> {
        orchestrate_topology_declaration_entry(
            TopologyEditFamily::AttachShellOrWireMembership,
            declaration.clone(),
        )?;

        let contracts = declaration.clone().into_contracts();
        let program = resolve_wire_split_program(bindings, &contracts)
            .expect("canonical wire split declaration should lower to an admitted split program");
        let receipt = self.compose_wire_split_program(program, bindings)?;
        self.finish_composed_membership_execution(
            mode,
            TopologySplitConnectedHalfEdgeSetToNewWireDeclaration::SEMANTIC_FAMILY_KEY,
            declaration.semantic_families(),
            declaration.topology_edit_digest(),
            declaration.naming_continuity_matrix(),
            declaration.naming_report(),
            receipt,
        )
    }
}
