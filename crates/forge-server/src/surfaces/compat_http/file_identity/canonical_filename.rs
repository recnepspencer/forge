use crate::{ForgeServerQueryHandoffDenial, ForgeServerQueryHandoffDenialCode};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerCanonicalFilename {
    original: String,
    canonical: String,
    canonical_digest: String,
}

impl ForgeServerCanonicalFilename {
    pub(crate) fn admit(
        value: &str,
        diagnostics_profile: forge_foundational::facade::DiagnosticRichnessProfile,
        denial_code: ForgeServerQueryHandoffDenialCode,
    ) -> Result<Self, ForgeServerQueryHandoffDenial> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(ForgeServerQueryHandoffDenial::new(
                denial_code,
                diagnostics_profile,
                "canonical external filename may not be blank",
            ));
        }
        if matches!(trimmed, "." | "..") {
            return Err(ForgeServerQueryHandoffDenial::new(
                denial_code,
                diagnostics_profile,
                "canonical external filename may not be a traversal sentinel",
            ));
        }
        if trimmed.contains('/') || trimmed.contains('\\') {
            return Err(ForgeServerQueryHandoffDenial::new(
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
            return Err(ForgeServerQueryHandoffDenial::new(
                denial_code,
                diagnostics_profile,
                "canonical external filename must stay ASCII-printable and portability-safe",
            ));
        }

        let canonical = trimmed.to_ascii_lowercase();
        let canonical_digest =
            format!("forge-server-canonical-filename-v1|original={trimmed}|canonical={canonical}");
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
    diagnostics_profile: forge_foundational::facade::DiagnosticRichnessProfile,
    denial_code: ForgeServerQueryHandoffDenialCode,
) -> Result<(), ForgeServerQueryHandoffDenial> {
    ForgeServerCanonicalFilename::admit(value, diagnostics_profile, denial_code).map(|_| ())
}
