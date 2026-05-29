use std::collections::BTreeMap;

use forge_foundational::facade::FieldKey;

use crate::identity::data::{EntityId, RelationId};
use crate::storage::data::{AuthoritativeFieldComparisonKey, EntityReadRecord, RelationReadRecord};

pub(super) fn build_entity_field_index(
    records: &[EntityReadRecord],
    field: &FieldKey,
) -> BTreeMap<AuthoritativeFieldComparisonKey, Vec<EntityId>> {
    let mut map: BTreeMap<AuthoritativeFieldComparisonKey, Vec<EntityId>> = BTreeMap::new();
    for entity in records {
        let Some(key) = entity.authoritative_field_comparison_key(field) else {
            continue;
        };
        map.entry(key.clone()).or_default().push(entity.entity_id);
    }
    map
}

pub(super) fn build_relation_field_index(
    records: &[RelationReadRecord],
    field: &FieldKey,
) -> BTreeMap<AuthoritativeFieldComparisonKey, Vec<RelationId>> {
    let mut map: BTreeMap<AuthoritativeFieldComparisonKey, Vec<RelationId>> = BTreeMap::new();
    for relation in records {
        let Some(key) = relation.authoritative_field_comparison_key(field) else {
            continue;
        };
        map.entry(key.clone())
            .or_default()
            .push(relation.relation_id);
    }
    map
}
