use std::collections::{BTreeMap, BTreeSet};
use std::ops::Bound::{Excluded, Unbounded};
use std::sync::Arc;

use crate::data::handle::NodeId;
use crate::data::output::PartitionTokenId;

use super::{
    DetailScopeKey, ForkConsumerMemberships, IndexedSubscriptionMembership,
    IndexedSubscriptionScope, ProducerAspectKey, SubscriberScopeBuckets,
};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(super) struct ReverseSubscriptionFlat {
    pub(super) buckets: BTreeMap<ProducerAspectKey, SubscriberScopeBuckets>,
    pub(super) by_consumer: BTreeMap<NodeId, Vec<IndexedSubscriptionMembership>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) enum ReverseSubscriptionStorage {
    Exclusive(ReverseSubscriptionFlat),
    ForkShared {
        base: Arc<ReverseSubscriptionFlat>,
        bucket_changes: im::OrdMap<ProducerAspectKey, BucketDelta>,
        consumer_changes: im::OrdMap<NodeId, Option<ForkConsumerMemberships>>,
    },
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(super) struct SetDelta {
    added: im::OrdSet<NodeId>,
    removed: im::OrdSet<NodeId>,
    retired_base_intervals: im::OrdMap<NodeId, NodeId>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(super) struct SetMergeTraversal {
    pub(super) base_members: usize,
    pub(super) range_seeks: usize,
}

impl SetDelta {
    fn contains(&self, base_contains: bool, node: &NodeId) -> bool {
        self.added.contains(node) || (base_contains && !self.removed.contains(node))
    }

    fn insert(&mut self, base: Option<&BTreeSet<NodeId>>, node: NodeId) {
        let base_contains = base.is_some_and(|base| base.contains(&node));
        if self.contains(base_contains, &node) {
            return;
        }
        if self.removed.remove(&node).is_some() {
            record_base_readmission(
                base.expect("retired member must belong to the immutable base"),
                &mut self.retired_base_intervals,
                node,
            );
        } else {
            self.added.insert(node);
        }
    }

    fn remove(&mut self, base: Option<&BTreeSet<NodeId>>, node: NodeId) {
        let base_contains = base.is_some_and(|base| base.contains(&node));
        if !self.contains(base_contains, &node) {
            return;
        }
        if self.added.remove(&node).is_none() {
            self.removed.insert(node);
            record_base_retirement(
                base.expect("removed inherited member must belong to the immutable base"),
                &mut self.retired_base_intervals,
                node,
            );
        }
    }

    fn is_empty(&self) -> bool {
        self.added.is_empty() && self.removed.is_empty() && self.retired_base_intervals.is_empty()
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(super) struct BucketDelta {
    pub(super) all: SetDelta,
    pub(super) unscoped: SetDelta,
    pub(super) whole_partitions: im::OrdMap<PartitionTokenId, SetDelta>,
    pub(super) exact_details: im::OrdMap<DetailScopeKey, SetDelta>,
    pub(super) partition_scoped: im::OrdMap<PartitionTokenId, SetDelta>,
}

impl BucketDelta {
    pub(super) fn insert(
        &mut self,
        base: Option<&SubscriberScopeBuckets>,
        consumer: NodeId,
        scope: &IndexedSubscriptionScope,
    ) {
        self.all.insert(base.map(|b| &b.all), consumer);
        match *scope {
            IndexedSubscriptionScope::Unscoped => {
                self.unscoped.insert(base.map(|b| &b.unscoped), consumer)
            }
            IndexedSubscriptionScope::WholePartition(partition) => {
                insert_map_member(
                    &mut self.whole_partitions,
                    base.and_then(|b| b.whole_partitions.get(&partition)),
                    partition,
                    consumer,
                );
                insert_map_member(
                    &mut self.partition_scoped,
                    base.and_then(|b| b.partition_scoped.get(&partition)),
                    partition,
                    consumer,
                );
            }
            IndexedSubscriptionScope::Detail(partition, detail) => {
                let key = DetailScopeKey { partition, detail };
                insert_map_member(
                    &mut self.exact_details,
                    base.and_then(|b| b.exact_details.get(&key)),
                    key,
                    consumer,
                );
                insert_map_member(
                    &mut self.partition_scoped,
                    base.and_then(|b| b.partition_scoped.get(&partition)),
                    partition,
                    consumer,
                );
            }
        }
    }

    pub(super) fn remove(
        &mut self,
        base: Option<&SubscriberScopeBuckets>,
        consumer: NodeId,
        scope: &IndexedSubscriptionScope,
    ) {
        self.all.remove(base.map(|b| &b.all), consumer);
        match *scope {
            IndexedSubscriptionScope::Unscoped => {
                self.unscoped.remove(base.map(|b| &b.unscoped), consumer)
            }
            IndexedSubscriptionScope::WholePartition(partition) => {
                remove_map_member(
                    &mut self.whole_partitions,
                    base.and_then(|b| b.whole_partitions.get(&partition)),
                    partition,
                    consumer,
                );
                self.refresh_partition_scoped(base, partition, consumer);
            }
            IndexedSubscriptionScope::Detail(partition, detail) => {
                let key = DetailScopeKey { partition, detail };
                remove_map_member(
                    &mut self.exact_details,
                    base.and_then(|b| b.exact_details.get(&key)),
                    key,
                    consumer,
                );
                self.refresh_partition_scoped(base, partition, consumer);
            }
        }
    }

    pub(super) fn is_empty(&self) -> bool {
        self.all.is_empty()
            && self.unscoped.is_empty()
            && self.whole_partitions.is_empty()
            && self.exact_details.is_empty()
            && self.partition_scoped.is_empty()
    }

    fn refresh_partition_scoped(
        &mut self,
        base: Option<&SubscriberScopeBuckets>,
        partition: PartitionTokenId,
        consumer: NodeId,
    ) {
        let remains = map_contains(
            base.and_then(|b| b.whole_partitions.get(&partition)),
            self.whole_partitions.get(&partition),
            &consumer,
        ) || merged_map_any(
            base.map(|b| &b.exact_details),
            &self.exact_details,
            |key| key.partition == partition,
            &consumer,
        );
        if !remains {
            remove_map_member(
                &mut self.partition_scoped,
                base.and_then(|b| b.partition_scoped.get(&partition)),
                partition,
                consumer,
            );
        }
    }
}

pub(super) fn extend_merged_set(
    base: Option<&BTreeSet<NodeId>>,
    delta: Option<&SetDelta>,
    target: &mut Vec<NodeId>,
) -> SetMergeTraversal {
    let mut traversal = SetMergeTraversal::default();
    if let Some(base) = base {
        extend_live_base(base, delta, target, &mut traversal);
    }
    if let Some(delta) = delta {
        target.extend(delta.added.iter().copied());
    }
    traversal
}

fn insert_map_member<K: Copy + Ord>(
    changes: &mut im::OrdMap<K, SetDelta>,
    base: Option<&BTreeSet<NodeId>>,
    key: K,
    consumer: NodeId,
) {
    let delta = changes.entry(key).or_default();
    delta.insert(base, consumer);
    if delta.is_empty() {
        changes.remove(&key);
    }
}

fn remove_map_member<K: Copy + Ord>(
    changes: &mut im::OrdMap<K, SetDelta>,
    base: Option<&BTreeSet<NodeId>>,
    key: K,
    consumer: NodeId,
) {
    let delta = changes.entry(key).or_default();
    delta.remove(base, consumer);
    if delta.is_empty() {
        changes.remove(&key);
    }
}

fn extend_live_base(
    base: &BTreeSet<NodeId>,
    delta: Option<&SetDelta>,
    target: &mut Vec<NodeId>,
    traversal: &mut SetMergeTraversal,
) {
    let Some(delta) = delta.filter(|delta| !delta.retired_base_intervals.is_empty()) else {
        traversal.base_members += base.len();
        target.extend(base.iter().copied());
        return;
    };

    let mut prior_retired_end = None;
    for (retired_start, retired_end) in &delta.retired_base_intervals {
        extend_base_run(
            base,
            prior_retired_end,
            Some(*retired_start),
            target,
            traversal,
        );
        prior_retired_end = Some(*retired_end);
    }
    extend_base_run(base, prior_retired_end, None, target, traversal);
}

fn extend_base_run(
    base: &BTreeSet<NodeId>,
    after: Option<NodeId>,
    before: Option<NodeId>,
    target: &mut Vec<NodeId>,
    traversal: &mut SetMergeTraversal,
) {
    let range = match (after, before) {
        (Some(after), Some(before)) => base.range((Excluded(after), Excluded(before))),
        (Some(after), None) => base.range((Excluded(after), Unbounded)),
        (None, Some(before)) => base.range((Unbounded, Excluded(before))),
        (None, None) => unreachable!("retired traversal always has a boundary"),
    };
    traversal.range_seeks += 1;
    for member in range {
        traversal.base_members += 1;
        target.push(*member);
    }
}

fn record_base_retirement(
    base: &BTreeSet<NodeId>,
    intervals: &mut im::OrdMap<NodeId, NodeId>,
    node: NodeId,
) {
    let left_start = base
        .range(..node)
        .next_back()
        .and_then(|predecessor| containing_interval(intervals, *predecessor))
        .map(|(start, _)| start);
    let right_start = base
        .range((Excluded(node), Unbounded))
        .next()
        .and_then(|successor| intervals.get(successor).map(|_| *successor));
    let start = left_start.unwrap_or(node);
    let end = right_start
        .and_then(|right| intervals.get(&right).copied())
        .unwrap_or(node);
    if let Some(left) = left_start {
        intervals.remove(&left);
    }
    if let Some(right) = right_start {
        intervals.remove(&right);
    }
    intervals.insert(start, end);
}

fn record_base_readmission(
    base: &BTreeSet<NodeId>,
    intervals: &mut im::OrdMap<NodeId, NodeId>,
    node: NodeId,
) {
    let Some((start, end)) = containing_interval(intervals, node) else {
        return;
    };
    intervals.remove(&start);
    if start < node {
        let predecessor = *base
            .range(..node)
            .next_back()
            .expect("non-start interval member has a predecessor");
        intervals.insert(start, predecessor);
    }
    if node < end {
        let successor = *base
            .range((Excluded(node), Unbounded))
            .next()
            .expect("non-end interval member has a successor");
        intervals.insert(successor, end);
    }
}

fn containing_interval(
    intervals: &im::OrdMap<NodeId, NodeId>,
    node: NodeId,
) -> Option<(NodeId, NodeId)> {
    intervals.get(&node).map(|end| (node, *end)).or_else(|| {
        intervals
            .get_prev(&node)
            .filter(|(_, end)| **end >= node)
            .map(|(start, end)| (*start, *end))
    })
}

fn map_contains(
    base: Option<&BTreeSet<NodeId>>,
    delta: Option<&SetDelta>,
    consumer: &NodeId,
) -> bool {
    delta.map_or_else(
        || base.is_some_and(|set| set.contains(consumer)),
        |delta| delta.contains(base.is_some_and(|set| set.contains(consumer)), consumer),
    )
}

fn merged_map_any<K: Copy + Ord>(
    base: Option<&BTreeMap<K, BTreeSet<NodeId>>>,
    changes: &im::OrdMap<K, SetDelta>,
    mut key_matches: impl FnMut(K) -> bool,
    consumer: &NodeId,
) -> bool {
    base.into_iter()
        .flatten()
        .any(|(key, set)| key_matches(*key) && map_contains(Some(set), changes.get(key), consumer))
        || changes.iter().any(|(key, delta)| {
            base.is_none_or(|base| !base.contains_key(key))
                && key_matches(*key)
                && delta.contains(false, consumer)
        })
}
