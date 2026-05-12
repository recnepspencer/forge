use std::collections::BTreeMap;

use forge_relational::facade::identity::EntityId;
use schema::facade::EntityKind;

use crate::derived_topology::materialized_graph::input_rows::MaterializationEntityRow;

pub fn collect_entity_kinds(
    entities: &[MaterializationEntityRow],
) -> BTreeMap<EntityId, EntityKind> {
    entities
        .iter()
        .map(|record| (record.entity_id, record.kind))
        .collect()
}
