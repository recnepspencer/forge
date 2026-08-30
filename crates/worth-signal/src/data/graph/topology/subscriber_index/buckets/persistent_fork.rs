use super::ReverseSubscriptionIndex;

impl ReverseSubscriptionIndex {
    pub(crate) fn fork_persistent(&mut self) -> Self {
        Self {
            buckets: self.buckets.fork_persistent(),
            by_consumer: self.by_consumer.fork_persistent(),
            valid: self.valid,
        }
    }

    #[cfg(test)]
    pub(crate) fn fork_storage_identity(&self) -> Self {
        Self {
            buckets: self.buckets.fork_storage_identity(),
            by_consumer: self.by_consumer.fork_storage_identity(),
            valid: self.valid,
        }
    }

    #[cfg(test)]
    pub(crate) fn shares_storage_with(&self, other: &Self) -> bool {
        self.buckets.ptr_eq(&other.buckets) && self.by_consumer.ptr_eq(&other.by_consumer)
    }
}
