//! Typed identity for one external-effect correlation family.

/// Stable semantic identity shared by declaration, installation, dispatch, and aftermath views.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct WorthQueryExternalEffectCorrelationFamily(String);

impl WorthQueryExternalEffectCorrelationFamily {
    pub fn new(value: impl Into<String>) -> Result<Self, &'static str> {
        let value = value.into();
        if value.is_empty()
            || value.trim() != value
            || value.chars().any(char::is_whitespace)
            || value.contains('.')
        {
            return Err("invalid-external-effect-correlation-family");
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::WorthQueryExternalEffectCorrelationFamily;

    #[test]
    fn correlation_family_is_an_atomic_identifier() {
        assert!(WorthQueryExternalEffectCorrelationFamily::new("dispatch-rail").is_ok());
        for invalid in ["", " dispatch", "dispatch rail", "dispatch.rail"] {
            assert!(WorthQueryExternalEffectCorrelationFamily::new(invalid).is_err());
        }
    }
}
