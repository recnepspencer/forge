use std::collections::BTreeMap;

use forge_foundational::facade::{
    AspectFieldLocator, AspectKey, AspectValue, CanonicalFieldPath, FieldKey, LocatorAuthority,
};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::aspect_wire::{
    decode_aspect_field_patch_canonical_bytes, decode_aspect_field_patch_target_canonical_bytes,
    encode_aspect_field_patch_canonical_bytes, encode_aspect_field_patch_target_canonical_bytes,
};

pub use crate::aspect_wire::AspectFieldPatchCodecError;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct AspectFieldPatch {
    targets: BTreeMap<AspectFieldPatchTarget, AspectValue>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct AspectFieldPatchTarget {
    locator: AspectFieldLocator,
}

impl AspectFieldPatchTarget {
    pub fn new(aspect_key: AspectKey, field_path: CanonicalFieldPath) -> Self {
        Self {
            locator: AspectFieldLocator::new(LocatorAuthority::Planned, aspect_key, field_path),
        }
    }

    pub fn single(aspect_key: AspectKey, field: FieldKey) -> Self {
        Self::new(aspect_key, CanonicalFieldPath::single(field))
    }

    pub(crate) fn from_locator(
        locator: AspectFieldLocator,
    ) -> Result<Self, AspectFieldPatchCodecError> {
        if locator.aspect().authority() != LocatorAuthority::Planned {
            return Err(AspectFieldPatchCodecError::new(
                "aspect field patch target locator must use planned authority",
            ));
        }
        Ok(Self { locator })
    }

    pub fn aspect_key(&self) -> &AspectKey {
        self.locator.aspect().aspect_key()
    }

    pub fn field_path(&self) -> &CanonicalFieldPath {
        self.locator.field_path()
    }

    pub fn locator(&self) -> &AspectFieldLocator {
        &self.locator
    }

    pub fn to_canonical_bytes(&self) -> Vec<u8> {
        encode_aspect_field_patch_target_canonical_bytes(self)
    }

    pub fn from_canonical_bytes(bytes: &[u8]) -> Result<Self, AspectFieldPatchCodecError> {
        decode_aspect_field_patch_target_canonical_bytes(bytes)
    }
}

impl AspectFieldPatch {
    pub fn new(targets: BTreeMap<AspectFieldPatchTarget, AspectValue>) -> Self {
        Self { targets }
    }

    pub fn from_target(target: AspectFieldPatchTarget, value: AspectValue) -> Self {
        let mut targets = BTreeMap::new();
        targets.insert(target, value);
        Self { targets }
    }

    pub fn is_empty(&self) -> bool {
        self.targets.is_empty()
    }

    pub fn len(&self) -> usize {
        self.targets.len()
    }

    pub fn targets(&self) -> impl Iterator<Item = &AspectFieldPatchTarget> {
        self.targets.keys()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&AspectFieldPatchTarget, &AspectValue)> {
        self.targets.iter()
    }

    pub fn get(&self, target: &AspectFieldPatchTarget) -> Option<&AspectValue> {
        self.targets.get(target)
    }

    pub fn to_canonical_bytes(&self) -> Result<Vec<u8>, AspectFieldPatchCodecError> {
        encode_aspect_field_patch_canonical_bytes(self)
    }

    pub fn from_canonical_bytes(bytes: &[u8]) -> Result<Self, AspectFieldPatchCodecError> {
        decode_aspect_field_patch_canonical_bytes(bytes)
    }
}

impl From<BTreeMap<AspectFieldPatchTarget, AspectValue>> for AspectFieldPatch {
    fn from(targets: BTreeMap<AspectFieldPatchTarget, AspectValue>) -> Self {
        Self::new(targets)
    }
}

impl Serialize for AspectFieldPatchTarget {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bytes(&self.to_canonical_bytes())
    }
}

impl<'de> Deserialize<'de> for AspectFieldPatchTarget {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let bytes = Vec::<u8>::deserialize(deserializer)?;
        Self::from_canonical_bytes(&bytes).map_err(serde::de::Error::custom)
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
