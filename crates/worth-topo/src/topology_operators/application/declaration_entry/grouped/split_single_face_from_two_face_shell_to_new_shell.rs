use crate::topology_operators::declaration_entry::TopologySplitSingleFaceFromTwoFaceShellToNewShellDeclaration;
use crate::topology_operators::local_rewrites::sheet_wire_laminar::resolve_single_face_two_face_shell_split_program;

use super::super::super::{
    TopologyEditApplicationMode, TopologyEditFamily, TopologyOperatorExecution,
    TopologyOperatorExecutionError, TopologyOperatorExecutionPath, TopologyOperatorRunner,
    TopologyQueryBindingIndex,
};
use super::super::contract_payload::TopologyDeclarationContractPayload;
use super::super::orchestration_boundary::orchestrate_topology_declaration_entry;

impl<'workspace, 'surfaces> TopologyOperatorRunner<'workspace, 'surfaces> {
    pub(crate) fn apply_split_single_face_from_two_face_shell_to_new_shell_declaration(
        &mut self,
        declaration: TopologySplitSingleFaceFromTwoFaceShellToNewShellDeclaration,
        bindings: &TopologyQueryBindingIndex,
        mode: TopologyEditApplicationMode,
    ) -> Result<TopologyOperatorExecution, TopologyOperatorExecutionError> {
        orchestrate_topology_declaration_entry(
            TopologyEditFamily::AttachShellOrWireMembership,
            declaration.clone(),
        )?;

        let contracts = declaration.clone().into_contracts();
        let program = resolve_single_face_two_face_shell_split_program(bindings, &contracts)
            .expect("canonical shell split declaration should lower to an admitted split program");
        let receipt = self.compose_shell_split_program(program, bindings)?;
        self.finish_composed_membership_execution(
            mode,
            TopologyOperatorExecutionPath::DeclarationEntry {
                semantic_family_key:
                    TopologySplitSingleFaceFromTwoFaceShellToNewShellDeclaration::SEMANTIC_FAMILY_KEY,
            },
            declaration.semantic_families(),
            declaration.topology_edit_digest(),
            declaration.naming_continuity_matrix(),
            declaration.naming_report(),
            receipt,
        )
    }
}
