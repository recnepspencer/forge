use std::collections::HashMap;

use super::partition_scope::{PresentationSemanticDetailKey, PresentationSemanticScope};
use super::{PresentationSemanticPartition, RegisteredSemanticInstance};

#[derive(Default)]
pub(super) struct SemanticSubscriberIndex {
    next_registration: u64,
    registrations: usize,
    by_aspect: [usize; 8],
    by_scope: HashMap<PresentationSemanticScope, usize>,
    by_detail: HashMap<PresentationSemanticDetailKey, usize>,
    by_partition: HashMap<PresentationSemanticPartition, RegisteredSemanticPartition>,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthUiPresentationScopeRejectionCounters {
    aspect: u64,
    partition: u64,
    detail: u64,
    range: u64,
}

pub(super) struct SemanticSubscriberSelection {
    pub(super) instances: Vec<RegisteredSemanticInstance>,
    pub(super) rejections: WorthUiPresentationScopeRejectionCounters,
}

#[derive(Default)]
struct RegisteredSemanticPartition {
    ordered: Vec<IndexedRegisteredSemanticInstance>,
    positions: HashMap<u64, usize>,
}

struct IndexedRegisteredSemanticInstance {
    registration: u64,
    instance: RegisteredSemanticInstance,
}

impl SemanticSubscriberIndex {
    pub(super) fn issue_registration(&mut self) -> u64 {
        self.next_registration = self
            .next_registration
            .checked_add(1)
            .expect("semantic subscriber registration identity exhausted");
        self.next_registration
    }

    pub(super) fn register(
        &mut self,
        partition: PresentationSemanticPartition,
        registration: u64,
        instance: RegisteredSemanticInstance,
    ) {
        increment(&mut self.registrations);
        increment(&mut self.by_aspect[partition.aspect_ordinal()]);
        increment(self.by_scope.entry(partition.scope()).or_default());
        increment(self.by_detail.entry(partition.detail_key()).or_default());
        let registered = self.by_partition.entry(partition).or_default();
        let position = registered.ordered.len();
        assert!(
            registered
                .positions
                .insert(registration, position)
                .is_none(),
            "one semantic registration cannot enter the same partition twice",
        );
        registered.ordered.push(IndexedRegisteredSemanticInstance {
            registration,
            instance,
        });
    }

    pub(super) fn unregister(
        &mut self,
        partition: &PresentationSemanticPartition,
        registration: u64,
    ) {
        self.registrations -= 1;
        decrement(&mut self.by_aspect[partition.aspect_ordinal()]);
        decrement_key(&mut self.by_scope, &partition.scope());
        decrement_key(&mut self.by_detail, &partition.detail_key());
        let remove_partition = self
            .by_partition
            .get_mut(partition)
            .is_some_and(|registered| {
                registered.remove(registration);
                registered.ordered.is_empty()
            });
        if remove_partition {
            self.by_partition.remove(partition);
        }
    }

    pub(super) fn instances(
        &self,
        partition: &PresentationSemanticPartition,
    ) -> Option<SemanticSubscriberSelection> {
        self.by_partition.get(partition).map(|registered| {
            let instances = registered
                .ordered
                .iter()
                .map(|indexed| indexed.instance.clone())
                .collect::<Vec<_>>();
            let exact = instances.len();
            let detail = self.by_detail[&partition.detail_key()];
            let scope = self.by_scope[&partition.scope()];
            let aspect = self.by_aspect[partition.aspect_ordinal()];
            SemanticSubscriberSelection {
                instances,
                rejections: WorthUiPresentationScopeRejectionCounters {
                    aspect: u64::try_from(self.registrations - aspect)
                        .expect("semantic rejection count fits the platform counter"),
                    partition: u64::try_from(aspect - scope)
                        .expect("semantic rejection count fits the platform counter"),
                    detail: u64::try_from(scope - detail)
                        .expect("semantic rejection count fits the platform counter"),
                    range: u64::try_from(detail - exact)
                        .expect("semantic rejection count fits the platform counter"),
                },
            }
        })
    }
}

impl WorthUiPresentationScopeRejectionCounters {
    pub const fn values(self) -> [u64; 4] {
        [self.aspect, self.partition, self.detail, self.range]
    }

    pub(super) fn checked_add(self, other: Self) -> Option<Self> {
        Some(Self {
            aspect: self.aspect.checked_add(other.aspect)?,
            partition: self.partition.checked_add(other.partition)?,
            detail: self.detail.checked_add(other.detail)?,
            range: self.range.checked_add(other.range)?,
        })
    }
}

fn decrement(value: &mut usize) {
    *value = value
        .checked_sub(1)
        .expect("registered semantic subscriber count cannot underflow");
}

fn increment(value: &mut usize) {
    *value = value
        .checked_add(1)
        .expect("registered semantic subscriber count cannot overflow");
}

fn decrement_key<K: Eq + std::hash::Hash>(counts: &mut HashMap<K, usize>, key: &K) {
    let remove = counts.get_mut(key).is_some_and(|value| {
        decrement(value);
        *value == 0
    });
    if remove {
        counts.remove(key);
    }
}

impl RegisteredSemanticPartition {
    fn remove(&mut self, registration: u64) {
        let position = self
            .positions
            .remove(&registration)
            .expect("registered semantic subscriber remains indexed until retirement");
        self.ordered.swap_remove(position);
        if let Some(moved) = self.ordered.get(position) {
            self.positions.insert(moved.registration, position);
        }
    }
}
