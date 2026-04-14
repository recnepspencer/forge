use super::AuthoringError;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct AuthoredResultShapeField {
    source_aspect: String,
    source_field: String,
    delivered_name: String,
}

impl AuthoredResultShapeField {
    pub fn new(
        source_aspect: impl Into<String>,
        source_field: impl Into<String>,
        delivered_name: impl Into<String>,
    ) -> Result<Self, AuthoringError> {
        let source_aspect = source_aspect.into();
        let source_field = source_field.into();
        let delivered_name = delivered_name.into();
        if source_aspect.trim().is_empty() || source_field.trim().is_empty() {
            return Err(AuthoringError::EmptyResultFieldSource);
        }
        if delivered_name.trim().is_empty() {
            return Err(AuthoringError::EmptyDeliveredFieldName);
        }
        Ok(Self {
            source_aspect,
            source_field,
            delivered_name,
        })
    }

    pub fn source_aspect(&self) -> &str {
        &self.source_aspect
    }

    pub fn source_field(&self) -> &str {
        &self.source_field
    }

    pub fn delivered_name(&self) -> &str {
        &self.delivered_name
    }
}
