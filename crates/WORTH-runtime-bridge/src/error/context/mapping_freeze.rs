use std::sync::Arc;

use crate::mapping::BridgeMappingId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeMappingFreezeContext {
    mapping_id: Option<BridgeMappingId>,
    conflicting_mapping_id: Option<BridgeMappingId>,
    invalid_field: Option<Arc<str>>,
}

impl BridgeMappingFreezeContext {
    pub fn for_mapping(mapping_id: BridgeMappingId) -> Self {
        Self {
            mapping_id: Some(mapping_id),
            conflicting_mapping_id: None,
            invalid_field: None,
        }
    }

    pub fn for_mapping_pair(
        mapping_id: BridgeMappingId,
        conflicting_mapping_id: BridgeMappingId,
    ) -> Self {
        Self {
            mapping_id: Some(mapping_id),
            conflicting_mapping_id: Some(conflicting_mapping_id),
            invalid_field: None,
        }
    }

    pub fn with_invalid_field(mut self, invalid_field: impl Into<Arc<str>>) -> Self {
        self.invalid_field = Some(invalid_field.into());
        self
    }

    pub fn mapping_id(&self) -> Option<&BridgeMappingId> {
        self.mapping_id.as_ref()
    }

    pub fn conflicting_mapping_id(&self) -> Option<&BridgeMappingId> {
        self.conflicting_mapping_id.as_ref()
    }

    pub fn invalid_field(&self) -> Option<&str> {
        self.invalid_field.as_deref()
    }
}
