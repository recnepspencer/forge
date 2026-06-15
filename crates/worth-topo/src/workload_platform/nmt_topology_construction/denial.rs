use super::pattern_spec::NmtTopologyPattern;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NmtTopologyConstructionDenial {
    pattern: NmtTopologyPattern,
    class: NmtTopologyConstructionDenialClass,
    reason: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NmtTopologyConstructionDenialClass {
    MissingDeclaration,
    UnsupportedCardinality,
    MissingRequiredEvidence,
    TopologyValidation,
    QueryAdmission,
}

impl NmtTopologyConstructionDenial {
    pub(crate) fn new(
        pattern: NmtTopologyPattern,
        class: NmtTopologyConstructionDenialClass,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            pattern,
            class,
            reason: normalize_reason(reason),
        }
    }

    pub fn pattern(&self) -> &NmtTopologyPattern {
        &self.pattern
    }

    pub fn class(&self) -> NmtTopologyConstructionDenialClass {
        self.class
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }
}

pub(crate) fn missing_declaration(pattern: NmtTopologyPattern) -> NmtTopologyConstructionDenial {
    NmtTopologyConstructionDenial::new(
        pattern,
        NmtTopologyConstructionDenialClass::MissingDeclaration,
        "NMT topology construction requires a human-readable declaration before any topology records are built.",
    )
}

pub(crate) fn unsupported_cardinality(
    pattern: NmtTopologyPattern,
    reason: impl Into<String>,
) -> NmtTopologyConstructionDenial {
    NmtTopologyConstructionDenial::new(
        pattern,
        NmtTopologyConstructionDenialClass::UnsupportedCardinality,
        reason,
    )
}

pub(crate) fn missing_required_evidence(
    pattern: NmtTopologyPattern,
    reason: impl Into<String>,
) -> NmtTopologyConstructionDenial {
    NmtTopologyConstructionDenial::new(
        pattern,
        NmtTopologyConstructionDenialClass::MissingRequiredEvidence,
        reason,
    )
}

pub(crate) fn topology_validation(
    pattern: NmtTopologyPattern,
    reason: impl Into<String>,
) -> NmtTopologyConstructionDenial {
    NmtTopologyConstructionDenial::new(
        pattern,
        NmtTopologyConstructionDenialClass::TopologyValidation,
        reason,
    )
}

pub(crate) fn query_admission(
    pattern: NmtTopologyPattern,
    reason: impl Into<String>,
) -> NmtTopologyConstructionDenial {
    NmtTopologyConstructionDenial::new(
        pattern,
        NmtTopologyConstructionDenialClass::QueryAdmission,
        reason,
    )
}

fn normalize_reason(reason: impl Into<String>) -> String {
    let reason = reason.into();
    if reason.trim().is_empty() {
        "NMT topology construction denials require a human-readable reason.".to_string()
    } else {
        reason
    }
}
