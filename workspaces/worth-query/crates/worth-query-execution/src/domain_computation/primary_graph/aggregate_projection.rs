use std::collections::{BTreeMap, BTreeSet};

use worth_foundational::facade::{AspectFieldLocator, AspectValue};
use worth_relational::facade::identity::{EntityId, KindId, VersionId};
use worth_relational::facade::publication::PublishedAuthoritativeRecordPatch;
use worth_relational::facade::snapshots::SnapshotHandle;
use worth_relational::facade::transactions::RecordRef;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(super) struct WorthQueryIncomingSumKey {
    pub(super) relation_kind: KindId,
    pub(super) source_kind: KindId,
    pub(super) target_kind: KindId,
    pub(super) field: AspectFieldLocator,
}

#[derive(Default)]
pub(super) struct WorthQueryAggregateProjections {
    incoming_sums: BTreeMap<WorthQueryIncomingSumKey, WorthQueryIncomingSumGeneration>,
}

struct WorthQueryIncomingSumGeneration {
    version: VersionId,
    materialized: BTreeMap<EntityId, WorthQueryIncomingAggregate>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct WorthQueryIncomingAggregate {
    pub(super) sum: i64,
    pub(super) source_count: u64,
}

impl WorthQueryAggregateProjections {
    pub(super) fn cached_aggregate(
        &mut self,
        key: &WorthQueryIncomingSumKey,
        target: EntityId,
        version: VersionId,
    ) -> Option<WorthQueryIncomingAggregate> {
        let generation = self.incoming_sums.get_mut(key)?;
        if generation.version != version {
            generation.materialized.clear();
            generation.version = version;
            return None;
        }
        generation.materialized.get(&target).copied()
    }

    pub(super) fn retain_aggregate(
        &mut self,
        key: WorthQueryIncomingSumKey,
        target: EntityId,
        version: VersionId,
        sum: i64,
        source_count: u64,
    ) {
        let generation =
            self.incoming_sums
                .entry(key)
                .or_insert_with(|| WorthQueryIncomingSumGeneration {
                    version,
                    materialized: BTreeMap::new(),
                });
        if generation.version != version {
            generation.materialized.clear();
            generation.version = version;
        }
        generation
            .materialized
            .insert(target, WorthQueryIncomingAggregate { sum, source_count });
    }

    pub(super) fn refresh_after_commit(
        &mut self,
        runtime: &mut worth_relational::facade::runtime::RelationalRuntime,
        before: &SnapshotHandle,
        after: &SnapshotHandle,
        patches: &[PublishedAuthoritativeRecordPatch],
    ) {
        for (key, generation) in &mut self.incoming_sums {
            if generation.version != before.version_id() {
                generation.materialized.clear();
                generation.version = after.version_id();
                continue;
            }
            let sources = affected_sources(runtime, key, before, after, patches);
            for source in sources {
                let old = source_contributions(runtime, key, before, source);
                let new = source_contributions(runtime, key, after, source);
                let (Ok(old), Ok(new)) = (old, new) else {
                    generation.materialized.clear();
                    break;
                };
                update_materialized_targets(&mut generation.materialized, old, new);
            }
            generation.version = after.version_id();
        }
    }
}

fn affected_sources(
    runtime: &worth_relational::facade::runtime::RelationalRuntime,
    key: &WorthQueryIncomingSumKey,
    before: &SnapshotHandle,
    after: &SnapshotHandle,
    patches: &[PublishedAuthoritativeRecordPatch],
) -> BTreeSet<EntityId> {
    let truth = runtime.read_truth();
    let mut sources = BTreeSet::new();
    for patch in patches {
        match patch.target {
            RecordRef::Entity(entity) => {
                for version in [before.version_id(), after.version_id()] {
                    if truth
                        .visible_entity_at_version(entity, version)
                        .is_some_and(|record| record.kind.kind_id == key.source_kind)
                    {
                        sources.insert(entity);
                    }
                }
            }
            RecordRef::Relation(relation) => {
                for version in [before.version_id(), after.version_id()] {
                    if let Some(record) = truth.visible_relation_at_version(relation, version) {
                        if record.kind.kind_id == key.relation_kind {
                            sources.insert(record.source);
                        }
                    }
                }
            }
        }
    }
    sources
}

fn source_contributions(
    runtime: &mut worth_relational::facade::runtime::RelationalRuntime,
    key: &WorthQueryIncomingSumKey,
    snapshot: &SnapshotHandle,
    source: EntityId,
) -> Result<BTreeMap<EntityId, i64>, ()> {
    let Some(record) = runtime
        .read_truth()
        .visible_entity_at_version(source, snapshot.version_id())
    else {
        return Ok(BTreeMap::new());
    };
    if record.kind.kind_id != key.source_kind {
        return Ok(BTreeMap::new());
    }
    let Some(AspectValue::Int64(value)) = super::application_attempt::observe_field_value(
        runtime,
        snapshot,
        source,
        key.source_kind,
        &key.field,
    ) else {
        return Err(());
    };
    let relations = runtime.read_truth().outgoing_relations_of_kind_at_version(
        source,
        key.relation_kind,
        snapshot.version_id(),
    );
    if relations.len() > 1 {
        return Err(());
    }
    let mut contributions = BTreeMap::new();
    for relation in relations {
        if runtime
            .read_truth()
            .visible_entity_at_version(relation.target, snapshot.version_id())
            .is_some_and(|target| target.kind.kind_id == key.target_kind)
        {
            let entry = contributions.entry(relation.target).or_insert(0_i64);
            *entry = entry.checked_add(value).ok_or(())?;
        }
    }
    Ok(contributions)
}

fn update_materialized_targets(
    materialized: &mut BTreeMap<EntityId, WorthQueryIncomingAggregate>,
    old: BTreeMap<EntityId, i64>,
    new: BTreeMap<EntityId, i64>,
) {
    let targets = old
        .keys()
        .chain(new.keys())
        .copied()
        .collect::<BTreeSet<_>>();
    for target in targets {
        let Some(current) = materialized.get(&target).copied() else {
            continue;
        };
        let sum = current
            .sum
            .checked_sub(old.get(&target).copied().unwrap_or_default())
            .and_then(|value| value.checked_add(new.get(&target).copied().unwrap_or_default()));
        let source_count = current
            .source_count
            .checked_sub(u64::from(old.contains_key(&target)))
            .and_then(|count| count.checked_add(u64::from(new.contains_key(&target))));
        match (sum, source_count) {
            (Some(sum), Some(source_count)) => {
                materialized.insert(target, WorthQueryIncomingAggregate { sum, source_count });
            }
            _ => {
                materialized.remove(&target);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{update_materialized_targets, WorthQueryIncomingAggregate};
    use std::collections::BTreeMap;
    use worth_relational::facade::identity::{EntityId, PartitionId};

    #[test]
    fn incremental_overflow_evicts_the_reconstructible_target() {
        let target = EntityId::new(PartitionId::new(1), 1, 1);
        let mut materialized = BTreeMap::from([(
            target,
            WorthQueryIncomingAggregate {
                sum: i64::MAX,
                source_count: 1,
            },
        )]);

        update_materialized_targets(
            &mut materialized,
            BTreeMap::new(),
            BTreeMap::from([(target, 1)]),
        );

        assert!(!materialized.contains_key(&target));
    }

    #[test]
    fn incremental_retargeting_updates_sum_and_source_count_together() {
        let old_target = EntityId::new(PartitionId::new(1), 1, 1);
        let new_target = EntityId::new(PartitionId::new(1), 2, 1);
        let mut materialized = BTreeMap::from([
            (
                old_target,
                WorthQueryIncomingAggregate {
                    sum: 7,
                    source_count: 1,
                },
            ),
            (
                new_target,
                WorthQueryIncomingAggregate {
                    sum: 3,
                    source_count: 1,
                },
            ),
        ]);

        update_materialized_targets(
            &mut materialized,
            BTreeMap::from([(old_target, 7)]),
            BTreeMap::from([(new_target, 7)]),
        );

        assert_eq!(
            materialized.get(&old_target),
            Some(&WorthQueryIncomingAggregate {
                sum: 0,
                source_count: 0,
            })
        );
        assert_eq!(
            materialized.get(&new_target),
            Some(&WorthQueryIncomingAggregate {
                sum: 10,
                source_count: 2,
            })
        );
    }
}
