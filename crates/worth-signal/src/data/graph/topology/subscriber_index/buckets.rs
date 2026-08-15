use std::collections::{BTreeMap, BTreeSet};

use crate::data::aspect::Aspect;
use crate::data::error::SignalError;
use crate::data::graph::signal_graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::data::output::{
    DetailTokenId, InternedPartitionSubscription, PartitionMatchMode, PartitionTokenId,
};
use crate::data::proof::invalidation::output_commit::{ProducedAspectChange, ScopePrecision};

use super::membership::remove_member;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum IndexedSubscriptionScope {
    Unscoped,
    WholePartition(PartitionTokenId),
    Detail(PartitionTokenId, DetailTokenId),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(super) struct ProducerAspectKey {
    producer: NodeId,
    aspect: Aspect,
}

impl ProducerAspectKey {
    fn from_authoritative_edge(producer: NodeId, aspect: Aspect) -> Self {
        Self { producer, aspect }
    }

    fn from_committed_output(producer: NodeId, aspect: Aspect) -> Self {
        Self { producer, aspect }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct DetailScopeKey {
    partition: PartitionTokenId,
    detail: DetailTokenId,
}

impl SignalGraph {
    pub(crate) fn indexed_consumers_for_declared_outputs(
        &self,
        producer: NodeId,
    ) -> Result<Vec<NodeId>, SignalError> {
        if !self.topology.reverse_subscriptions.is_valid() {
            return Err(SignalError::internal(
                "reverse subscription index requires authority rebuild",
            ));
        }
        let produces = self.get_contract(producer)?.semantics.produces;
        let mut consumers = Vec::new();
        for index in 0..crate::data::aspect::MAX_ASPECTS {
            let aspect = Aspect::new(index as u8);
            if !produces.contains(crate::data::aspect::AspectMask::from_aspect(aspect)) {
                continue;
            }
            consumers.extend(
                self.topology
                    .reverse_subscriptions
                    .query_whole_aspect(producer, aspect)
                    .candidates,
            );
        }
        consumers.sort_unstable();
        consumers.dedup();
        consumers.retain(|consumer| self.is_alive(*consumer));
        Ok(consumers)
    }

    pub(crate) fn query_reverse_subscriptions(
        &mut self,
        producer: NodeId,
        change: &ProducedAspectChange,
        precision: ScopePrecision,
    ) -> Result<ReverseSubscriptionQuery, SignalError> {
        if !self.topology.reverse_subscriptions.is_valid() {
            return Err(SignalError::internal(
                "reverse subscription index requires authority rebuild",
            ));
        }
        let mut result = if precision == ScopePrecision::ConservativeLegacyUnion
            || change.changed_scopes.is_empty()
        {
            self.topology
                .reverse_subscriptions
                .query_whole_aspect(producer, change.aspect)
        } else {
            let mut candidates = Vec::new();
            let mut bucket_probes = 0;
            for scope in change.changed_scopes.as_slice() {
                let Some(interned) = self
                    .observation
                    .partition_interner()
                    .resolve_subscription(scope)
                else {
                    let query = self
                        .topology
                        .reverse_subscriptions
                        .query_unscoped(producer, change.aspect);
                    bucket_probes += query.bucket_probes;
                    candidates.extend(query.candidates);
                    continue;
                };
                let query = self.topology.reverse_subscriptions.query_scope(
                    producer,
                    change.aspect,
                    interned,
                );
                bucket_probes += query.bucket_probes;
                candidates.extend(query.candidates);
            }
            candidates.sort_unstable();
            candidates.dedup();
            ReverseSubscriptionQuery {
                candidates,
                bucket_probes,
            }
        };
        self.observation
            .telemetry
            .invalidation
            .reverse_subscription_bucket_probes += result.bucket_probes;
        self.observation
            .telemetry
            .invalidation
            .reverse_subscription_candidates_returned += result.candidates.len() as u64;
        result
            .candidates
            .retain(|candidate| self.is_alive(*candidate));
        Ok(result)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct IndexedSubscriptionMembership {
    key: ProducerAspectKey,
    pub(crate) scope: IndexedSubscriptionScope,
}

impl IndexedSubscriptionMembership {
    pub(crate) fn from_edge(
        producer: NodeId,
        aspect: Aspect,
        scope: Option<InternedPartitionSubscription>,
    ) -> Option<Self> {
        let scope = match scope {
            None => IndexedSubscriptionScope::Unscoped,
            Some(scope) if scope.match_mode == PartitionMatchMode::WholePartition => {
                IndexedSubscriptionScope::WholePartition(scope.partition)
            }
            Some(scope) => IndexedSubscriptionScope::Detail(scope.partition, scope.detail?),
        };
        Some(Self {
            key: ProducerAspectKey::from_authoritative_edge(producer, aspect),
            scope,
        })
    }
}

#[derive(Clone, Debug, Default)]
struct SubscriberScopeBuckets {
    all: BTreeSet<NodeId>,
    unscoped: BTreeSet<NodeId>,
    whole_partitions: BTreeMap<PartitionTokenId, BTreeSet<NodeId>>,
    exact_details: BTreeMap<DetailScopeKey, BTreeSet<NodeId>>,
    partition_scoped: BTreeMap<PartitionTokenId, BTreeSet<NodeId>>,
}

#[derive(Clone, Debug)]
pub(crate) struct ReverseSubscriptionIndex {
    buckets: BTreeMap<ProducerAspectKey, SubscriberScopeBuckets>,
    by_consumer: BTreeMap<NodeId, Vec<IndexedSubscriptionMembership>>,
    valid: bool,
}

impl Default for ReverseSubscriptionIndex {
    fn default() -> Self {
        Self {
            buckets: BTreeMap::new(),
            by_consumer: BTreeMap::new(),
            valid: true,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct ReverseSubscriptionQuery {
    pub(crate) candidates: Vec<NodeId>,
    pub(crate) bucket_probes: u64,
}

impl ReverseSubscriptionIndex {
    pub(super) fn mark_rebuilt(&mut self) {
        self.valid = true;
    }

    pub(crate) fn is_valid(&self) -> bool {
        self.valid
    }

    pub(super) fn clear(&mut self) {
        self.buckets.clear();
        self.by_consumer.clear();
        self.valid = false;
    }

    pub(super) fn replace_consumer(
        &mut self,
        consumer: NodeId,
        memberships: Vec<IndexedSubscriptionMembership>,
    ) {
        self.remove_consumer(consumer);
        for membership in &memberships {
            self.insert_membership(consumer, membership);
        }
        if !memberships.is_empty() {
            self.by_consumer.insert(consumer, memberships);
        }
    }

    #[cfg(test)]
    pub(super) fn inject_candidate_drift_for_test(
        &mut self,
        producer: NodeId,
        aspect: Aspect,
        consumer: NodeId,
    ) {
        let membership = IndexedSubscriptionMembership::from_edge(producer, aspect, None)
            .expect("unscoped test membership must be indexable");
        self.insert_membership(consumer, &membership);
    }

    pub(crate) fn query_whole_aspect(
        &self,
        producer: NodeId,
        aspect: Aspect,
    ) -> ReverseSubscriptionQuery {
        let key = ProducerAspectKey::from_committed_output(producer, aspect);
        let Some(buckets) = self.buckets.get(&key) else {
            return ReverseSubscriptionQuery {
                candidates: Vec::new(),
                bucket_probes: 1,
            };
        };
        ReverseSubscriptionQuery {
            candidates: buckets.all.iter().copied().collect(),
            bucket_probes: 1,
        }
    }

    fn query_unscoped(&self, producer: NodeId, aspect: Aspect) -> ReverseSubscriptionQuery {
        let key = ProducerAspectKey::from_committed_output(producer, aspect);
        let Some(buckets) = self.buckets.get(&key) else {
            return ReverseSubscriptionQuery {
                candidates: Vec::new(),
                bucket_probes: 1,
            };
        };
        ReverseSubscriptionQuery {
            candidates: buckets.unscoped.iter().copied().collect(),
            bucket_probes: 1,
        }
    }

    pub(crate) fn query_scope(
        &self,
        producer: NodeId,
        aspect: Aspect,
        scope: InternedPartitionSubscription,
    ) -> ReverseSubscriptionQuery {
        let key = ProducerAspectKey::from_committed_output(producer, aspect);
        let Some(buckets) = self.buckets.get(&key) else {
            return ReverseSubscriptionQuery {
                candidates: Vec::new(),
                bucket_probes: match scope.match_mode {
                    PartitionMatchMode::WholePartition => 2,
                    PartitionMatchMode::PartitionAndDetail => 3,
                },
            };
        };
        let mut candidates = buckets.unscoped.iter().copied().collect::<Vec<_>>();
        let bucket_probes = match scope.match_mode {
            PartitionMatchMode::WholePartition => {
                if let Some(scoped) = buckets.partition_scoped.get(&scope.partition) {
                    candidates.extend(scoped.iter().copied());
                }
                2
            }
            PartitionMatchMode::PartitionAndDetail => {
                if let Some(whole) = buckets.whole_partitions.get(&scope.partition) {
                    candidates.extend(whole.iter().copied());
                }
                if let Some(detail) = scope.detail.and_then(|detail| {
                    buckets.exact_details.get(&DetailScopeKey {
                        partition: scope.partition,
                        detail,
                    })
                }) {
                    candidates.extend(detail.iter().copied());
                }
                3
            }
        };
        candidates.sort_unstable();
        candidates.dedup();
        ReverseSubscriptionQuery {
            candidates,
            bucket_probes,
        }
    }

    fn insert_membership(&mut self, consumer: NodeId, membership: &IndexedSubscriptionMembership) {
        let buckets = self.buckets.entry(membership.key).or_default();
        buckets.all.insert(consumer);
        match membership.scope {
            IndexedSubscriptionScope::Unscoped => {
                buckets.unscoped.insert(consumer);
            }
            IndexedSubscriptionScope::WholePartition(partition) => {
                buckets
                    .whole_partitions
                    .entry(partition)
                    .or_default()
                    .insert(consumer);
                buckets
                    .partition_scoped
                    .entry(partition)
                    .or_default()
                    .insert(consumer);
            }
            IndexedSubscriptionScope::Detail(partition, detail) => {
                buckets
                    .exact_details
                    .entry(DetailScopeKey { partition, detail })
                    .or_default()
                    .insert(consumer);
                buckets
                    .partition_scoped
                    .entry(partition)
                    .or_default()
                    .insert(consumer);
            }
        }
    }

    fn remove_consumer(&mut self, consumer: NodeId) {
        let Some(memberships) = self.by_consumer.remove(&consumer) else {
            return;
        };
        for membership in memberships {
            let key = membership.key;
            let Some(buckets) = self.buckets.get_mut(&key) else {
                continue;
            };
            buckets.all.remove(&consumer);
            match membership.scope {
                IndexedSubscriptionScope::Unscoped => {
                    buckets.unscoped.remove(&consumer);
                }
                IndexedSubscriptionScope::WholePartition(partition) => {
                    remove_member(&mut buckets.whole_partitions, partition, consumer);
                    refresh_partition_scoped(buckets, partition, consumer);
                }
                IndexedSubscriptionScope::Detail(partition, detail) => {
                    remove_member(
                        &mut buckets.exact_details,
                        DetailScopeKey { partition, detail },
                        consumer,
                    );
                    refresh_partition_scoped(buckets, partition, consumer);
                }
            }
            if buckets.all.is_empty() {
                self.buckets.remove(&key);
            }
        }
    }
}

fn refresh_partition_scoped(
    buckets: &mut SubscriberScopeBuckets,
    partition: PartitionTokenId,
    consumer: NodeId,
) {
    let remains = buckets
        .whole_partitions
        .get(&partition)
        .is_some_and(|members| members.contains(&consumer))
        || buckets.exact_details.iter().any(|(candidate, members)| {
            candidate.partition == partition && members.contains(&consumer)
        });
    if !remains {
        remove_member(&mut buckets.partition_scoped, partition, consumer);
    }
}
