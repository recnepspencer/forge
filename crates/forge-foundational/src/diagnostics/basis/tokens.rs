use crate::diagnostics::materialization::{
    FoundationalDiagnosticAssemblyDebtClass, FoundationalDiagnosticPartiality,
    FoundationalDiagnosticSupportClaimStrength,
};
use crate::diagnostics::rows::{
    FoundationalDiagnosticLocalityClaim, FoundationalDiagnosticWidenedFalloutPosture,
};
use crate::diagnostics::{
    FoundationalDiagnosticArtifactKind, FoundationalDiagnosticAvailability,
    FoundationalDiagnosticBreachClass, FoundationalDiagnosticDenialClass,
    FoundationalDiagnosticEvidencePosture, FoundationalDiagnosticGapClass,
    FoundationalDiagnosticGapClosurePosture, FoundationalDiagnosticGapTarget,
    FoundationalDiagnosticSeverity,
};
use crate::profiles::{
    AdmissionReadinessProfile, CertificationPostureProfile, CompatibilityPostureProfile,
    DiagnosticRichnessProfile, RetentionDeliveryProfile, SupportPostureProfile,
};

pub(super) fn artifact_kind_token(value: FoundationalDiagnosticArtifactKind) -> &'static str {
    match value {
        FoundationalDiagnosticArtifactKind::Summary => "summary",
        FoundationalDiagnosticArtifactKind::Report => "report",
        FoundationalDiagnosticArtifactKind::FailureBundle => "failure-bundle",
        FoundationalDiagnosticArtifactKind::ComparisonBundle => "comparison-bundle",
        FoundationalDiagnosticArtifactKind::SupportReport => "support-report",
        FoundationalDiagnosticArtifactKind::ExplanationBundle => "explanation-bundle",
    }
}

pub(super) fn delivery_class_token(
    value: crate::diagnostics::FoundationalDiagnosticDeliveryClass,
) -> &'static str {
    match value {
        crate::diagnostics::FoundationalDiagnosticDeliveryClass::MustBeHot => "must-be-hot",
        crate::diagnostics::FoundationalDiagnosticDeliveryClass::CanDefer => "can-defer",
        crate::diagnostics::FoundationalDiagnosticDeliveryClass::ReconstructableFromReplay => {
            "reconstructable-from-replay"
        }
        crate::diagnostics::FoundationalDiagnosticDeliveryClass::UnavailableByPolicy => {
            "unavailable-by-policy"
        }
    }
}

pub(super) fn availability_token(value: FoundationalDiagnosticAvailability) -> &'static str {
    match value {
        FoundationalDiagnosticAvailability::RetainedHot => "retained-hot",
        FoundationalDiagnosticAvailability::DeferredCold => "deferred-cold",
        FoundationalDiagnosticAvailability::Reconstructable => "reconstructable",
        FoundationalDiagnosticAvailability::Redacted => "redacted",
        FoundationalDiagnosticAvailability::Unavailable => "unavailable",
    }
}

pub(super) fn partiality_token(value: &FoundationalDiagnosticPartiality) -> &'static str {
    match value {
        FoundationalDiagnosticPartiality::Complete => "complete",
        FoundationalDiagnosticPartiality::PartialWithNamedGaps(_) => "partial-with-named-gaps",
    }
}

pub(super) fn support_claim_strength_token(
    value: FoundationalDiagnosticSupportClaimStrength,
) -> &'static str {
    match value {
        FoundationalDiagnosticSupportClaimStrength::DescriptiveOnly => "descriptive-only",
        FoundationalDiagnosticSupportClaimStrength::DurableSupportReady => "durable-support-ready",
        FoundationalDiagnosticSupportClaimStrength::CertifiedSupportReady => {
            "certified-support-ready"
        }
    }
}

pub(super) fn severity_token(value: FoundationalDiagnosticSeverity) -> &'static str {
    match value {
        FoundationalDiagnosticSeverity::Info => "info",
        FoundationalDiagnosticSeverity::Advisory => "advisory",
        FoundationalDiagnosticSeverity::Warning => "warning",
        FoundationalDiagnosticSeverity::Denial => "denial",
        FoundationalDiagnosticSeverity::Failure => "failure",
        FoundationalDiagnosticSeverity::Violation => "violation",
    }
}

pub(super) fn denial_class_token(value: FoundationalDiagnosticDenialClass) -> &'static str {
    match value {
        FoundationalDiagnosticDenialClass::DomainDenied => "domain-denied",
        FoundationalDiagnosticDenialClass::PolicyDenied => "policy-denied",
        FoundationalDiagnosticDenialClass::UnsupportedDenied => "unsupported-denied",
        FoundationalDiagnosticDenialClass::EvidenceUnavailableDenied => {
            "evidence-unavailable-denied"
        }
    }
}

pub(super) fn breach_class_token(value: FoundationalDiagnosticBreachClass) -> &'static str {
    match value {
        FoundationalDiagnosticBreachClass::ConstructionBug => "construction-bug",
        FoundationalDiagnosticBreachClass::IntegrityMismatch => "integrity-mismatch",
        FoundationalDiagnosticBreachClass::CoverageOmission => "coverage-omission",
        FoundationalDiagnosticBreachClass::CanonicalizationViolation => {
            "canonicalization-violation"
        }
    }
}

pub(super) fn evidence_posture_token(value: FoundationalDiagnosticEvidencePosture) -> &'static str {
    match value {
        FoundationalDiagnosticEvidencePosture::RetainedDirect => "retained-direct",
        FoundationalDiagnosticEvidencePosture::Reconstructed => "reconstructed",
        FoundationalDiagnosticEvidencePosture::Summarized => "summarized",
        FoundationalDiagnosticEvidencePosture::Redacted => "redacted",
        FoundationalDiagnosticEvidencePosture::AbsentExpected => "absent-expected",
    }
}

pub(super) fn locality_claim_token(value: FoundationalDiagnosticLocalityClaim) -> &'static str {
    match value {
        FoundationalDiagnosticLocalityClaim::ExactSubject => "exact-subject",
        FoundationalDiagnosticLocalityClaim::SubjectNeighborhood => "subject-neighborhood",
        FoundationalDiagnosticLocalityClaim::WidenedScope => "widened-scope",
    }
}

pub(super) fn widened_fallout_token(
    value: FoundationalDiagnosticWidenedFalloutPosture,
) -> &'static str {
    match value {
        FoundationalDiagnosticWidenedFalloutPosture::NotWidened => "not-widened",
        FoundationalDiagnosticWidenedFalloutPosture::WidenedExpected => "widened-expected",
        FoundationalDiagnosticWidenedFalloutPosture::WidenedUnexpected => "widened-unexpected",
    }
}

pub(super) fn gap_class_token(value: FoundationalDiagnosticGapClass) -> &'static str {
    match value {
        FoundationalDiagnosticGapClass::OptionalEvidenceOmitted => "optional-evidence-omitted",
        FoundationalDiagnosticGapClass::SupportBreadthUnavailable => "support-breadth-unavailable",
        FoundationalDiagnosticGapClass::ReplayEvidenceUnavailable => "replay-evidence-unavailable",
        FoundationalDiagnosticGapClass::LocalityMismatch => "locality-mismatch",
        FoundationalDiagnosticGapClass::WidenedFallback => "widened-fallback",
        FoundationalDiagnosticGapClass::CoverageOmission => "coverage-omission",
    }
}

pub(super) fn gap_closure_posture_token(
    value: FoundationalDiagnosticGapClosurePosture,
) -> &'static str {
    match value {
        FoundationalDiagnosticGapClosurePosture::Deferred => "deferred",
        FoundationalDiagnosticGapClosurePosture::Unsupported => "unsupported",
        FoundationalDiagnosticGapClosurePosture::Denied => "denied",
        FoundationalDiagnosticGapClosurePosture::DebtNamed => "debt-named",
    }
}

pub(super) fn assembly_debt_class_token(
    value: FoundationalDiagnosticAssemblyDebtClass,
) -> &'static str {
    match value {
        FoundationalDiagnosticAssemblyDebtClass::RowScanFallback => "row-scan-fallback",
        FoundationalDiagnosticAssemblyDebtClass::WholeViewFallback => "whole-view-fallback",
        FoundationalDiagnosticAssemblyDebtClass::RepeatedRediscovery => "repeated-rediscovery",
    }
}

pub(super) fn gap_target_fragment(
    target: &FoundationalDiagnosticGapTarget,
) -> (&'static str, String) {
    match target {
        FoundationalDiagnosticGapTarget::Subject(subject) => {
            ("subject", subject.canonical_key_fragment())
        }
        FoundationalDiagnosticGapTarget::Locator(locator) => {
            ("locator", locator.canonical_key_fragment())
        }
    }
}

pub(super) fn diagnostic_richness_token(value: DiagnosticRichnessProfile) -> &'static str {
    match value {
        DiagnosticRichnessProfile::OperationalMinimal => "operational-minimal",
        DiagnosticRichnessProfile::Standard => "standard",
        DiagnosticRichnessProfile::Forensic => "forensic",
    }
}

pub(super) fn support_posture_token(value: SupportPostureProfile) -> &'static str {
    match value {
        SupportPostureProfile::InternalOnly => "internal-only",
        SupportPostureProfile::SupportReady => "support-ready",
        SupportPostureProfile::CertificationReady => "certification-ready",
    }
}

pub(super) fn compatibility_posture_token(value: CompatibilityPostureProfile) -> &'static str {
    match value {
        CompatibilityPostureProfile::NativeOnly => "native-only",
        CompatibilityPostureProfile::CompatibilityLowered => "compatibility-lowered",
        CompatibilityPostureProfile::CompatibilityRequired => "compatibility-required",
    }
}

pub(super) fn admission_readiness_token(value: AdmissionReadinessProfile) -> &'static str {
    match value {
        AdmissionReadinessProfile::CandidateOnly => "candidate-only",
        AdmissionReadinessProfile::Admitted => "admitted",
        AdmissionReadinessProfile::ProductionGateReady => "production-gate-ready",
    }
}

pub(super) fn retention_delivery_token(value: RetentionDeliveryProfile) -> &'static str {
    match value {
        RetentionDeliveryProfile::Ephemeral => "ephemeral",
        RetentionDeliveryProfile::Retained => "retained",
        RetentionDeliveryProfile::Durable => "durable",
    }
}

pub(super) fn certification_posture_token(value: CertificationPostureProfile) -> &'static str {
    match value {
        CertificationPostureProfile::Uncertified => "uncertified",
        CertificationPostureProfile::EvidenceBacked => "evidence-backed",
        CertificationPostureProfile::ProductionCertified => "production-certified",
    }
}
