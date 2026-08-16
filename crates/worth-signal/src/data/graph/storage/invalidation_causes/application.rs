use crate::data::error::SignalError;
use crate::data::graph::storage::invalidation_causes::PendingCauseSetId;
use crate::data::handle::NodeId;
use crate::data::proof::invalidation::binding::{DependencyRevision, ResolvedDependencyCause};
use crate::data::proof::invalidation::output_commit::ProducedAspectDelta;

use crate::data::graph::SignalGraph;

impl SignalGraph {
    #[cfg(test)]
    #[cfg_attr(not(feature = "parallel"), allow(dead_code))]
    pub(crate) fn published_output_commit_order_for_test(&self) -> Vec<(u64, NodeId)> {
        self.cause_sets.published_order_probe.clone()
    }

    pub(crate) fn dependency_revision(
        &self,
        node: NodeId,
    ) -> Result<DependencyRevision, SignalError> {
        Ok(self.get_entry(node)?.dependency_revision())
    }

    pub(crate) fn pending_causes(
        &self,
        node: NodeId,
    ) -> Result<&[ResolvedDependencyCause], SignalError> {
        self.ensure_cause_readmission_complete()?;
        let id = self.get_entry(node)?.pending_cause_set_id();
        self.cause_sets.get(id)
    }

    pub(crate) fn pending_cause_set_id(
        &self,
        node: NodeId,
    ) -> Result<PendingCauseSetId, SignalError> {
        self.ensure_cause_readmission_complete()?;
        Ok(self.get_entry(node)?.pending_cause_set_id())
    }

    pub(crate) fn replace_pending_causes(
        &mut self,
        node: NodeId,
        causes: impl IntoIterator<Item = ResolvedDependencyCause>,
    ) -> Result<PendingCauseSetId, SignalError> {
        let revision = self.dependency_revision(node)?;
        let graph_instance = self.runtime_instance_id();
        let causes = causes.into_iter().collect::<Vec<_>>();
        self.validate_pending_causes(node, &causes)?;
        for cause in &causes {
            if cause.key.graph_instance != graph_instance
                || cause.key.consumer != node
                || cause.key.dependency_revision != revision
                || cause.binding_axes.graph_instance != graph_instance
                || cause.binding_axes.consumer != node
                || cause.binding_axes.dependency_revision != revision
            {
                return Err(SignalError::invalid_input(
                    "pending dependency cause is stale or bound to another consumer",
                ));
            }
        }
        let current = self.get_entry(node)?.pending_cause_set_id();
        let id = self.cause_sets.replace_set(current, causes)?;
        self.get_entry_mut(node)?.set_pending_cause_set_id(id);
        self.rebuild_dirty_caches_from_pending_causes(node)?;
        if !self.cause_readmission_required {
            self.compact_cause_set_storage_if_sparse()?;
        }
        Ok(self.get_entry(node)?.pending_cause_set_id())
    }

    pub(crate) fn replace_prepared_pending_causes(
        &mut self,
        node: NodeId,
        causes: Vec<ResolvedDependencyCause>,
        delta: &ProducedAspectDelta,
    ) -> Result<PendingCauseSetId, SignalError> {
        self.validate_prepared_pending_causes(node, &causes, delta)?;
        let current = self.get_entry(node)?.pending_cause_set_id();
        let id = self.cause_sets.replace_set(current, causes)?;
        self.get_entry_mut(node)?.set_pending_cause_set_id(id);
        self.rebuild_dirty_caches_from_pending_causes(node)?;
        self.compact_cause_set_storage_if_sparse()?;
        Ok(self.get_entry(node)?.pending_cause_set_id())
    }

    pub(crate) fn merge_pending_causes(
        &mut self,
        node: NodeId,
        causes: impl IntoIterator<Item = ResolvedDependencyCause>,
    ) -> Result<PendingCauseSetId, SignalError> {
        let revision = self.dependency_revision(node)?;
        let graph_instance = self.runtime_instance_id();
        let causes = causes.into_iter().collect::<Vec<_>>();
        self.validate_pending_causes(node, &causes)?;
        for cause in &causes {
            if cause.key.graph_instance != graph_instance
                || cause.key.consumer != node
                || cause.key.dependency_revision != revision
            {
                return Err(SignalError::invalid_input(
                    "pending dependency cause is stale or bound to another consumer",
                ));
            }
        }
        let current = self.get_entry(node)?.pending_cause_set_id();
        let id = self.cause_sets.replace(current, causes)?;
        self.get_entry_mut(node)?.set_pending_cause_set_id(id);
        self.rebuild_dirty_caches_from_pending_causes(node)?;
        self.compact_cause_set_storage_if_sparse()?;
        Ok(self.get_entry(node)?.pending_cause_set_id())
    }

    pub(crate) fn rebuild_dirty_caches_from_pending_causes(
        &mut self,
        node: NodeId,
    ) -> Result<(), SignalError> {
        let causes = self.pending_causes(node)?.to_vec();
        let direct = self.get_entry(node)?.direct_invalidation_basis().cloned();
        let mut entry = self.get_entry_mut(node)?;
        entry.set_dirty_aspects(crate::data::aspect::AspectMask::EMPTY);
        entry.clear_dirty_partition_scopes();
        if let Some(direct) = direct {
            entry.set_dirty_aspects(direct.dirty_aspects());
            for (aspect, scope) in direct.scoped_aspects() {
                entry.add_dirty_partition_scope(*aspect, scope.clone());
            }
        }
        for cause in causes {
            entry.add_dirty_aspect(cause.key.aspect);
            for scope in cause.changed_scopes.as_slice() {
                entry.add_dirty_partition_scope(cause.key.aspect, scope.clone());
            }
        }
        Ok(())
    }

    pub(crate) fn release_pending_causes(&mut self, node: NodeId) -> Result<(), SignalError> {
        let current = self.get_entry(node)?.pending_cause_set_id();
        if current == PendingCauseSetId::EMPTY {
            return Ok(());
        }
        self.cause_sets.release(current)?;
        {
            let mut entry = self.get_entry_mut(node)?;
            entry.set_pending_cause_set_id(PendingCauseSetId::EMPTY);
            entry.set_dirty_aspects(crate::data::aspect::AspectMask::EMPTY);
            entry.clear_dirty_partition_scopes();
        }
        self.compact_cause_set_storage_if_sparse()?;
        Ok(())
    }

    fn compact_cause_set_storage_if_sparse(&mut self) -> Result<(), SignalError> {
        if !self.cause_readmission_required && self.cause_sets.should_compact() {
            self.compact_cause_set_storage()?;
        }
        Ok(())
    }

    pub(crate) fn compact_cause_set_storage(&mut self) -> Result<(), SignalError> {
        let remaps = self.cause_sets.rebuild_occupied_generation()?;
        for remap in remaps {
            let mut entry = self.get_entry_mut(remap.consumer)?;
            if entry.pending_cause_set_id() != remap.previous {
                return Err(SignalError::invalid_input(
                    "canonical cause-set handle does not match its consumer",
                ));
            }
            entry.set_pending_cause_set_id(remap.current);
        }
        Ok(())
    }
}
