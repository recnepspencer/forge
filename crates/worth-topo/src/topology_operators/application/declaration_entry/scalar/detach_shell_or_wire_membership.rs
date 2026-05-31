use crate::topology_operators::declaration_entry::TopologyDetachShellOrWireMembershipDeclaration;

use super::super::super::{
    TopologyDeclaredMutationArtifact, TopologyEditApplicationMode, TopologyOperatorExecutionError,
    TopologyOperatorRunner, TopologyQueryBindingIndex,
};
use super::shared::apply_scalar_declaration;

impl<'workspace, 'surfaces> TopologyOperatorRunner<'workspace, 'surfaces> {
    pub(crate) fn apply_detach_shell_or_wire_membership_declaration(
        &mut self,
        declaration: TopologyDetachShellOrWireMembershipDeclaration,
        bindings: &TopologyQueryBindingIndex,
        mode: TopologyEditApplicationMode,
    ) -> Result<TopologyDeclaredMutationArtifact, TopologyOperatorExecutionError> {
        apply_scalar_declaration(self, declaration, bindings, mode)
    }
}
