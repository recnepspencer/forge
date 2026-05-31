use crate::topology_operators::declaration_entry::TopologyRetireTopologyEntityDeclaration;

use super::super::super::{
    TopologyDeclaredMutationArtifact, TopologyEditApplicationMode, TopologyOperatorExecutionError,
    TopologyOperatorRunner, TopologyQueryBindingIndex,
};
use super::shared::apply_scalar_declaration;

impl<'workspace, 'surfaces> TopologyOperatorRunner<'workspace, 'surfaces> {
    pub(crate) fn apply_retire_topology_entity_declaration(
        &mut self,
        declaration: TopologyRetireTopologyEntityDeclaration,
        bindings: &TopologyQueryBindingIndex,
        mode: TopologyEditApplicationMode,
    ) -> Result<TopologyDeclaredMutationArtifact, TopologyOperatorExecutionError> {
        apply_scalar_declaration(self, declaration, bindings, mode)
    }
}
