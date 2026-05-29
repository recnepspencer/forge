use forge_foundational::facade::{
    AspectFieldLocator, AspectKey, CanonicalFieldPath, FieldKey, LocatorAuthority,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct UniqueEntityAspectField {
    #[serde(with = "crate::aspect_wire::serde_canonical_aspect_field_locator")]
    field_locator: AspectFieldLocator,
}

impl UniqueEntityAspectField {
    pub fn new(field_locator: AspectFieldLocator) -> Self {
        Self { field_locator }
    }

    pub fn single(aspect_key: AspectKey, field: FieldKey) -> Self {
        Self::new(AspectFieldLocator::new(
            LocatorAuthority::Planned,
            aspect_key,
            CanonicalFieldPath::single(field),
        ))
    }

    pub fn field_locator(&self) -> &AspectFieldLocator {
        &self.field_locator
    }

    pub fn single_field(&self) -> Option<&FieldKey> {
        match self.field_locator.field_path().fields() {
            [field] => Some(field),
            _ => None,
        }
    }
}
