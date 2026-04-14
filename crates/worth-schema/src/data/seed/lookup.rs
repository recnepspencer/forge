use forge_relational::facade::identity::EntityId;
use forge_relational::facade::payloads::RecordPayload;
use forge_relational::facade::runtime::RelationalReadView;

use crate::data::entities::WorthEntityKind;

pub fn find_seeded_entity(
    read_view: &RelationalReadView,
    kind: WorthEntityKind,
    label: &str,
) -> EntityId {
    read_view
        .entities()
        .iter()
        .find(|record| {
            record.kind.kind_id == kind.kind_id() && record_label(&record.payload) == Some(label)
        })
        .map(|record| record.entity_id)
        .expect("worth bootstrap entity should be visible in seeded snapshot")
}

fn record_label(payload: &RecordPayload) -> Option<&str> {
    payload
        .as_json()
        .and_then(|json| json.get("label"))
        .and_then(|value| value.as_str())
}
