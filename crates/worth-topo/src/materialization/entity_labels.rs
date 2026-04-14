use forge_relational::facade::payloads::RecordPayload;
use forge_relational::facade::runtime::EntityReadRecord;

pub fn entity_label(record: &EntityReadRecord) -> String {
    record_label(&record.payload).unwrap_or_else(|| record.kind.kind_name.clone())
}

fn record_label(payload: &RecordPayload) -> Option<String> {
    payload
        .as_json()
        .and_then(|json| json.get("label"))
        .and_then(|value| value.as_str())
        .map(str::to_string)
}
