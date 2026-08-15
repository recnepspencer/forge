use crate::data::error::SignalError;
use crate::data::graph::signal_graph::SignalGraph;

impl SignalGraph {
    pub(crate) fn rebuild_reverse_subscription_index_from_dependencies(
        &mut self,
    ) -> Result<(), SignalError> {
        let authority = self
            .live_node_ids()
            .into_iter()
            .map(|consumer| {
                self.raw_dependencies_of(consumer)
                    .map(|edges| (consumer, edges.to_vec()))
            })
            .collect::<Result<Vec<_>, _>>()?;
        self.topology.reverse_subscriptions.clear();
        for (consumer, edges) in authority {
            self.replace_reverse_subscriptions_for_consumer(consumer, &edges)?;
        }
        self.topology.reverse_subscriptions.mark_rebuilt();
        Ok(())
    }

    #[cfg(test)]
    pub(crate) fn destroy_reverse_subscription_index_for_test(&mut self) {
        self.topology.reverse_subscriptions.clear();
    }
}
