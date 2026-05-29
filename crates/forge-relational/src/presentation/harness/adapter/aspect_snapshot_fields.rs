use std::collections::BTreeMap;

use forge_foundational::facade::{
    AspectValue, AuthoritativeRecordAspectState, ContractValidatedAspectValueView,
    StructAspectValue,
};
use serde_json::{Map, Value};

use crate::identity::data::{EntityId, RelationId};
use crate::storage::data::{EntityReadRecord, RecordLifecycleState, RelationReadRecord};

pub(super) fn entity_snapshot_fields(record: &EntityReadRecord) -> BTreeMap<String, Value> {
    let mut fields = record_identity_fields("entity", entity_identity(record.entity_id));
    fields.insert(
        "kind_id".to_string(),
        Value::from(record.kind.kind_id.0 as u64),
    );
    fields.insert(
        "lifecycle".to_string(),
        Value::String(lifecycle_label(record.lifecycle).to_string()),
    );
    fields.insert(
        "created_at_version".to_string(),
        Value::from(record.created_at_version.0),
    );
    fields.insert(
        "retired_at_version".to_string(),
        optional_u64(record.retired_at_version.map(|version| version.0)),
    );
    fields.insert(
        "authoritative_aspects".to_string(),
        authoritative_aspect_state_value(record.authoritative_aspect_state.as_ref()),
    );
    fields.insert(
        "authoritative_field_comparison_keys".to_string(),
        comparison_keys_value(
            record
                .authoritative_field_key_comparison_keys
                .iter()
                .map(|(field, key)| (field.as_str(), key.display_value())),
        ),
    );
    fields
}

pub(super) fn relation_snapshot_fields(record: &RelationReadRecord) -> BTreeMap<String, Value> {
    let mut fields = record_identity_fields("relation", relation_identity(record.relation_id));
    fields.insert(
        "kind_id".to_string(),
        Value::from(record.kind.kind_id.0 as u64),
    );
    fields.insert(
        "source".to_string(),
        Value::String(entity_identity(record.source)),
    );
    fields.insert(
        "target".to_string(),
        Value::String(entity_identity(record.target)),
    );
    fields.insert(
        "lifecycle".to_string(),
        Value::String(lifecycle_label(record.lifecycle).to_string()),
    );
    fields.insert(
        "created_at_version".to_string(),
        Value::from(record.created_at_version.0),
    );
    fields.insert(
        "retired_at_version".to_string(),
        optional_u64(record.retired_at_version.map(|version| version.0)),
    );
    fields.insert(
        "authoritative_aspects".to_string(),
        authoritative_aspect_state_value(record.authoritative_aspect_state.as_ref()),
    );
    fields.insert(
        "authoritative_field_comparison_keys".to_string(),
        comparison_keys_value(
            record
                .authoritative_field_key_comparison_keys
                .iter()
                .map(|(field, key)| (field.as_str(), key.display_value())),
        ),
    );
    fields
}

fn record_identity_fields(record_kind: &str, identity: String) -> BTreeMap<String, Value> {
    BTreeMap::from([
        (
            "record_kind".to_string(),
            Value::String(record_kind.to_string()),
        ),
        ("identity".to_string(), Value::String(identity)),
    ])
}

fn authoritative_aspect_state_value(state: Option<&AuthoritativeRecordAspectState>) -> Value {
    let Some(state) = state else {
        return Value::Array(Vec::new());
    };
    Value::Array(
        state
            .aspects()
            .entries()
            .map(|(aspect_key, value)| {
                Value::Object(Map::from_iter([
                    (
                        "aspect_key".to_string(),
                        Value::String(aspect_key.as_str().to_string()),
                    ),
                    (
                        "value_family".to_string(),
                        Value::String(validated_value_family(value.view())),
                    ),
                    (
                        "canonical_value_bytes".to_string(),
                        canonical_value_bytes(value.view()),
                    ),
                ]))
            })
            .collect(),
    )
}

fn validated_value_family(value: ContractValidatedAspectValueView<'_>) -> String {
    match value {
        ContractValidatedAspectValueView::Scalar(scalar) => {
            format!("{:?}", scalar.value_family())
        }
        ContractValidatedAspectValueView::Struct(_) => "Struct".to_string(),
    }
}

fn canonical_value_bytes(value: ContractValidatedAspectValueView<'_>) -> Value {
    match value {
        ContractValidatedAspectValueView::Scalar(scalar) => aspect_value_bytes(scalar),
        ContractValidatedAspectValueView::Struct(struct_value) => {
            struct_aspect_value_bytes(struct_value)
        }
    }
}

fn aspect_value_bytes(value: &AspectValue) -> Value {
    crate::aspect_wire::encode_aspect_value(value)
        .map(byte_array_value)
        .unwrap_or(Value::Null)
}

fn struct_aspect_value_bytes(value: &StructAspectValue) -> Value {
    let fields = value
        .fields()
        .map(|(field, scalar)| {
            Value::Object(Map::from_iter([
                (
                    "field".to_string(),
                    Value::String(field.as_str().to_string()),
                ),
                (
                    "value_family".to_string(),
                    Value::String(format!("{:?}", scalar.value_family())),
                ),
                (
                    "canonical_value_bytes".to_string(),
                    aspect_value_bytes(scalar),
                ),
            ]))
        })
        .collect::<Vec<_>>();
    Value::Array(fields)
}

fn comparison_keys_value<'a>(keys: impl Iterator<Item = (&'a str, &'a str)>) -> Value {
    Value::Object(Map::from_iter(keys.map(|(field, key)| {
        (field.to_string(), Value::String(key.to_string()))
    })))
}

fn byte_array_value(bytes: Vec<u8>) -> Value {
    Value::Array(
        bytes
            .into_iter()
            .map(|byte| Value::from(byte as u64))
            .collect(),
    )
}

fn optional_u64(value: Option<u64>) -> Value {
    value.map(Value::from).unwrap_or(Value::Null)
}

fn lifecycle_label(lifecycle: RecordLifecycleState) -> &'static str {
    match lifecycle {
        RecordLifecycleState::Live => "live",
        RecordLifecycleState::DeletedRetained => "deleted_retained",
        RecordLifecycleState::RetainedDanglingForAudit => "retained_dangling_for_audit",
        RecordLifecycleState::PinnedBySnapshot => "pinned_by_snapshot",
        RecordLifecycleState::PinnedByBranch => "pinned_by_branch",
        RecordLifecycleState::PinnedByReplayRetention => "pinned_by_replay_retention",
        RecordLifecycleState::Reclaimable => "reclaimable",
        RecordLifecycleState::Reusable => "reusable",
    }
}

fn entity_identity(entity_id: EntityId) -> String {
    format!(
        "entity:{}:{}:{}",
        entity_id.partition_id.0, entity_id.local_slot.0, entity_id.generation.0
    )
}

fn relation_identity(relation_id: RelationId) -> String {
    format!(
        "relation:{}:{}:{}",
        relation_id.partition_id.0, relation_id.local_slot.0, relation_id.generation.0
    )
}
