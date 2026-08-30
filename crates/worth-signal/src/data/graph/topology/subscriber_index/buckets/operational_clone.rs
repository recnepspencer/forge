use super::{ReverseSubscriptionIndex, SubscriberScopeBuckets};

impl ReverseSubscriptionIndex {
    pub(crate) fn operational_clone(&self) -> Self {
        Self {
            buckets: self
                .buckets
                .iter()
                .map(|(key, buckets)| (*key, buckets.operational_clone()))
                .collect(),
            by_consumer: self
                .by_consumer
                .iter()
                .map(|(node, memberships)| (*node, memberships.clone()))
                .collect(),
            valid: self.valid,
        }
    }
}

impl SubscriberScopeBuckets {
    fn operational_clone(&self) -> Self {
        Self {
            all: self.all.clone(),
            unscoped: self.unscoped.clone(),
            whole_partitions: self
                .whole_partitions
                .iter()
                .map(|(key, nodes)| (*key, nodes.clone()))
                .collect(),
            exact_details: self
                .exact_details
                .iter()
                .map(|(key, nodes)| (*key, nodes.clone()))
                .collect(),
            partition_scoped: self
                .partition_scoped
                .iter()
                .map(|(key, nodes)| (*key, nodes.clone()))
                .collect(),
        }
    }
}
