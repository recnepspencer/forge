use crate::data::aspect::Aspect;
use crate::data::dependency::{CanonicalDependencies, DependencyEdge};
use crate::data::error::SignalError;
use crate::data::handle::NodeId;
use crate::data::output::PartitionSubscription;
use crate::data::proof::DependencyBatchEdit;

use crate::data::graph::signal_graph::SignalGraph;

impl SignalGraph {
    pub fn set_dependencies(
        &mut self,
        node: NodeId,
        desired: impl IntoIterator<Item = DependencyEdge>,
    ) -> Result<(), SignalError> {
        let desired = CanonicalDependencies::new(desired);
        let _ = self.reconcile_dependencies(node, desired.as_slice())?;
        Ok(())
    }

    pub fn clear_dependencies(&mut self, node: NodeId) -> Result<(), SignalError> {
        self.set_dependencies(node, std::iter::empty())
    }

    #[cfg(test)]
    pub(crate) fn edit_dependencies(
        &mut self,
        node: NodeId,
        edit: impl FnOnce(&mut Vec<DependencyEdge>),
    ) -> Result<(), SignalError> {
        let mut desired = self.dependencies_of(node)?.to_vec();
        edit(&mut desired);
        self.set_dependencies(node, desired)
    }

    pub fn apply_dependency_batch_edit(
        &mut self,
        edit: &DependencyBatchEdit,
    ) -> Result<(), SignalError> {
        let reconciliations = edit
            .as_slice()
            .iter()
            .map(|entry| (entry.node, entry.dependencies.clone()))
            .collect::<Vec<_>>();
        let _ = self.reconcile_dependencies_batch(&reconciliations)?;
        Ok(())
    }

    pub(crate) fn build_dependency_edge(
        &mut self,
        upstream: NodeId,
        aspect: Aspect,
        scope: Option<PartitionSubscription>,
    ) -> DependencyEdge {
        match scope {
            Some(scope) => {
                let token_count_before = self.observation.partition_interner.token_count();
                let interned_scope = self
                    .observation
                    .partition_interner
                    .intern_subscription(&scope);
                self.observation
                    .telemetry
                    .invalidation
                    .partition_interner_growth_delta +=
                    self.observation
                        .partition_interner
                        .token_count()
                        .saturating_sub(token_count_before) as u64;
                DependencyEdge::with_scope(upstream, aspect, scope, interned_scope)
            }
            None => DependencyEdge::new(upstream, aspect),
        }
    }

    pub(super) fn intern_dependency_edges(
        &mut self,
        desired: CanonicalDependencies,
    ) -> CanonicalDependencies {
        CanonicalDependencies::new(desired.as_slice().iter().map(|edge| {
            self.build_dependency_edge(edge.source(), edge.aspect(), edge.scope_ref().cloned())
        }))
    }
}
