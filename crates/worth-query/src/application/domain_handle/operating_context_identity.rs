use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryDomainOperatingContextIdentityError {
    EmptyFieldName,
    InvalidFieldName,
    DuplicateField,
}

impl std::fmt::Display for WorthQueryDomainOperatingContextIdentityError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Self::EmptyFieldName => "operating-context identity field name cannot be empty",
            Self::InvalidFieldName => {
                "operating-context identity field names use ASCII letters, digits, '-' or '_'"
            }
            Self::DuplicateField => "operating-context identity fields must be unique",
        };
        formatter.write_str(message)
    }
}

impl std::error::Error for WorthQueryDomainOperatingContextIdentityError {}

/// Domain-owned semantic fields that Query canonicalizes into context identity.
///
/// Values are data, never pre-encoded digests. Field order is intentionally
/// erased so equivalent declarations converge before Query seals identity.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryDomainOperatingContextIdentityDeclaration {
    fields: BTreeMap<&'static str, String>,
}

impl WorthQueryDomainOperatingContextIdentityDeclaration {
    /// Declares a context whose semantic identity has one value.
    pub fn single(value: impl Into<String>) -> Self {
        Self {
            fields: BTreeMap::from([("context", value.into())]),
        }
    }

    /// Declares named semantic fields; Query owns ordering and identity encoding.
    pub fn from_fields<I, V>(
        fields: I,
    ) -> Result<Self, WorthQueryDomainOperatingContextIdentityError>
    where
        I: IntoIterator<Item = (&'static str, V)>,
        V: Into<String>,
    {
        let mut declared = BTreeMap::new();
        for (name, value) in fields {
            validate_field_name(name)?;
            if declared.insert(name, value.into()).is_some() {
                return Err(WorthQueryDomainOperatingContextIdentityError::DuplicateField);
            }
        }
        Ok(Self { fields: declared })
    }

    pub fn fields(&self) -> impl ExactSizeIterator<Item = (&'static str, &str)> + '_ {
        self.fields
            .iter()
            .map(|(name, value)| (*name, value.as_str()))
    }
}

fn validate_field_name(
    name: &'static str,
) -> Result<(), WorthQueryDomainOperatingContextIdentityError> {
    if name.is_empty() {
        return Err(WorthQueryDomainOperatingContextIdentityError::EmptyFieldName);
    }
    if !name
        .bytes()
        .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-' || byte == b'_')
    {
        return Err(WorthQueryDomainOperatingContextIdentityError::InvalidFieldName);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn field_order_is_canonical_and_duplicate_fields_deny() {
        let left = WorthQueryDomainOperatingContextIdentityDeclaration::from_fields([
            ("mode", "strict"),
            ("tenant", "alpha"),
        ])
        .unwrap();
        let right = WorthQueryDomainOperatingContextIdentityDeclaration::from_fields([
            ("tenant", "alpha"),
            ("mode", "strict"),
        ])
        .unwrap();
        assert_eq!(left, right);

        let duplicate = WorthQueryDomainOperatingContextIdentityDeclaration::from_fields([
            ("mode", "strict"),
            ("mode", "relaxed"),
        ]);
        assert_eq!(
            duplicate.unwrap_err(),
            WorthQueryDomainOperatingContextIdentityError::DuplicateField
        );
    }
}
