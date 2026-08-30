use crate::data::dependency::DependencyEdge;
use crate::data::error::SignalError;
use crate::data::graph::signal_graph::SignalGraph;
use crate::data::handle::NodeId;

use super::buckets::IndexedSubscriptionMembership;

impl SignalGraph {
    pub(in crate::data::graph) fn replace_reverse_subscriptions_for_consumer(
        &mut self,
        consumer: NodeId,
        edges: &[DependencyEdge],
    ) -> Result<(), SignalError> {
        self.validate_handle(consumer)?;
        let memberships = edges
            .iter()
            .map(|edge| {
                IndexedSubscriptionMembership::from_edge(
                    edge.source(),
                    edge.aspect(),
                    edge.interned_scope(),
                )
                .ok_or_else(|| {
                    SignalError::internal(
                        "scoped dependency reached reverse indexing without interned scope",
                    )
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
        self.topology
            .reverse_subscriptions
            .replace_consumer(consumer, memberships);
        Ok(())
    }
}

pub(super) fn remove_member<Key: Ord + Copy>(
    buckets: &mut im::OrdMap<Key, im::OrdSet<NodeId>>,
    key: Key,
    consumer: NodeId,
) {
    let Some(members) = buckets.get_mut(&key) else {
        return;
    };
    members.remove(&consumer);
    if members.is_empty() {
        buckets.remove(&key);
    }
}
