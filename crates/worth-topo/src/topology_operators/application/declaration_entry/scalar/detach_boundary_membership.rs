use crate::topology_operators::declaration_entry::TopologyDetachBoundaryMembershipDeclaration;

use super::super::super::{
    TopologyEditApplicationMode, TopologyOperatorExecution, TopologyOperatorExecutionError,
    TopologyOperatorRunner, TopologyQueryBindingIndex,
};
use super::shared::apply_scalar_declaration;

impl<'workspace, 'surfaces> TopologyOperatorRunner<'workspace, 'surfaces> {
    pub(crate) fn apply_detach_boundary_membership_declaration(
        &mut self,
        declaration: TopologyDetachBoundaryMembershipDeclaration,
        bindings: &TopologyQueryBindingIndex,
        mode: TopologyEditApplicationMode,
    ) -> Result<TopologyOperatorExecution, TopologyOperatorExecutionError> {
        apply_scalar_declaration(self, declaration, bindings, mode)
    }
}
