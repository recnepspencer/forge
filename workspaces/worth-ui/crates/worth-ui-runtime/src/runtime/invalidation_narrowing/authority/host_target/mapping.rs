use crate::graph::UiGraphNodeIdentity;
use crate::runtime::persistent_index::UiPersistentIndexMutationWork;

use super::UiHostInvalidationTargetMapping;

impl UiHostInvalidationTargetMapping {
    pub(in crate::runtime::invalidation_narrowing) fn from_owners(
        owners: crate::runtime::persistent_index::UiPersistentOrdMap<UiGraphNodeIdentity, usize>,
    ) -> Self {
        Self {
            node_owners: owners,
        }
    }

    pub(in crate::runtime::invalidation_narrowing) fn add_owner(
        &mut self,
        node: UiGraphNodeIdentity,
    ) -> Result<UiPersistentIndexMutationWork, ()> {
        let (owners, probes) = self.node_owners.get_with_probes(&node);
        let next = owners
            .copied()
            .unwrap_or_default()
            .checked_add(1)
            .ok_or(())?;
        let mut work = UiPersistentIndexMutationWork::with_key_probes(probes);
        work.merge(self.node_owners.insert_with_work(node, next))?;
        Ok(work)
    }

    pub(in crate::runtime::invalidation_narrowing) fn remove_owner(
        &mut self,
        node: UiGraphNodeIdentity,
    ) -> Result<UiPersistentIndexMutationWork, ()> {
        let (owners, probes) = self.node_owners.get_with_probes(&node);
        let owners = owners.copied().ok_or(())?;
        let mut work = UiPersistentIndexMutationWork::with_key_probes(probes);
        if owners == 1 {
            let (removed, mutation) = self.node_owners.remove_with_work(&node);
            if !removed {
                return Err(());
            }
            work.merge(mutation)?;
        } else {
            work.merge(self.node_owners.insert_with_work(node, owners - 1))?;
        }
        Ok(work)
    }

    pub(in crate::runtime::invalidation_narrowing) fn is_empty(&self) -> bool {
        self.node_owners.is_empty()
    }

    pub(super) fn materialize(
        &self,
        graph: &crate::graph::UiGraphReplanAuthority,
    ) -> Option<crate::graph::UiAdmittedAllocationInvalidationTargetSet> {
        let nodes = self
            .node_owners
            .iter()
            .map(|(node, _)| *node)
            .collect::<Vec<_>>();
        graph.target_set_for_nodes(&nodes)
    }
}
