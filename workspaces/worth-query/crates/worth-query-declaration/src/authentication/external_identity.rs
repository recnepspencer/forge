use worth_foundational::facade::{AspectValue, InternedString, ScalarAspectType};

use crate::application_schema::TypedApplicationValue;

const MAX_ISSUER_BYTES: usize = 2_048;
const MAX_SUBJECT_BYTES: usize = 1_024;

/// Stable external identity carried across authentication adapters.
///
/// Issuer and subject remain separate typed components. The Foundational value
/// conversion uses a length-delimited encoding so one equality index can
/// resolve the pair without a lossy digest or issuer-wide scan.
#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct WorthQueryExternalPrincipalIdentity {
    issuer: String,
    subject: String,
}
crate::worth_query_portable_type!(
    WorthQueryExternalPrincipalIdentity => "worth.query.external_principal_identity.v1"
);

impl std::fmt::Debug for WorthQueryExternalPrincipalIdentity {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("WorthQueryExternalPrincipalIdentity")
            .finish_non_exhaustive()
    }
}

impl WorthQueryExternalPrincipalIdentity {
    pub fn new(
        issuer: impl Into<String>,
        subject: impl Into<String>,
    ) -> Result<Self, WorthQueryExternalPrincipalIdentityDenial> {
        let issuer = issuer.into();
        let subject = subject.into();
        validate_component("issuer", &issuer, MAX_ISSUER_BYTES)?;
        validate_component("subject", &subject, MAX_SUBJECT_BYTES)?;
        Ok(Self { issuer, subject })
    }

    pub fn issuer(&self) -> &str {
        &self.issuer
    }

    pub fn subject(&self) -> &str {
        &self.subject
    }

    fn into_index_value(self) -> String {
        format!(
            "{}:{}{}:{}",
            self.issuer.len(),
            self.issuer,
            self.subject.len(),
            self.subject
        )
    }
}

impl TypedApplicationValue for WorthQueryExternalPrincipalIdentity {
    const SCALAR_FAMILY: ScalarAspectType = ScalarAspectType::String;

    fn into_foundational_value(self) -> AspectValue {
        AspectValue::String(InternedString::from(self.into_index_value()))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryExternalPrincipalIdentityDenialKind {
    Empty,
    SurroundingWhitespace,
    ControlCharacter,
    TooLong,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryExternalPrincipalIdentityDenial {
    component: &'static str,
    kind: WorthQueryExternalPrincipalIdentityDenialKind,
}

impl WorthQueryExternalPrincipalIdentityDenial {
    pub const fn component(&self) -> &'static str {
        self.component
    }

    pub const fn kind(&self) -> WorthQueryExternalPrincipalIdentityDenialKind {
        self.kind
    }
}

impl std::fmt::Display for WorthQueryExternalPrincipalIdentityDenial {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "external principal {} denied: {:?}",
            self.component, self.kind
        )
    }
}

impl std::error::Error for WorthQueryExternalPrincipalIdentityDenial {}

fn validate_component(
    component: &'static str,
    value: &str,
    maximum_bytes: usize,
) -> Result<(), WorthQueryExternalPrincipalIdentityDenial> {
    let kind = if value.is_empty() {
        Some(WorthQueryExternalPrincipalIdentityDenialKind::Empty)
    } else if value.trim() != value {
        Some(WorthQueryExternalPrincipalIdentityDenialKind::SurroundingWhitespace)
    } else if value.chars().any(char::is_control) {
        Some(WorthQueryExternalPrincipalIdentityDenialKind::ControlCharacter)
    } else if value.len() > maximum_bytes {
        Some(WorthQueryExternalPrincipalIdentityDenialKind::TooLong)
    } else {
        None
    };
    match kind {
        Some(kind) => Err(WorthQueryExternalPrincipalIdentityDenial { component, kind }),
        None => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn length_delimited_pairs_do_not_alias() {
        let left = WorthQueryExternalPrincipalIdentity::new("ab", "c")
            .unwrap()
            .into_index_value();
        let right = WorthQueryExternalPrincipalIdentity::new("a", "bc")
            .unwrap()
            .into_index_value();
        assert_ne!(left, right);
    }

    #[test]
    fn invalid_components_fail_before_index_encoding() {
        for (value, expected) in [
            ("", WorthQueryExternalPrincipalIdentityDenialKind::Empty),
            (
                " issuer",
                WorthQueryExternalPrincipalIdentityDenialKind::SurroundingWhitespace,
            ),
            (
                "issuer\n",
                WorthQueryExternalPrincipalIdentityDenialKind::SurroundingWhitespace,
            ),
        ] {
            assert_eq!(
                WorthQueryExternalPrincipalIdentity::new(value, "subject")
                    .unwrap_err()
                    .kind(),
                expected
            );
        }
    }

    #[test]
    fn control_characters_and_overlength_components_fail_at_the_typed_boundary() {
        for (issuer, subject, component, kind) in [
            (
                "issuer\u{0001}".to_string(),
                "subject".to_string(),
                "issuer",
                WorthQueryExternalPrincipalIdentityDenialKind::ControlCharacter,
            ),
            (
                "issuer".to_string(),
                "subject\u{007f}".to_string(),
                "subject",
                WorthQueryExternalPrincipalIdentityDenialKind::ControlCharacter,
            ),
            (
                "i".repeat(MAX_ISSUER_BYTES + 1),
                "subject".to_string(),
                "issuer",
                WorthQueryExternalPrincipalIdentityDenialKind::TooLong,
            ),
            (
                "issuer".to_string(),
                "s".repeat(MAX_SUBJECT_BYTES + 1),
                "subject",
                WorthQueryExternalPrincipalIdentityDenialKind::TooLong,
            ),
        ] {
            let denial = WorthQueryExternalPrincipalIdentity::new(issuer, subject).unwrap_err();
            assert_eq!(denial.component(), component);
            assert_eq!(denial.kind(), kind);
        }
    }

    #[test]
    fn debug_output_discloses_neither_issuer_nor_subject() {
        let identity =
            WorthQueryExternalPrincipalIdentity::new("https://issuer.example", "subject-123")
                .unwrap();
        let debug = format!("{identity:?}");
        assert!(!debug.contains("https://issuer.example"));
        assert!(!debug.contains("subject-123"));
    }
}
