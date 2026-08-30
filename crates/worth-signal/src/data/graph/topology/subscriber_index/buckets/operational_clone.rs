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

    #[cfg(test)]
    pub(crate) fn shares_storage_with(&self, other: &Self) -> bool {
        self.buckets.ptr_eq(&other.buckets) && self.by_consumer.ptr_eq(&other.by_consumer)
    }
}

impl SubscriberScopeBuckets {
    fn operational_clone(&self) -> Self {
        Self {
            all: self.all.iter().copied().collect(),
            unscoped: self.unscoped.iter().copied().collect(),
            whole_partitions: self
                .whole_partitions
                .iter()
                .map(|(key, nodes)| (*key, nodes.iter().copied().collect::<im::OrdSet<_>>()))
                .collect(),
            exact_details: self
                .exact_details
                .iter()
                .map(|(key, nodes)| (*key, nodes.iter().copied().collect::<im::OrdSet<_>>()))
                .collect(),
            partition_scoped: self
                .partition_scoped
                .iter()
                .map(|(key, nodes)| (*key, nodes.iter().copied().collect::<im::OrdSet<_>>()))
                .collect(),
        }
    }
}
