use std::collections::BTreeSet;

use crate::identity::data::RecordId;
use crate::publication::data::diff::AspectKey;
use crate::storage::logic::state::{
    partition_of, slot_of, EntityRecordKind, RecordKind, RelationRecordKind,
};
use crate::symbols::data::{InternedString, StringInterner};

use crate::logic::runtime::WorkingState;

pub(super) fn aspect_keys_for_payload(
    payload: Option<&crate::payloads::data::RecordPayload>,
    _symbols: &mut StringInterner,
) -> Vec<AspectKey> {
    aspect_names_for_payload(payload)
        .into_iter()
        .map(|name| AspectKey(InternedString::Raw(name)))
        .collect()
}

pub(super) fn aspect_names_for_payload(
    payload: Option<&crate::payloads::data::RecordPayload>,
) -> Vec<String> {
    let mut aspects = BTreeSet::new();
    match payload {
        Some(crate::payloads::data::RecordPayload::StructuredJson(value)) => {
            if let Some(object) = value.as_object() {
                for key in object.keys() {
                    aspects.insert(key.clone());
                }
            }
        }
        Some(crate::payloads::data::RecordPayload::OpaqueBytes(_)) => {
            aspects.insert("opaque_payload".to_string());
        }
        None => {}
    }
    aspects.into_iter().collect()
}

pub(super) fn write_entity_aspect_versions(
    staged: &mut WorkingState,
    entity_id: crate::identity::data::EntityId,
    version_id: crate::identity::data::VersionId,
    payload: &crate::payloads::data::RecordPayload,
    symbols: &mut StringInterner,
) {
    write_aspect_versions::<EntityRecordKind>(
        staged,
        entity_id,
        version_id,
        Some(payload),
        symbols,
    );
}

pub(super) fn write_relation_aspect_versions(
    staged: &mut WorkingState,
    relation_id: crate::identity::data::RelationId,
    version_id: crate::identity::data::VersionId,
    payload: Option<&crate::payloads::data::RecordPayload>,
    symbols: &mut StringInterner,
) {
    write_aspect_versions::<RelationRecordKind>(staged, relation_id, version_id, payload, symbols);
}

fn write_aspect_versions<K: RecordKind>(
    staged: &mut WorkingState,
    record_id: RecordId<K::Domain>,
    version_id: crate::identity::data::VersionId,
    payload: Option<&crate::payloads::data::RecordPayload>,
    symbols: &mut StringInterner,
) {
    let slot = slot_of::<K>(&record_id);
    let partition = staged.get_partition_mut(partition_of::<K>(&record_id));
    let versions = &mut K::arena_mut(partition).aspect_versions[slot];
    for name in aspect_names_for_payload(payload) {
        versions.insert(symbols.intern(&name), version_id.0);
    }
}
