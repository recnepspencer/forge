use crate::data::aspect::Aspect;
use crate::data::handle::NodeId;
use crate::data::output::{InternedPartitionSubscription, PartitionMatchMode};

use super::fork_overlay::{
    extend_merged_set, BucketDelta, ReverseSubscriptionStorage, SetMergeTraversal,
};
use super::{
    DetailScopeKey, ProducerAspectKey, ReverseSubscriptionIndex, ReverseSubscriptionQuery,
    SubscriberScopeBuckets,
};

impl ReverseSubscriptionIndex {
    pub(crate) fn query_whole_aspect(
        &self,
        producer: NodeId,
        aspect: Aspect,
    ) -> ReverseSubscriptionQuery {
        self.query_whole_aspect_observed(producer, aspect).0
    }

    fn query_whole_aspect_observed(
        &self,
        producer: NodeId,
        aspect: Aspect,
    ) -> (ReverseSubscriptionQuery, SetMergeTraversal) {
        let key = ProducerAspectKey::from_committed_output(producer, aspect);
        let (base, delta) = self.bucket_view(&key);
        if base.is_none() && delta.is_none() {
            return (empty_query(1), SetMergeTraversal::default());
        }
        let mut candidates = Vec::new();
        let traversal = extend_merged_set(
            base.map(|buckets| &buckets.all),
            delta.map(|delta| &delta.all),
            &mut candidates,
        );
        (finish_query(candidates, 1), traversal)
    }

    #[cfg(test)]
    pub(super) fn query_whole_aspect_with_traversal(
        &self,
        producer: NodeId,
        aspect: Aspect,
    ) -> (ReverseSubscriptionQuery, SetMergeTraversal) {
        self.query_whole_aspect_observed(producer, aspect)
    }

    pub(super) fn query_unscoped(
        &self,
        producer: NodeId,
        aspect: Aspect,
    ) -> ReverseSubscriptionQuery {
        let key = ProducerAspectKey::from_committed_output(producer, aspect);
        let (base, delta) = self.bucket_view(&key);
        if base.is_none() && delta.is_none() {
            return empty_query(1);
        }
        let mut candidates = Vec::new();
        let _ = extend_merged_set(
            base.map(|buckets| &buckets.unscoped),
            delta.map(|delta| &delta.unscoped),
            &mut candidates,
        );
        finish_query(candidates, 1)
    }

    pub(crate) fn query_scope(
        &self,
        producer: NodeId,
        aspect: Aspect,
        scope: InternedPartitionSubscription,
    ) -> ReverseSubscriptionQuery {
        let key = ProducerAspectKey::from_committed_output(producer, aspect);
        let (base, delta) = self.bucket_view(&key);
        if base.is_none() && delta.is_none() {
            return empty_query(match scope.match_mode {
                PartitionMatchMode::WholePartition => 2,
                PartitionMatchMode::PartitionAndDetail => 3,
            });
        }
        let mut candidates = Vec::new();
        let _ = extend_merged_set(
            base.map(|buckets| &buckets.unscoped),
            delta.map(|delta| &delta.unscoped),
            &mut candidates,
        );
        let probes = match scope.match_mode {
            PartitionMatchMode::WholePartition => {
                let _ = extend_merged_set(
                    base.and_then(|buckets| buckets.partition_scoped.get(&scope.partition)),
                    delta.and_then(|delta| delta.partition_scoped.get(&scope.partition)),
                    &mut candidates,
                );
                2
            }
            PartitionMatchMode::PartitionAndDetail => {
                let _ = extend_merged_set(
                    base.and_then(|buckets| buckets.whole_partitions.get(&scope.partition)),
                    delta.and_then(|delta| delta.whole_partitions.get(&scope.partition)),
                    &mut candidates,
                );
                if let Some(detail) = scope.detail {
                    let key = DetailScopeKey {
                        partition: scope.partition,
                        detail,
                    };
                    let _ = extend_merged_set(
                        base.and_then(|buckets| buckets.exact_details.get(&key)),
                        delta.and_then(|delta| delta.exact_details.get(&key)),
                        &mut candidates,
                    );
                }
                3
            }
        };
        finish_query(candidates, probes)
    }

    fn bucket_view(
        &self,
        key: &ProducerAspectKey,
    ) -> (Option<&SubscriberScopeBuckets>, Option<&BucketDelta>) {
        match &self.storage {
            ReverseSubscriptionStorage::Exclusive(flat) => (flat.buckets.get(key), None),
            ReverseSubscriptionStorage::ForkShared {
                base,
                bucket_changes,
                ..
            } => (base.buckets.get(key), bucket_changes.get(key)),
        }
    }
}

fn empty_query(bucket_probes: u64) -> ReverseSubscriptionQuery {
    ReverseSubscriptionQuery {
        candidates: Vec::new(),
        bucket_probes,
    }
}

fn finish_query(mut candidates: Vec<NodeId>, bucket_probes: u64) -> ReverseSubscriptionQuery {
    candidates.sort_unstable();
    candidates.dedup();
    ReverseSubscriptionQuery {
        candidates,
        bucket_probes,
    }
}
