use crate::publication::patch::data::{AspectKey, CanonicalAspectSet};
use forge_foundational::facade::{AspectValue, FieldKey, StructAspectValue};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct PublishedAuthoritativePatch {
    pub operations: Vec<PublishedAuthoritativePatchOperation>,
}

impl PublishedAuthoritativePatch {
    pub fn empty() -> Self {
        Self::default()
    }

    pub fn new(operations: Vec<PublishedAuthoritativePatchOperation>) -> Self {
        Self { operations }.canonicalized()
    }

    pub fn canonicalized(&self) -> Self {
        let mut operations = self.operations.clone();
        operations.sort_unstable_by(published_operation_order);
        Self { operations }
    }

    pub fn changed_aspects(&self) -> CanonicalAspectSet {
        CanonicalAspectSet::new(self.changed_aspect_keys().cloned())
    }

    pub fn changed_aspect_keys(&self) -> impl Iterator<Item = &AspectKey> {
        self.operations
            .iter()
            .map(PublishedAuthoritativePatchOperation::aspect_key)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PublishedAuthoritativePatchOperation {
    WholeAspectSet {
        aspect_key: AspectKey,
        value: PublishedAuthoritativePatchValue,
    },
    WholeAspectClear {
        aspect_key: AspectKey,
    },
    FieldLevelPatch {
        aspect_key: AspectKey,
        field_sets: Vec<PublishedAuthoritativeFieldSet>,
        field_clears: Vec<FieldKey>,
    },
}

impl PublishedAuthoritativePatchOperation {
    pub fn aspect_key(&self) -> &AspectKey {
        match self {
            Self::WholeAspectSet { aspect_key, .. }
            | Self::WholeAspectClear { aspect_key }
            | Self::FieldLevelPatch { aspect_key, .. } => aspect_key,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PublishedAuthoritativePatchValue {
    Scalar(AspectValue),
    Struct(StructAspectValue),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct PublishedAuthoritativeFieldSet {
    pub field: FieldKey,
    pub value: AspectValue,
}

fn published_operation_order(
    left: &PublishedAuthoritativePatchOperation,
    right: &PublishedAuthoritativePatchOperation,
) -> std::cmp::Ordering {
    operation_rank(left)
        .cmp(&operation_rank(right))
        .then_with(|| left.aspect_key().cmp(right.aspect_key()))
}

fn operation_rank(operation: &PublishedAuthoritativePatchOperation) -> u8 {
    match operation {
        PublishedAuthoritativePatchOperation::WholeAspectSet { .. } => 0,
        PublishedAuthoritativePatchOperation::WholeAspectClear { .. } => 1,
        PublishedAuthoritativePatchOperation::FieldLevelPatch { .. } => 2,
    }
}
