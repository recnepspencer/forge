use std::collections::BTreeMap;

use forge_foundational::facade::{AspectFieldLocator, AspectKey, AspectValue};

use crate::identity::data::{EntityId, KindId};
use crate::storage::data::{
    authoritative_aspect_value_field_comparison_key, AuthoritativeFieldComparisonKey,
};
use crate::visibility::materialization::read_records::EntityProjectionRecord;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StrategyEntityAspectReadRecord {
    entity_id: EntityId,
    kind_id: KindId,
    kind_name: String,
    scalar_aspect_values: BTreeMap<AspectKey, AspectValue>,
}

impl StrategyEntityAspectReadRecord {
    pub(crate) fn from_projection(
        record: EntityProjectionRecord<'_>,
        aspect_keys: &[AspectKey],
    ) -> Self {
        let scalar_aspect_values = aspect_keys
            .iter()
            .filter_map(|aspect_key| {
                record
                    .aspect_value(aspect_key)
                    .cloned()
                    .map(|value| (aspect_key.clone(), value))
            })
            .collect();
        Self {
            entity_id: record.entity_id(),
            kind_id: record.kind_id(),
            kind_name: record.kind_name().to_owned(),
            scalar_aspect_values,
        }
    }

    pub fn entity_id(&self) -> EntityId {
        self.entity_id
    }

    pub fn kind_id(&self) -> KindId {
        self.kind_id
    }

    pub fn kind_name(&self) -> &str {
        &self.kind_name
    }

    pub fn scalar_aspect_value(&self, aspect_key: &AspectKey) -> Option<&AspectValue> {
        self.scalar_aspect_values.get(aspect_key)
    }

    pub fn scalar_field_comparison_key(
        &self,
        field_locator: &AspectFieldLocator,
    ) -> Option<AuthoritativeFieldComparisonKey> {
        self.scalar_aspect_value(field_locator.aspect().aspect_key())
            .map(authoritative_aspect_value_field_comparison_key)
    }
}
