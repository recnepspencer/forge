use std::borrow::Borrow;
use std::fmt;

use super::AuthoringError;
use forge_foundational::facade::{AspectKey, FieldKey};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct AspectName(AspectKey);

impl AspectName {
    pub fn new(name: impl Into<String>) -> Result<Self, AuthoringError> {
        let name = name.into();
        let Some(key) = AspectKey::new(name) else {
            return Err(AuthoringError::EmptyProjectionSelector);
        };
        Ok(Self(key))
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl Borrow<str> for AspectName {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for AspectName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct FieldName(FieldKey);

impl FieldName {
    pub fn new(name: impl Into<String>) -> Result<Self, AuthoringError> {
        let name = name.into();
        let Some(key) = FieldKey::new(name) else {
            return Err(AuthoringError::EmptyProjectionSelector);
        };
        Ok(Self(key))
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl Borrow<str> for FieldName {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for FieldName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct DeliveredFieldName(String);

impl DeliveredFieldName {
    pub fn new(name: impl Into<String>) -> Result<Self, AuthoringError> {
        let name = name.into();
        if name.trim().is_empty() {
            return Err(AuthoringError::EmptyDeliveredFieldName);
        }
        Ok(Self(name))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Borrow<str> for DeliveredFieldName {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for DeliveredFieldName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct RelationName(String);

impl RelationName {
    pub fn new(name: impl Into<String>) -> Result<Self, AuthoringError> {
        let name = name.into();
        if name.trim().is_empty() {
            return Err(AuthoringError::EmptyTraversalRelation);
        }
        Ok(Self(name))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Borrow<str> for RelationName {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for RelationName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct AspectFieldKey {
    aspect: AspectName,
    field: FieldName,
}

impl AspectFieldKey {
    pub fn from_authoring_parts(
        aspect: impl Into<String>,
        field: impl Into<String>,
    ) -> Result<Self, AuthoringError> {
        Ok(Self {
            aspect: AspectName::new(aspect)?,
            field: FieldName::new(field)?,
        })
    }

    pub fn from_parts(aspect: AspectName, field: FieldName) -> Self {
        Self { aspect, field }
    }

    pub fn from_native_keys(aspect: &AspectKey, field: &FieldKey) -> Self {
        Self {
            aspect: AspectName(aspect.clone()),
            field: FieldName(field.clone()),
        }
    }

    pub fn aspect(&self) -> &AspectName {
        &self.aspect
    }

    pub fn field(&self) -> &FieldName {
        &self.field
    }

    pub(crate) fn native_aspect_key(&self) -> AspectKey {
        self.aspect.0.clone()
    }

    pub(crate) fn native_field_key(&self) -> FieldKey {
        self.field.0.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aspect_field_key_admission_uses_foundational_keys() {
        assert!(AspectFieldKey::from_authoring_parts("", "value").is_err());
        assert!(AspectFieldKey::from_authoring_parts("title", "").is_err());
        assert!(AspectFieldKey::from_authoring_parts("bad key", "value").is_err());
        assert!(AspectFieldKey::from_authoring_parts("title", "bad key").is_err());
    }

    #[test]
    fn aspect_field_key_from_native_keys_preserves_native_carriers() {
        let aspect = AspectKey::new("title").expect("aspect key should admit");
        let field = FieldKey::new("value").expect("field key should admit");
        let key = AspectFieldKey::from_native_keys(&aspect, &field);

        assert_eq!(key.native_aspect_key(), aspect);
        assert_eq!(key.native_field_key(), field);
        assert_eq!(key.aspect().as_str(), "title");
        assert_eq!(key.field().as_str(), "value");
    }
}
