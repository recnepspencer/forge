use crate::logic::runtime::RelationalRuntime;
use crate::publication::data::diff::AspectKey;
use crate::symbols::data::InternedString;

impl RelationalRuntime {
    pub fn entity_aspect_versions(
        &self,
        entity_id: crate::identity::data::EntityId,
    ) -> Option<Vec<(AspectKey, u64)>> {
        let partition = self.partition(entity_id.partition_id)?;
        let slot = entity_id.local_slot.0 as usize;
        let versions = partition.entity_arena.aspect_versions.get(slot)?;
        Some(
            versions
                .iter()
                .filter_map(|(symbol, version)| {
                    self.symbols
                        .resolve(*symbol)
                        .map(|_| (AspectKey(InternedString::Symbol(*symbol)), *version))
                })
                .collect(),
        )
    }

    pub fn relation_aspect_versions(
        &self,
        relation_id: crate::identity::data::RelationId,
    ) -> Option<Vec<(AspectKey, u64)>> {
        let partition = self.partition(relation_id.partition_id)?;
        let slot = relation_id.local_slot.0 as usize;
        let versions = partition.relation_arena.aspect_versions.get(slot)?;
        Some(
            versions
                .iter()
                .filter_map(|(symbol, version)| {
                    self.symbols
                        .resolve(*symbol)
                        .map(|_| (AspectKey(InternedString::Symbol(*symbol)), *version))
                })
                .collect(),
        )
    }

    pub fn entity_aspects_at_version(
        &self,
        entity_id: crate::identity::data::EntityId,
        version_id: crate::identity::data::VersionId,
    ) -> Option<Vec<AspectKey>> {
        let state = self.current_state();
        let record = self.entity_record_for_id_at_version(&state, entity_id, version_id)?;
        Some(aspect_keys_for_payload(
            &record.payload,
            &mut self.symbols.clone(),
        ))
    }

    pub fn relation_aspects_at_version(
        &self,
        relation_id: crate::identity::data::RelationId,
        version_id: crate::identity::data::VersionId,
    ) -> Option<Vec<AspectKey>> {
        let state = self.current_state();
        let record = self.relation_record_for_id_at_version(&state, relation_id, version_id)?;
        record
            .payload
            .as_ref()
            .map(|payload| aspect_keys_for_payload(payload, &mut self.symbols.clone()))
    }
}

fn aspect_keys_for_payload(
    payload: &crate::payloads::data::RecordPayload,
    _symbols: &mut crate::symbols::data::StringInterner,
) -> Vec<AspectKey> {
    let mut aspects = Vec::new();
    match payload {
        crate::payloads::data::RecordPayload::StructuredJson(value) => {
            if let Some(object) = value.as_object() {
                for key in object.keys() {
                    aspects.push(AspectKey(InternedString::Raw(key.clone())));
                }
            }
        }
        crate::payloads::data::RecordPayload::OpaqueBytes(_) => {
            aspects.push(AspectKey(InternedString::Raw("opaque_payload".to_string())));
        }
    }
    aspects.sort();
    aspects.dedup();
    aspects
}
