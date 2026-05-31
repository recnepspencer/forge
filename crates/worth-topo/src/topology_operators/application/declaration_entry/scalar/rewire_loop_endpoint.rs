use crate::topology_operators::declaration_entry::TopologyRewireLoopEndpointDeclaration;

use super::super::super::{
    TopologyEditApplicationMode, TopologyOperatorExecution, TopologyOperatorExecutionError,
    TopologyOperatorRunner, TopologyQueryBindingIndex,
};
use super::shared::apply_scalar_declaration;

impl<'workspace, 'surfaces> TopologyOperatorRunner<'workspace, 'surfaces> {
    pub(crate) fn apply_rewire_loop_endpoint_declaration(
        &mut self,
        declaration: TopologyRewireLoopEndpointDeclaration,
        bindings: &TopologyQueryBindingIndex,
        mode: TopologyEditApplicationMode,
    ) -> Result<TopologyOperatorExecution, TopologyOperatorExecutionError> {
        apply_scalar_declaration(self, declaration, bindings, mode)
    }
}
