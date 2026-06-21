use crate::topology_operators::declaration_entry::TopologyRehomeAllOwnedFacesToNewShellDeclaration;
use crate::topology_operators::local_rewrites::sheet_wire_laminar::parse_shell_face_rehome_program;

use super::super::super::{
    TopologyDeclaredMutationArtifact, TopologyMutationApplicationError,
    TopologyMutationApplicationMode, TopologyMutationApplicationRunner, TopologyMutationFamily,
    TopologyQueryBindingIndex,
};
use super::super::mutation_payload::TopologyDeclarationMutationPayload;
use super::super::orchestration_boundary::orchestrate_topology_declaration_entry;

impl<'workspace, 'surfaces> TopologyMutationApplicationRunner<'workspace, 'surfaces> {
    pub(crate) fn apply_rehome_all_owned_faces_to_new_shell_declaration(
        &mut self,
        declaration: TopologyRehomeAllOwnedFacesToNewShellDeclaration,
        bindings: &TopologyQueryBindingIndex,
        mode: TopologyMutationApplicationMode,
    ) -> Result<TopologyDeclaredMutationArtifact, TopologyMutationApplicationError> {
        let retained_handoff = orchestrate_topology_declaration_entry(
            TopologyMutationFamily::AttachShellOrWireMembership,
            declaration.clone(),
            mode.clone(),
        )?;

        let sequence = declaration.clone().into_mutation_sequence();
        let program = parse_shell_face_rehome_program(&sequence).expect(
            "canonical shell rehome declaration should lower to a parseable composed shell program",
        );
        self.compose_shell_rehome_program(
            retained_handoff,
            mode,
            TopologyRehomeAllOwnedFacesToNewShellDeclaration::SEMANTIC_FAMILY_KEY,
            program,
            &sequence,
            bindings,
        )
    }
}
