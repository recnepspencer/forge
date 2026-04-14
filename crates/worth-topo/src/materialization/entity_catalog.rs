use std::collections::BTreeMap;

use forge_relational::facade::identity::EntityId;
use forge_relational::facade::runtime::RelationalReadView;
use worth_schema::facade::WorthEntityKind;

pub fn collect_entity_kinds(read_view: &RelationalReadView) -> BTreeMap<EntityId, WorthEntityKind> {
    read_view
        .entities()
        .iter()
        .filter_map(|record| {
            WorthEntityKind::from_kind_id(record.kind.kind_id).map(|kind| (record.entity_id, kind))
        })
        .collect()
}
