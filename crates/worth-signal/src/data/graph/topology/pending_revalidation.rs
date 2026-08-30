use crate::data::error::SignalError;
use crate::data::graph::signal_graph::SignalGraph;
use crate::data::handle::NodeId;

impl SignalGraph {
    pub(in crate::data::graph) fn replace_pending_revalidation_waiters(
        &mut self,
        consumer: NodeId,
        previous: &[NodeId],
        current: &[NodeId],
    ) {
        for producer in previous {
            if let Some(waiters) = self.topology.pending_revalidation_waiters.get_mut(producer) {
                waiters.remove(&consumer);
                if waiters.is_empty() {
                    self.topology.pending_revalidation_waiters.remove(producer);
                }
            }
        }
        for producer in current {
            self.topology
                .pending_revalidation_waiters
                .entry(*producer)
                .or_default()
                .insert(consumer);
        }
    }

    pub(crate) fn pending_revalidation_waiters(
        &mut self,
        producer: NodeId,
    ) -> Result<Vec<NodeId>, SignalError> {
        let candidates = self
            .topology
            .pending_revalidation_waiters
            .get(&producer)
            .cloned()
            .unwrap_or_default();
        let mut current = im::OrdSet::new();
        for consumer in candidates {
            if !self.is_alive(consumer) {
                continue;
            }
            if self
                .pending_dependency_revalidation(consumer)?
                .is_some_and(|pending| pending.unresolved_producers().contains(&producer))
            {
                current.insert(consumer);
            }
        }
        if current.is_empty() {
            self.topology.pending_revalidation_waiters.remove(&producer);
        } else {
            self.topology
                .pending_revalidation_waiters
                .insert(producer, current.clone());
        }
        Ok(current.into_iter().collect())
    }

    pub(crate) fn rebuild_pending_revalidation_waiters(&mut self) -> Result<(), SignalError> {
        let mut rebuilt =
            crate::data::persistent_ord_map::PersistentOrdMap::<NodeId, im::OrdSet<NodeId>>::new();
        for consumer in self.live_node_ids() {
            let Some(pending) = self.pending_dependency_revalidation(consumer)? else {
                continue;
            };
            for producer in pending.unresolved_producers() {
                rebuilt.entry(*producer).or_default().insert(consumer);
            }
        }
        self.topology.pending_revalidation_waiters = rebuilt;
        Ok(())
    }
}
