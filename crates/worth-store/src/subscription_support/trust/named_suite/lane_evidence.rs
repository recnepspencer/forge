use super::super::certification::SupportCertificationEvidenceBundle;
use super::super::failure::{
    SupportTrustFailure, SupportTrustFailureKind, SupportTrustRecoveryPosture,
};
use super::super::reports::CertifiedSupportTrustReport;
use super::super::taxonomy::{SupportTrustClass, SupportTrustProvenance, SupportTrustStrength};
use super::digest::stable_digest;
use super::lane_validation::{
    validate_certified_report_lane, validate_lane_outcome, validate_zero_counter_lane,
};
use super::row_kind::SubscriptionSupportAccuracyCertificationRowKind;
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum SubscriptionSupportAccuracyLaneOutcome {
    CertifiedPass,
    TypedRejection,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SubscriptionSupportAccuracyLaneEvidence {
    row_kind: SubscriptionSupportAccuracyCertificationRowKind,
    outcome: SubscriptionSupportAccuracyLaneOutcome,
    failure_kind: Option<SupportTrustFailureKind>,
    recovery_posture: Option<SupportTrustRecoveryPosture>,
    source_digest: String,
    diagnostics_digest: String,
    counter_digest: String,
    evidence_digest: String,
}

impl SubscriptionSupportAccuracyLaneEvidence {
    pub fn certified_pass_from_report(
        row_kind: SubscriptionSupportAccuracyCertificationRowKind,
        report: &CertifiedSupportTrustReport,
        diagnostics_digest: impl Into<String>,
        counter_digest: impl Into<String>,
    ) -> Result<Self, SupportTrustFailure> {
        validate_certified_report_lane(row_kind, report)?;
        let source_digest =
            stable_digest(&SubscriptionSupportAccuracyCertifiedReportLaneDigestBasis {
                row_kind,
                trust_class: report.trust_class(),
                trust_strength: report.trust_strength(),
                provenance: report.provenance(),
                suite_version: report.certification_stamp().suite_version(),
                row_id: report.certification_stamp().row_id(),
                evidence_bundle_digest: report.certification_stamp().evidence_bundle_digest(),
            })?;
        Self::certified_pass(row_kind, source_digest, diagnostics_digest, counter_digest)
    }

    pub fn certified_counter_pass_from_evidence_bundle(
        row_kind: SubscriptionSupportAccuracyCertificationRowKind,
        evidence_bundle: &SupportCertificationEvidenceBundle,
    ) -> Result<Self, SupportTrustFailure> {
        validate_zero_counter_lane(row_kind, evidence_bundle)?;
        let source_digest = stable_digest(&SubscriptionSupportAccuracyCounterLaneDigestBasis {
            row_kind,
            evidence_bundle_digest: evidence_bundle.evidence_bundle_digest(),
            forbidden_exact_overclaim_count: evidence_bundle
                .counter_snapshot()
                .forbidden_exact_overclaim_count(),
            global_scan_debt_count: evidence_bundle.counter_snapshot().global_scan_debt_count(),
        })?;
        Self::certified_pass(
            row_kind,
            source_digest,
            evidence_bundle.diagnostics_digest(),
            evidence_bundle.counter_snapshot_digest(),
        )
    }

    pub(crate) fn certified_pass(
        row_kind: SubscriptionSupportAccuracyCertificationRowKind,
        source_digest: impl Into<String>,
        diagnostics_digest: impl Into<String>,
        counter_digest: impl Into<String>,
    ) -> Result<Self, SupportTrustFailure> {
        Self::new(
            row_kind,
            SubscriptionSupportAccuracyLaneOutcome::CertifiedPass,
            None,
            None,
            source_digest,
            diagnostics_digest,
            counter_digest,
        )
    }

    pub fn typed_rejection_from_failure(
        row_kind: SubscriptionSupportAccuracyCertificationRowKind,
        failure: &SupportTrustFailure,
    ) -> Result<Self, SupportTrustFailure> {
        let source_digest = stable_digest(&SubscriptionSupportAccuracyFailureLaneDigestBasis {
            row_kind,
            digest_role: "failure-source",
            failure,
        })?;
        let diagnostics_digest =
            stable_digest(&SubscriptionSupportAccuracyFailureLaneDigestBasis {
                row_kind,
                digest_role: "failure-diagnostics",
                failure,
            })?;
        let counter_digest = stable_digest(&SubscriptionSupportAccuracyFailureLaneDigestBasis {
            row_kind,
            digest_role: "failure-counter",
            failure,
        })?;
        Self::typed_rejection(
            row_kind,
            failure,
            source_digest,
            diagnostics_digest,
            counter_digest,
        )
    }

    pub(crate) fn typed_rejection(
        row_kind: SubscriptionSupportAccuracyCertificationRowKind,
        failure: &SupportTrustFailure,
        source_digest: impl Into<String>,
        diagnostics_digest: impl Into<String>,
        counter_digest: impl Into<String>,
    ) -> Result<Self, SupportTrustFailure> {
        Self::new(
            row_kind,
            SubscriptionSupportAccuracyLaneOutcome::TypedRejection,
            Some(failure.kind()),
            Some(failure.recovery_posture()),
            source_digest,
            diagnostics_digest,
            counter_digest,
        )
    }

    pub fn row_kind(&self) -> SubscriptionSupportAccuracyCertificationRowKind {
        self.row_kind
    }

    pub fn evidence_digest(&self) -> &str {
        &self.evidence_digest
    }

    #[allow(clippy::too_many_arguments)]
    fn new(
        row_kind: SubscriptionSupportAccuracyCertificationRowKind,
        outcome: SubscriptionSupportAccuracyLaneOutcome,
        failure_kind: Option<SupportTrustFailureKind>,
        recovery_posture: Option<SupportTrustRecoveryPosture>,
        source_digest: impl Into<String>,
        diagnostics_digest: impl Into<String>,
        counter_digest: impl Into<String>,
    ) -> Result<Self, SupportTrustFailure> {
        validate_lane_outcome(row_kind, outcome, failure_kind)?;
        let mut evidence = Self {
            row_kind,
            outcome,
            failure_kind,
            recovery_posture,
            source_digest: require_non_empty("lane source digest", source_digest)?,
            diagnostics_digest: require_non_empty("lane diagnostics digest", diagnostics_digest)?,
            counter_digest: require_non_empty("lane counter digest", counter_digest)?,
            evidence_digest: String::new(),
        };
        evidence.evidence_digest =
            stable_digest(&SubscriptionSupportAccuracyLaneEvidenceDigestBasis {
                row_kind: evidence.row_kind,
                outcome: evidence.outcome,
                failure_kind: evidence.failure_kind,
                recovery_posture: evidence.recovery_posture,
                source_digest: &evidence.source_digest,
                diagnostics_digest: &evidence.diagnostics_digest,
                counter_digest: &evidence.counter_digest,
            })?;
        Ok(evidence)
    }
}

#[derive(Serialize)]
struct SubscriptionSupportAccuracyLaneEvidenceDigestBasis<'a> {
    row_kind: SubscriptionSupportAccuracyCertificationRowKind,
    outcome: SubscriptionSupportAccuracyLaneOutcome,
    failure_kind: Option<SupportTrustFailureKind>,
    recovery_posture: Option<SupportTrustRecoveryPosture>,
    source_digest: &'a str,
    diagnostics_digest: &'a str,
    counter_digest: &'a str,
}

#[derive(Serialize)]
struct SubscriptionSupportAccuracyCertifiedReportLaneDigestBasis<'a> {
    row_kind: SubscriptionSupportAccuracyCertificationRowKind,
    trust_class: SupportTrustClass,
    trust_strength: SupportTrustStrength,
    provenance: SupportTrustProvenance,
    suite_version: &'a str,
    row_id: &'a str,
    evidence_bundle_digest: &'a str,
}

#[derive(Serialize)]
struct SubscriptionSupportAccuracyCounterLaneDigestBasis<'a> {
    row_kind: SubscriptionSupportAccuracyCertificationRowKind,
    evidence_bundle_digest: &'a str,
    forbidden_exact_overclaim_count: u64,
    global_scan_debt_count: u64,
}

#[derive(Serialize)]
struct SubscriptionSupportAccuracyFailureLaneDigestBasis<'a> {
    row_kind: SubscriptionSupportAccuracyCertificationRowKind,
    digest_role: &'static str,
    failure: &'a SupportTrustFailure,
}

fn require_non_empty(
    label: &'static str,
    value: impl Into<String>,
) -> Result<String, SupportTrustFailure> {
    let value = value.into();
    if value.trim().is_empty() {
        return Err(SupportTrustFailure::new(
            SupportTrustFailureKind::SupportTrustCoverageMissing,
            SupportTrustRecoveryPosture::RerunCertification,
            format!("subscription-support accuracy suite {label} must be non-empty"),
        ));
    }
    Ok(value)
}
