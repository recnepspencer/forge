use crate::topology_operators::declaration_entry::TopologyRehomeAllOwnedFacesToNewShellDeclaration;
use crate::topology_operators::local_rewrites::sheet_wire_laminar::parse_shell_face_rehome_program;

use super::super::super::{
    TopologyEditApplicationMode, TopologyEditFamily, TopologyOperatorExecution,
    TopologyOperatorExecutionError, TopologyOperatorExecutionPath, TopologyOperatorRunner,
    TopologyQueryBindingIndex,
};
use super::super::contract_payload::TopologyDeclarationContractPayload;
use super::super::orchestration_boundary::orchestrate_topology_declaration_entry;

impl<'workspace, 'surfaces> TopologyOperatorRunner<'workspace, 'surfaces> {
    pub(crate) fn apply_rehome_all_owned_faces_to_new_shell_declaration(
        &mut self,
        declaration: TopologyRehomeAllOwnedFacesToNewShellDeclaration,
        bindings: &TopologyQueryBindingIndex,
        mode: TopologyEditApplicationMode,
    ) -> Result<TopologyOperatorExecution, TopologyOperatorExecutionError> {
        orchestrate_topology_declaration_entry(
            TopologyEditFamily::AttachShellOrWireMembership,
            declaration.clone(),
        )?;

        let contracts = declaration.clone().into_contracts();
        let program = parse_shell_face_rehome_program(&contracts).expect(
            "canonical shell rehome declaration should lower to a parseable composed shell program",
        );
        let receipt = self.compose_shell_rehome_program(program, &contracts, bindings)?;
        self.finish_composed_membership_execution(
            mode,
            TopologyOperatorExecutionPath::DeclarationEntry {
                semantic_family_key:
                    TopologyRehomeAllOwnedFacesToNewShellDeclaration::SEMANTIC_FAMILY_KEY,
            },
            declaration.semantic_families(),
            declaration.topology_edit_digest(),
            declaration.naming_continuity_matrix(),
            declaration.naming_report(),
            receipt,
        )
    }
}
