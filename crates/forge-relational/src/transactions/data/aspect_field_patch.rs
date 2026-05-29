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

    pub fn single(aspect_key: AspectKey, field: FieldKey, value: AspectValue) -> Self {
        let mut targets = BTreeMap::new();
        targets.insert(AspectFieldPatchTarget::single(aspect_key, field), value);
        Self { targets }
    }

    pub fn from_single_field_map(
        aspect_key: AspectKey,
        fields: BTreeMap<FieldKey, AspectValue>,
    ) -> Self {
        Self::new(
            fields
                .into_iter()
                .map(|(field, value)| {
                    (
                        AspectFieldPatchTarget::single(aspect_key.clone(), field),
                        value,
                    )
                })
                .collect(),
        )
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

    pub fn path_labels(&self) -> impl Iterator<Item = String> + '_ {
        self.targets.keys().map(aspect_field_patch_target_label)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&AspectFieldPatchTarget, &AspectValue)> {
        self.targets.iter()
    }

    pub fn get(&self, target: &AspectFieldPatchTarget) -> Option<&AspectValue> {
        self.targets.get(target)
    }

    pub fn get_single_field(
        &self,
        aspect_key: &AspectKey,
        field: &FieldKey,
    ) -> Option<&AspectValue> {
        self.get(&AspectFieldPatchTarget::single(
            aspect_key.clone(),
            field.clone(),
        ))
    }

    pub fn to_canonical_bytes(&self) -> Result<Vec<u8>, AspectFieldPatchCodecError> {
        encode_aspect_field_patch_canonical_bytes(self)
    }

    pub fn from_canonical_bytes(bytes: &[u8]) -> Result<Self, AspectFieldPatchCodecError> {
        decode_aspect_field_patch_canonical_bytes(bytes)
    }
}

pub fn aspect_field_patch_target_label(target: &AspectFieldPatchTarget) -> String {
    format!(
        "{}:{}",
        target.aspect_key().as_str(),
        canonical_field_path_label(target.field_path())
    )
}

pub fn canonical_field_path_label(path: &CanonicalFieldPath) -> String {
    path.fields()
        .iter()
        .map(FieldKey::as_str)
        .collect::<Vec<_>>()
        .join(".")
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
