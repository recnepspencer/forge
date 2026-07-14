#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FoundationalDiagnosticPrimitiveConstructionDenial {
    CodeMustNotBeEmpty,
    CodeMustUseLowercaseAsciiTokens,
    CodeMustNotContainEmptySegments,
    ScopeMustNotBeEmpty,
    ScopeMustUseLowercaseAsciiTokens,
    ScopeMustNotContainEmptySegments,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FoundationalDiagnosticCodeId(String);

impl FoundationalDiagnosticCodeId {
    pub fn new(
        value: impl Into<String>,
    ) -> Result<Self, FoundationalDiagnosticPrimitiveConstructionDenial> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(FoundationalDiagnosticPrimitiveConstructionDenial::CodeMustNotBeEmpty);
        }
        if !is_lowercase_ascii_token(&value) {
            return Err(
                FoundationalDiagnosticPrimitiveConstructionDenial::CodeMustUseLowercaseAsciiTokens,
            );
        }
        if !has_non_empty_token_segments(&value) {
            return Err(
                FoundationalDiagnosticPrimitiveConstructionDenial::CodeMustNotContainEmptySegments,
            );
        }

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FoundationalDiagnosticScopeId(String);

impl FoundationalDiagnosticScopeId {
    pub fn new(
        value: impl Into<String>,
    ) -> Result<Self, FoundationalDiagnosticPrimitiveConstructionDenial> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(FoundationalDiagnosticPrimitiveConstructionDenial::ScopeMustNotBeEmpty);
        }
        if !is_lowercase_ascii_token(&value) {
            return Err(
                FoundationalDiagnosticPrimitiveConstructionDenial::ScopeMustUseLowercaseAsciiTokens,
            );
        }
        if !has_non_empty_token_segments(&value) {
            return Err(
                FoundationalDiagnosticPrimitiveConstructionDenial::ScopeMustNotContainEmptySegments,
            );
        }

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalDiagnosticSeverity {
    Info,
    Advisory,
    Warning,
    Denial,
    Failure,
    Violation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalDiagnosticBreachClass {
    ConstructionBug,
    IntegrityMismatch,
    CoverageOmission,
    CanonicalizationViolation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalDiagnosticDenialClass {
    DomainDenied,
    PolicyDenied,
    UnsupportedDenied,
    EvidenceUnavailableDenied,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalDiagnosticEvidencePosture {
    RetainedDirect,
    Reconstructed,
    Summarized,
    Redacted,
    AbsentExpected,
}

pub fn foundational_diagnostic_code(
    value: impl Into<String>,
) -> Result<FoundationalDiagnosticCodeId, FoundationalDiagnosticPrimitiveConstructionDenial> {
    FoundationalDiagnosticCodeId::new(value)
}

pub fn foundational_diagnostic_scope(
    value: impl Into<String>,
) -> Result<FoundationalDiagnosticScopeId, FoundationalDiagnosticPrimitiveConstructionDenial> {
    FoundationalDiagnosticScopeId::new(value)
}

fn is_lowercase_ascii_token(value: &str) -> bool {
    value
        .bytes()
        .all(|byte| matches!(byte, b'a'..=b'z' | b'0'..=b'9' | b'.' | b'-' | b'_'))
}

fn has_non_empty_token_segments(value: &str) -> bool {
    let mut previous_was_separator = true;
    let mut saw_alphanumeric = false;

    for byte in value.bytes() {
        let is_separator = matches!(byte, b'.' | b'-' | b'_');
        if is_separator {
            if previous_was_separator {
                return false;
            }
            previous_was_separator = true;
            continue;
        }

        saw_alphanumeric = true;
        previous_was_separator = false;
    }

    saw_alphanumeric && !previous_was_separator
}
