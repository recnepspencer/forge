use crate::{WorthServerQueryHandoffDenial, WorthServerQueryHandoffDenialCode};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerCanonicalFilename {
    original: String,
    canonical: String,
    canonical_digest: String,
}

impl WorthServerCanonicalFilename {
    pub(crate) fn admit(
        value: &str,
        diagnostics_profile: worth_foundational::facade::DiagnosticRichnessProfile,
        denial_code: WorthServerQueryHandoffDenialCode,
    ) -> Result<Self, WorthServerQueryHandoffDenial> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(WorthServerQueryHandoffDenial::new(
                denial_code,
                diagnostics_profile,
                "canonical external filename may not be blank",
            ));
        }
        if matches!(trimmed, "." | "..") {
            return Err(WorthServerQueryHandoffDenial::new(
                denial_code,
                diagnostics_profile,
                "canonical external filename may not be a traversal sentinel",
            ));
        }
        if trimmed.contains('/') || trimmed.contains('\\') {
            return Err(WorthServerQueryHandoffDenial::new(
                denial_code,
                diagnostics_profile,
                "canonical external filename may not contain path separators",
            ));
        }
        if trimmed.chars().any(|ch| {
            ch.is_control()
                || !ch.is_ascii()
                || matches!(ch, ':' | '*' | '?' | '"' | '<' | '>' | '|')
        }) {
            return Err(WorthServerQueryHandoffDenial::new(
                denial_code,
                diagnostics_profile,
                "canonical external filename must stay ASCII-printable and portability-safe",
            ));
        }

        let canonical = trimmed.to_ascii_lowercase();
        let canonical_digest =
            format!("worth-server-canonical-filename-v1|original={trimmed}|canonical={canonical}");
        Ok(Self {
            original: trimmed.to_string(),
            canonical,
            canonical_digest,
        })
    }

    pub fn original(&self) -> &str {
        &self.original
    }

    pub fn canonical(&self) -> &str {
        &self.canonical
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }
}

pub(crate) fn validate_canonical_filename(
    value: &str,
    diagnostics_profile: worth_foundational::facade::DiagnosticRichnessProfile,
    denial_code: WorthServerQueryHandoffDenialCode,
) -> Result<(), WorthServerQueryHandoffDenial> {
    WorthServerCanonicalFilename::admit(value, diagnostics_profile, denial_code).map(|_| ())
}
