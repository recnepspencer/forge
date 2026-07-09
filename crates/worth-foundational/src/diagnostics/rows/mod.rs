mod labels;
mod types;

use crate::diagnostics::outcomes::FoundationalDiagnosticOutcomeKind;
use crate::diagnostics::primitives::{FoundationalDiagnosticCodeId, FoundationalDiagnosticScopeId};
use crate::diagnostics::subjects::{FoundationalDiagnosticLocator, FoundationalDiagnosticSubject};

pub use labels::{
    FoundationalDiagnosticLocalityClaim, FoundationalDiagnosticRowFamily,
    FoundationalDiagnosticSemanticLabelSet, FoundationalDiagnosticSupportEvidencePosture,
    FoundationalDiagnosticWidenedFalloutPosture,
};
pub use types::{
    FoundationalDiagnosticComparisonRow, FoundationalDiagnosticDecisionRow,
    FoundationalDiagnosticFailureRow, FoundationalDiagnosticProvenanceReadyRow,
    FoundationalDiagnosticSupportRow,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FoundationalDiagnosticRow {
    Decision(FoundationalDiagnosticDecisionRow),
    Failure(FoundationalDiagnosticFailureRow),
    Comparison(FoundationalDiagnosticComparisonRow),
    Support(FoundationalDiagnosticSupportRow),
    ProvenanceReady(FoundationalDiagnosticProvenanceReadyRow),
}

impl FoundationalDiagnosticRow {
    pub const fn family(&self) -> FoundationalDiagnosticRowFamily {
        match self {
            Self::Decision(_) => FoundationalDiagnosticRowFamily::Decision,
            Self::Failure(_) => FoundationalDiagnosticRowFamily::Failure,
            Self::Comparison(_) => FoundationalDiagnosticRowFamily::Comparison,
            Self::Support(_) => FoundationalDiagnosticRowFamily::Support,
            Self::ProvenanceReady(_) => FoundationalDiagnosticRowFamily::ProvenanceReady,
        }
    }

    pub fn code(&self) -> &FoundationalDiagnosticCodeId {
        match self {
            Self::Decision(row) => row.code(),
            Self::Failure(row) => row.code(),
            Self::Comparison(row) => row.code(),
            Self::Support(row) => row.code(),
            Self::ProvenanceReady(row) => row.code(),
        }
    }

    pub fn scope(&self) -> &FoundationalDiagnosticScopeId {
        match self {
            Self::Decision(row) => row.scope(),
            Self::Failure(row) => row.scope(),
            Self::Comparison(row) => row.scope(),
            Self::Support(row) => row.scope(),
            Self::ProvenanceReady(row) => row.scope(),
        }
    }

    pub fn subject(&self) -> &FoundationalDiagnosticSubject {
        match self {
            Self::Decision(row) => row.subject(),
            Self::Failure(row) => row.subject(),
            Self::Comparison(row) => row.subject(),
            Self::Support(row) => row.subject(),
            Self::ProvenanceReady(row) => row.subject(),
        }
    }

    pub fn locator(&self) -> &FoundationalDiagnosticLocator {
        match self {
            Self::Decision(row) => row.locator(),
            Self::Failure(row) => row.locator(),
            Self::Comparison(row) => row.locator(),
            Self::Support(row) => row.locator(),
            Self::ProvenanceReady(row) => row.locator(),
        }
    }

    pub const fn outcome_kind(&self) -> FoundationalDiagnosticOutcomeKind {
        match self {
            Self::Decision(row) => row.outcome_kind(),
            Self::Failure(row) => row.outcome_kind(),
            Self::Comparison(row) => row.outcome_kind(),
            Self::Support(row) => row.outcome_kind(),
            Self::ProvenanceReady(row) => row.outcome_kind(),
        }
    }

    pub fn semantic_labels(&self) -> &FoundationalDiagnosticSemanticLabelSet {
        match self {
            Self::Decision(row) => row.semantic_labels(),
            Self::Failure(row) => row.semantic_labels(),
            Self::Comparison(row) => row.semantic_labels(),
            Self::Support(row) => row.semantic_labels(),
            Self::ProvenanceReady(row) => row.semantic_labels(),
        }
    }

    pub fn canonical_order_key(&self) -> String {
        match self {
            Self::Decision(row) => format!(
                "{}|{}|{}|{}",
                common_order_key(
                    self.family(),
                    row.code().as_str(),
                    row.scope().as_str(),
                    severity_token(row.severity()),
                    row.subject(),
                    row.locator(),
                    row.outcome_kind(),
                    row.semantic_labels(),
                ),
                optional_denial_class_token(row.denial_class()),
                locality_claim_token(row.locality_claim()),
                widened_fallout_token(row.widened_fallout_posture()),
            ),
            Self::Failure(row) => format!(
                "{}|{}|{}|{}",
                common_order_key(
                    self.family(),
                    row.code().as_str(),
                    row.scope().as_str(),
                    severity_token(row.severity()),
                    row.subject(),
                    row.locator(),
                    row.outcome_kind(),
                    row.semantic_labels(),
                ),
                breach_class_token(row.breach_class()),
                locality_claim_token(row.locality_claim()),
                widened_fallout_token(row.widened_fallout_posture()),
            ),
            Self::Comparison(row) => format!(
                "{}|{}|{}",
                common_order_key(
                    self.family(),
                    row.code().as_str(),
                    row.scope().as_str(),
                    severity_token(row.severity()),
                    row.subject(),
                    row.locator(),
                    row.outcome_kind(),
                    row.semantic_labels(),
                ),
                optional_locator_key(row.mismatch_locator()),
                evidence_posture_token(row.evidence_posture()),
            ),
            Self::Support(row) => format!(
                "{}|{}|{}|{}",
                common_order_key(
                    self.family(),
                    row.code().as_str(),
                    row.scope().as_str(),
                    severity_token(row.severity()),
                    row.subject(),
                    row.locator(),
                    row.outcome_kind(),
                    row.semantic_labels(),
                ),
                support_evidence_posture_key(row.evidence_posture()),
                locality_claim_token(row.locality_claim()),
                widened_fallout_token(row.widened_fallout_posture()),
            ),
            Self::ProvenanceReady(row) => format!(
                "{}|{}|{}",
                common_order_key(
                    self.family(),
                    row.code().as_str(),
                    row.scope().as_str(),
                    severity_token(row.severity()),
                    row.subject(),
                    row.locator(),
                    row.outcome_kind(),
                    row.semantic_labels(),
                ),
                row.evidence_origin_locator().canonical_key_fragment(),
                evidence_posture_token(row.evidence_posture()),
            ),
        }
    }
}

pub fn sort_foundational_diagnostic_rows(rows: &mut [FoundationalDiagnosticRow]) {
    rows.sort_by_cached_key(FoundationalDiagnosticRow::canonical_order_key);
}

fn common_order_key(
    family: FoundationalDiagnosticRowFamily,
    code: &str,
    scope: &str,
    severity: &'static str,
    subject: &FoundationalDiagnosticSubject,
    locator: &FoundationalDiagnosticLocator,
    outcome_kind: FoundationalDiagnosticOutcomeKind,
    labels: &FoundationalDiagnosticSemanticLabelSet,
) -> String {
    let labels = labels
        .labels()
        .iter()
        .map(FoundationalDiagnosticCodeId::as_str)
        .collect::<Vec<_>>()
        .join(",");
    format!(
        "{}|{}|{}|{}|{}|{}|{}|{}",
        family.canonical_name(),
        scope,
        code,
        severity,
        subject.canonical_key_fragment(),
        locator.canonical_key_fragment(),
        outcome_kind.canonical_name(),
        labels
    )
}

fn severity_token(value: crate::diagnostics::FoundationalDiagnosticSeverity) -> &'static str {
    match value {
        crate::diagnostics::FoundationalDiagnosticSeverity::Info => "info",
        crate::diagnostics::FoundationalDiagnosticSeverity::Advisory => "advisory",
        crate::diagnostics::FoundationalDiagnosticSeverity::Warning => "warning",
        crate::diagnostics::FoundationalDiagnosticSeverity::Denial => "denial",
        crate::diagnostics::FoundationalDiagnosticSeverity::Failure => "failure",
        crate::diagnostics::FoundationalDiagnosticSeverity::Violation => "violation",
    }
}

fn optional_denial_class_token(
    value: Option<crate::diagnostics::FoundationalDiagnosticDenialClass>,
) -> &'static str {
    match value {
        Some(crate::diagnostics::FoundationalDiagnosticDenialClass::DomainDenied) => {
            "domain-denied"
        }
        Some(crate::diagnostics::FoundationalDiagnosticDenialClass::PolicyDenied) => {
            "policy-denied"
        }
        Some(crate::diagnostics::FoundationalDiagnosticDenialClass::UnsupportedDenied) => {
            "unsupported-denied"
        }
        Some(crate::diagnostics::FoundationalDiagnosticDenialClass::EvidenceUnavailableDenied) => {
            "evidence-unavailable-denied"
        }
        None => "none",
    }
}

fn breach_class_token(
    value: crate::diagnostics::FoundationalDiagnosticBreachClass,
) -> &'static str {
    match value {
        crate::diagnostics::FoundationalDiagnosticBreachClass::ConstructionBug => {
            "construction-bug"
        }
        crate::diagnostics::FoundationalDiagnosticBreachClass::IntegrityMismatch => {
            "integrity-mismatch"
        }
        crate::diagnostics::FoundationalDiagnosticBreachClass::CoverageOmission => {
            "coverage-omission"
        }
        crate::diagnostics::FoundationalDiagnosticBreachClass::CanonicalizationViolation => {
            "canonicalization-violation"
        }
    }
}

fn evidence_posture_token(
    value: crate::diagnostics::FoundationalDiagnosticEvidencePosture,
) -> &'static str {
    match value {
        crate::diagnostics::FoundationalDiagnosticEvidencePosture::RetainedDirect => {
            "retained-direct"
        }
        crate::diagnostics::FoundationalDiagnosticEvidencePosture::Reconstructed => "reconstructed",
        crate::diagnostics::FoundationalDiagnosticEvidencePosture::Summarized => "summarized",
        crate::diagnostics::FoundationalDiagnosticEvidencePosture::Redacted => "redacted",
        crate::diagnostics::FoundationalDiagnosticEvidencePosture::AbsentExpected => {
            "absent-expected"
        }
    }
}

fn locality_claim_token(
    value: crate::diagnostics::FoundationalDiagnosticLocalityClaim,
) -> &'static str {
    match value {
        crate::diagnostics::FoundationalDiagnosticLocalityClaim::ExactSubject => "exact-subject",
        crate::diagnostics::FoundationalDiagnosticLocalityClaim::SubjectNeighborhood => {
            "subject-neighborhood"
        }
        crate::diagnostics::FoundationalDiagnosticLocalityClaim::WidenedScope => "widened-scope",
    }
}

fn widened_fallout_token(
    value: crate::diagnostics::FoundationalDiagnosticWidenedFalloutPosture,
) -> &'static str {
    match value {
        crate::diagnostics::FoundationalDiagnosticWidenedFalloutPosture::NotWidened => {
            "not-widened"
        }
        crate::diagnostics::FoundationalDiagnosticWidenedFalloutPosture::WidenedExpected => {
            "widened-expected"
        }
        crate::diagnostics::FoundationalDiagnosticWidenedFalloutPosture::WidenedUnexpected => {
            "widened-unexpected"
        }
    }
}

fn support_evidence_posture_key(value: &FoundationalDiagnosticSupportEvidencePosture) -> String {
    match value {
        FoundationalDiagnosticSupportEvidencePosture::Present(posture) => {
            format!("present:{}", evidence_posture_token(*posture))
        }
        FoundationalDiagnosticSupportEvidencePosture::Absent(cause) => {
            format!("absent:{}", cause.canonical_name())
        }
        FoundationalDiagnosticSupportEvidencePosture::OmittedConstructionBug(class) => {
            format!("omitted-construction-bug:{}", breach_class_token(*class))
        }
    }
}

fn optional_locator_key(value: Option<&FoundationalDiagnosticLocator>) -> String {
    value
        .map(FoundationalDiagnosticLocator::canonical_key_fragment)
        .unwrap_or_else(|| "none".to_string())
}
