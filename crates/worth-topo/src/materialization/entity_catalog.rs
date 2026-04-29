use std::collections::BTreeMap;

use forge_relational::facade::identity::EntityId;
use forge_relational::facade::runtime::EntityReadRecord;
use worth_schema::facade::WorthEntityKind;

pub fn collect_entity_kinds(entities: &[EntityReadRecord]) -> BTreeMap<EntityId, WorthEntityKind> {
    entities
        .iter()
        .filter_map(|record| {
            WorthEntityKind::from_kind_id(record.kind.kind_id).map(|kind| (record.entity_id, kind))
        })
        .collect()
}
