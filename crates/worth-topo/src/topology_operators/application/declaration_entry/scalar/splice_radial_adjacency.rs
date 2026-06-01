use crate::topology_operators::declaration_entry::TopologySpliceRadialAdjacencyDeclaration;

use super::super::super::{
    TopologyDeclaredMutationArtifact, TopologyMutationApplicationError,
    TopologyMutationApplicationMode, TopologyMutationApplicationRunner, TopologyQueryBindingIndex,
};
use super::shared::apply_scalar_declaration;

impl<'workspace, 'surfaces> TopologyMutationApplicationRunner<'workspace, 'surfaces> {
    pub(crate) fn apply_splice_radial_adjacency_declaration(
        &mut self,
        declaration: TopologySpliceRadialAdjacencyDeclaration,
        bindings: &TopologyQueryBindingIndex,
        mode: TopologyMutationApplicationMode,
    ) -> Result<TopologyDeclaredMutationArtifact, TopologyMutationApplicationError> {
        apply_scalar_declaration(self, declaration, bindings, mode)
    }
}
