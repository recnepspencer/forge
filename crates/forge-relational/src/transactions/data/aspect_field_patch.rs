use std::collections::BTreeMap;

use forge_foundational::facade::{
    AspectFieldLocator, AspectKey, AspectValue, CanonicalFieldPath, FieldKey, LocatorAuthority,
};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::aspect_wire::{
    decode_aspect_field_patch_canonical_bytes, encode_aspect_field_patch_canonical_bytes,
};

pub use crate::aspect_wire::AspectFieldPatchCodecError;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct AspectFieldPatch {
    locators: BTreeMap<AspectFieldLocator, AspectValue>,
}

pub fn planned_aspect_field_locator(
    aspect_key: AspectKey,
    field_path: CanonicalFieldPath,
) -> AspectFieldLocator {
    AspectFieldLocator::new(LocatorAuthority::Planned, aspect_key, field_path)
}

pub fn planned_single_field_locator(aspect_key: AspectKey, field: FieldKey) -> AspectFieldLocator {
    planned_aspect_field_locator(aspect_key, CanonicalFieldPath::single(field))
}

pub(crate) fn validate_planned_aspect_field_locator(
    locator: AspectFieldLocator,
) -> Result<AspectFieldLocator, AspectFieldPatchCodecError> {
    if locator.aspect().authority() != LocatorAuthority::Planned {
        return Err(AspectFieldPatchCodecError::new(
            "aspect field patch locator must use planned authority",
        ));
    }
    Ok(locator)
}

impl AspectFieldPatch {
    pub fn new(locators: BTreeMap<AspectFieldLocator, AspectValue>) -> Self {
        Self { locators }
    }

    pub fn from_locator(locator: AspectFieldLocator, value: AspectValue) -> Self {
        let mut locators = BTreeMap::new();
        locators.insert(locator, value);
        Self { locators }
    }

    pub fn is_empty(&self) -> bool {
        self.locators.is_empty()
    }

    pub fn len(&self) -> usize {
        self.locators.len()
    }

    pub fn locators(&self) -> impl Iterator<Item = &AspectFieldLocator> {
        self.locators.keys()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&AspectFieldLocator, &AspectValue)> {
        self.locators.iter()
    }

    pub fn get(&self, locator: &AspectFieldLocator) -> Option<&AspectValue> {
        self.locators.get(locator)
    }

    pub fn to_canonical_bytes(&self) -> Result<Vec<u8>, AspectFieldPatchCodecError> {
        encode_aspect_field_patch_canonical_bytes(self)
    }

    pub fn from_canonical_bytes(bytes: &[u8]) -> Result<Self, AspectFieldPatchCodecError> {
        decode_aspect_field_patch_canonical_bytes(bytes)
    }
}

impl From<BTreeMap<AspectFieldLocator, AspectValue>> for AspectFieldPatch {
    fn from(locators: BTreeMap<AspectFieldLocator, AspectValue>) -> Self {
        Self::new(locators)
    }
}

impl Serialize for AspectFieldPatch {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let bytes = self
            .to_canonical_bytes()
            .map_err(serde::ser::Error::custom)?;
        serializer.serialize_bytes(&bytes)
    }
}

impl<'de> Deserialize<'de> for AspectFieldPatch {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let bytes = Vec::<u8>::deserialize(deserializer)?;
        Self::from_canonical_bytes(&bytes).map_err(serde::de::Error::custom)
    }
}
