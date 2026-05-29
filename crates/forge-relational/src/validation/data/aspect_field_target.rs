use forge_foundational::facade::{
    AspectFieldLocator, AspectKey, CanonicalFieldPath, FieldKey, LocatorAuthority,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct UniqueEntityAspectField {
    #[serde(with = "aspect_field_locator_serde")]
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

pub(crate) mod aspect_field_locator_serde {
    use super::{aspect_field_locator_from_parts, AspectFieldLocator, AspectKey, FieldKey};
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S>(locator: &AspectFieldLocator, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        (locator.aspect().aspect_key(), locator.field_path().fields()).serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<AspectFieldLocator, D::Error>
    where
        D: Deserializer<'de>,
    {
        let (aspect_key, fields) = <(AspectKey, Vec<FieldKey>)>::deserialize(deserializer)?;
        aspect_field_locator_from_parts(aspect_key, fields).map_err(serde::de::Error::custom)
    }
}

fn aspect_field_locator_from_parts(
    aspect_key: AspectKey,
    fields: Vec<FieldKey>,
) -> Result<AspectFieldLocator, &'static str> {
    let field_path = CanonicalFieldPath::new(fields)
        .ok_or("unique entity aspect field path must not be empty")?;
    Ok(AspectFieldLocator::new(
        LocatorAuthority::Planned,
        aspect_key,
        field_path,
    ))
}
