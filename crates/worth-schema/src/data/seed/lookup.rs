use forge_relational::facade::identity::EntityId;
use forge_relational::facade::runtime::RelationalReadView;

use crate::data::authority::aspect_field_patches::entity_record_label;
use crate::data::entities::EntityKind;

pub fn find_seeded_entity(
    read_view: &RelationalReadView,
    kind: EntityKind,
    label: &str,
) -> EntityId {
    read_view
        .entities()
        .iter()
        .find(|record| {
            record.kind.kind_id == kind.kind_id()
                && entity_record_label(record, kind) == Some(label)
        })
        .map(|record| record.entity_id)
        .expect(" bootstrap entity should be visible in seeded snapshot")
}
