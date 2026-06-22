use crate::topology_operators::declaration_entry::TopologySplitSingleFaceFromTwoFaceShellToNewShellDeclaration;
use crate::topology_operators::local_rewrites::sheet_wire_laminar::resolve_single_face_two_face_shell_split_program;

use super::super::super::{
    TopologyDeclaredMutationArtifact, TopologyMutationApplicationError,
    TopologyMutationApplicationMode, TopologyMutationApplicationRunner, TopologyMutationFamily,
    TopologyQueryBindingIndex,
};
use super::super::mutation_payload::TopologyDeclarationMutationPayload;
use super::super::orchestration_boundary::orchestrate_topology_declaration_entry;

impl<'workspace, 'surfaces> TopologyMutationApplicationRunner<'workspace, 'surfaces> {
    pub(crate) fn apply_split_single_face_from_two_face_shell_to_new_shell_declaration(
        &mut self,
        declaration: TopologySplitSingleFaceFromTwoFaceShellToNewShellDeclaration,
        bindings: &TopologyQueryBindingIndex,
        mode: TopologyMutationApplicationMode,
    ) -> Result<TopologyDeclaredMutationArtifact, TopologyMutationApplicationError> {
        let retained_handoff = orchestrate_topology_declaration_entry(
            TopologyMutationFamily::AttachShellOrWireMembership,
            declaration.clone(),
            mode.clone(),
        )?;

        let sequence = declaration.clone().into_mutation_sequence();
        let program = resolve_single_face_two_face_shell_split_program(bindings, &sequence)
            .expect("canonical shell split declaration should lower to an admitted split program");
        self.compose_shell_split_program(
            retained_handoff,
            mode,
            TopologySplitSingleFaceFromTwoFaceShellToNewShellDeclaration::SEMANTIC_FAMILY_KEY,
            program,
            &sequence,
            bindings,
        )
    }
}
