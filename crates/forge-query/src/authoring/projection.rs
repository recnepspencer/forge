use super::{AspectFieldKey, AspectName, AuthoringError, FieldName};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct AspectFieldSelector {
    key: AspectFieldKey,
}

impl AspectFieldSelector {
    pub fn new(
        aspect: impl Into<String>,
        field: impl Into<String>,
    ) -> Result<Self, AuthoringError> {
        Ok(Self {
            key: AspectFieldKey::from_authoring_parts(aspect, field)?,
        })
    }

    pub fn from_source_field_key(key: AspectFieldKey) -> Self {
        Self { key }
    }

    pub fn source_field_key(&self) -> &AspectFieldKey {
        &self.key
    }

    pub(crate) fn aspect(&self) -> &str {
        self.key.aspect().as_str()
    }

    pub(crate) fn field(&self) -> &str {
        self.key.field().as_str()
    }

    pub fn aspect_name(&self) -> &AspectName {
        self.key.aspect()
    }

    pub fn field_name(&self) -> &FieldName {
        self.key.field()
    }
}
