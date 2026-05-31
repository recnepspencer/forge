use crate::topology_operators::declaration_entry::TopologyDetachShellOrWireMembershipDeclaration;

use super::super::super::{
    TopologyDeclaredMutationArtifact, TopologyMutationApplicationError,
    TopologyMutationApplicationMode, TopologyMutationApplicationRunner, TopologyQueryBindingIndex,
};
use super::shared::apply_scalar_declaration;

impl<'workspace, 'surfaces> TopologyMutationApplicationRunner<'workspace, 'surfaces> {
    pub(crate) fn apply_detach_shell_or_wire_membership_declaration(
        &mut self,
        declaration: TopologyDetachShellOrWireMembershipDeclaration,
        bindings: &TopologyQueryBindingIndex,
        mode: TopologyMutationApplicationMode,
    ) -> Result<TopologyDeclaredMutationArtifact, TopologyMutationApplicationError> {
        apply_scalar_declaration(self, declaration, bindings, mode)
    }
}
