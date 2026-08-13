use std::collections::BTreeMap;

use crate::identity::data::{EntityId, KindId, RelationId};
use crate::indexes::data::{
    RelationJoinDefinition, RelationJoinEntry, RelationJoinKey, RelationJoinLeg,
    RelationJoinSharedEndpoint,
};
use crate::storage::data::RelationReadRecord;

use super::IndexProjectionSource;

pub(in crate::indexes) fn build_relation_join_index(
    projection: &IndexProjectionSource<'_, '_>,
    definition: RelationJoinDefinition,
) -> BTreeMap<RelationJoinKey, Vec<RelationJoinEntry>> {
    let left = collect_join_relations(
        projection,
        definition.left(),
        definition.shared_entity_kind(),
    );
    let right = collect_join_relations(
        projection,
        definition.right(),
        definition.shared_entity_kind(),
    );
    let mut entries = BTreeMap::<RelationJoinKey, Vec<RelationJoinEntry>>::new();
    for (shared_entity, left_relations) in left {
        let Some(right_relations) = right.get(&shared_entity) else {
            continue;
        };
        for (left_entity, left_relation) in left_relations {
            for (right_entity, right_relation) in right_relations {
                entries
                    .entry(RelationJoinKey::new(left_entity, *right_entity))
                    .or_default()
                    .push(RelationJoinEntry::new(
                        shared_entity,
                        left_relation,
                        *right_relation,
                    ));
            }
        }
    }
    for joined in entries.values_mut() {
        joined.sort();
        joined.dedup_by_key(|entry| entry.shared_entity_id());
    }
    entries
}

fn collect_join_relations(
    projection: &IndexProjectionSource<'_, '_>,
    leg: RelationJoinLeg,
    shared_entity_kind: KindId,
) -> BTreeMap<EntityId, BTreeMap<EntityId, RelationId>> {
    let mut collected = BTreeMap::<EntityId, BTreeMap<EntityId, RelationId>>::new();
    projection.for_each_relation(leg.relation_kind(), |relation| {
        let (shared, external) = join_endpoints(relation, leg.shared_endpoint());
        if projection.with_entity(shared, |record| record.kind.kind_id) != Some(shared_entity_kind)
            || projection.with_entity(external, |record| record.kind.kind_id)
                != Some(leg.external_entity_kind())
        {
            return;
        }
        collected
            .entry(shared)
            .or_default()
            .entry(external)
            .and_modify(|selected| *selected = (*selected).min(relation.relation_id))
            .or_insert(relation.relation_id);
    });
    collected
}

pub(in crate::indexes) const fn join_endpoints(
    relation: &RelationReadRecord,
    shared_endpoint: RelationJoinSharedEndpoint,
) -> (EntityId, EntityId) {
    match shared_endpoint {
        RelationJoinSharedEndpoint::Source => (relation.source, relation.target),
        RelationJoinSharedEndpoint::Target => (relation.target, relation.source),
    }
}
