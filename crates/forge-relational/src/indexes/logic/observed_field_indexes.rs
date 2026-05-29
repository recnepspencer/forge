use std::collections::BTreeMap;

use forge_foundational::facade::AspectFieldLocator;

use crate::identity::data::{EntityId, RelationId};
use crate::storage::data::{
    entity_authoritative_aspect_field_comparison_key,
    relation_authoritative_aspect_field_comparison_key, AuthoritativeFieldComparisonKey,
    EntityReadRecord, RelationReadRecord,
};

pub(super) fn build_entity_aspect_field_index(
    records: &[EntityReadRecord],
    field_locator: &AspectFieldLocator,
) -> BTreeMap<AuthoritativeFieldComparisonKey, Vec<EntityId>> {
    let mut map: BTreeMap<AuthoritativeFieldComparisonKey, Vec<EntityId>> = BTreeMap::new();
    for entity in records {
        let Some(key) = entity_authoritative_aspect_field_comparison_key(entity, field_locator)
        else {
            continue;
        };
        map.entry(key).or_default().push(entity.entity_id);
    }
    map
}

pub(super) fn build_relation_aspect_field_index(
    records: &[RelationReadRecord],
    field_locator: &AspectFieldLocator,
) -> BTreeMap<AuthoritativeFieldComparisonKey, Vec<RelationId>> {
    let mut map: BTreeMap<AuthoritativeFieldComparisonKey, Vec<RelationId>> = BTreeMap::new();
    for relation in records {
        let Some(key) = relation_authoritative_aspect_field_comparison_key(relation, field_locator)
        else {
            continue;
        };
        map.entry(key).or_default().push(relation.relation_id);
    }
    map
}
