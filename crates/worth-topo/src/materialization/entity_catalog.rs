use std::collections::BTreeMap;

use forge_relational::facade::identity::EntityId;
use worth_schema::facade::WorthEntityKind;

use crate::materialization::input_rows::MaterializationEntityRow;

pub fn collect_entity_kinds(
    entities: &[MaterializationEntityRow],
) -> BTreeMap<EntityId, WorthEntityKind> {
    entities
        .iter()
        .map(|record| (record.entity_id, record.kind))
        .collect()
}
