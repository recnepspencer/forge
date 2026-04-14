use super::{AspectFieldKey, AspectName, AuthoringError, DeliveredFieldName, FieldName};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct AuthoredResultShapeField {
    source: AspectFieldKey,
    delivered_name: DeliveredFieldName,
}

impl AuthoredResultShapeField {
    pub fn new(
        source_aspect: impl Into<String>,
        source_field: impl Into<String>,
        delivered_name: impl Into<String>,
    ) -> Result<Self, AuthoringError> {
        Ok(Self {
            source: AspectFieldKey::new(source_aspect, source_field)
                .map_err(|_| AuthoringError::EmptyResultFieldSource)?,
            delivered_name: DeliveredFieldName::new(delivered_name)?,
        })
    }

    pub fn source_aspect(&self) -> &str {
        self.source.aspect().as_str()
    }

    pub fn source_field(&self) -> &str {
        self.source.field().as_str()
    }

    pub fn delivered_name(&self) -> &str {
        self.delivered_name.as_str()
    }

    pub fn source_aspect_name(&self) -> &AspectName {
        self.source.aspect()
    }

    pub fn source_field_name(&self) -> &FieldName {
        self.source.field()
    }

    pub fn delivered_field_name(&self) -> &DeliveredFieldName {
        &self.delivered_name
    }
}
