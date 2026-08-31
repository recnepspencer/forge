use std::sync::Arc;

use super::fork_overlay::ReverseSubscriptionStorage;
use super::ReverseSubscriptionIndex;

impl ReverseSubscriptionIndex {
    pub(crate) fn fork_persistent(&mut self) -> Self {
        if let ReverseSubscriptionStorage::Exclusive(flat) = &mut self.storage {
            self.storage = ReverseSubscriptionStorage::ForkShared {
                base: Arc::new(std::mem::take(flat)),
                bucket_changes: im::OrdMap::new(),
                consumer_changes: im::OrdMap::new(),
            };
        }
        self.fork_storage_identity_impl()
    }

    #[cfg(test)]
    pub(crate) fn fork_storage_identity(&self) -> Self {
        self.fork_storage_identity_impl()
    }

    fn fork_storage_identity_impl(&self) -> Self {
        let storage = match &self.storage {
            ReverseSubscriptionStorage::ForkShared {
                base,
                bucket_changes,
                consumer_changes,
            } => ReverseSubscriptionStorage::ForkShared {
                base: Arc::clone(base),
                bucket_changes: bucket_changes.clone(),
                consumer_changes: consumer_changes.clone(),
            },
            ReverseSubscriptionStorage::Exclusive(_) => {
                unreachable!("persistent fork must install shared storage")
            }
        };
        Self {
            storage,
            valid: self.valid,
        }
    }

    #[cfg(test)]
    pub(crate) fn shares_storage_with(&self, other: &Self) -> bool {
        matches!(
            (&self.storage, &other.storage),
            (
                ReverseSubscriptionStorage::ForkShared {
                    base: left_base,
                    bucket_changes: left_buckets,
                    consumer_changes: left_consumers,
                },
                ReverseSubscriptionStorage::ForkShared {
                    base: right_base,
                    bucket_changes: right_buckets,
                    consumer_changes: right_consumers,
                },
            ) if Arc::ptr_eq(left_base, right_base)
                && left_buckets.ptr_eq(right_buckets)
                && left_consumers.ptr_eq(right_consumers)
        )
    }
}
