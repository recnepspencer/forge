use super::super::certification::SupportCertificationEvidenceBundle;
use super::super::domain_certification::{
    SupportCertificationHandoffReport, SupportDomainCertificationBundle,
    SupportGenericCertificationReport,
};
use super::super::failure::{
    SupportTrustFailure, SupportTrustFailureKind, SupportTrustRecoveryPosture,
};
use super::certification_row::SubscriptionSupportAccuracyCertificationRow;
use super::counter_snapshot::SubscriptionSupportAccuracyCertificationCounterSnapshot;
use super::digest::stable_digest;
use super::handoff_validation::{validate_handoff, validate_handoff_matches_phase_artifacts};
use super::lane_evidence_set::SubscriptionSupportAccuracyLaneEvidenceSet;
use super::outputs::SubscriptionSupportAccuracyCertificationOutputs;
use super::phase_artifact_rows::{
    build_required_rows_from_phase_artifacts, validate_rows_match_phase_artifacts,
};
use super::row_kind::{
    SubscriptionSupportAccuracyCertificationRowKind,
    SUBSCRIPTION_SUPPORT_ACCURACY_CERTIFICATION_SUITE_NAME,
};
use super::suite_validation::{validate_required_row_count, validate_required_rows};
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SubscriptionSupportAccuracyCertificationSuite {
    suite_name: String,
    rows: Vec<SubscriptionSupportAccuracyCertificationRow>,
    required_outputs: SubscriptionSupportAccuracyCertificationOutputs,
    counter_snapshot: SubscriptionSupportAccuracyCertificationCounterSnapshot,
    generic_certification_digest: String,
    domain_certification_digest: String,
    handoff_digest: String,
    suite_digest: String,
}

impl SubscriptionSupportAccuracyCertificationSuite {
    pub fn from_phase_artifacts_and_lane_evidence(
        evidence_bundle: &SupportCertificationEvidenceBundle,
        generic_report: &SupportGenericCertificationReport,
        domain_bundle: &SupportDomainCertificationBundle,
        handoff_report: &SupportCertificationHandoffReport,
        lane_evidence: &SubscriptionSupportAccuracyLaneEvidenceSet,
    ) -> Result<Self, SupportTrustFailure> {
        let rows = build_required_rows_from_phase_artifacts(
            evidence_bundle,
            generic_report,
            domain_bundle,
            handoff_report,
            lane_evidence,
        )?;
        Self::from_rows_and_phase_artifacts(
            rows,
            evidence_bundle,
            generic_report,
            domain_bundle,
            handoff_report,
            lane_evidence,
        )
    }

    pub(crate) fn from_rows_and_phase_artifacts(
        mut rows: Vec<SubscriptionSupportAccuracyCertificationRow>,
        evidence_bundle: &SupportCertificationEvidenceBundle,
        generic_report: &SupportGenericCertificationReport,
        domain_bundle: &SupportDomainCertificationBundle,
        handoff_report: &SupportCertificationHandoffReport,
        lane_evidence: &SubscriptionSupportAccuracyLaneEvidenceSet,
    ) -> Result<Self, SupportTrustFailure> {
        validate_suite_phase_artifacts(generic_report, domain_bundle, handoff_report)?;
        sort_rows_for_suite(&mut rows);
        validate_required_rows(&rows)?;
        validate_rows_match_phase_artifacts(
            &rows,
            evidence_bundle,
            generic_report,
            domain_bundle,
            handoff_report,
            lane_evidence,
        )?;
        let required_outputs = build_required_outputs(evidence_bundle)?;
        let counter_snapshot = build_suite_counter_snapshot(evidence_bundle, rows.len() as u64)?;
        validate_required_row_count(&counter_snapshot)?;
        assemble_suite(
            rows,
            required_outputs,
            counter_snapshot,
            generic_report,
            domain_bundle,
            handoff_report,
        )
    }

    pub fn suite_name(&self) -> &str {
        &self.suite_name
    }

    pub fn rows(&self) -> &[SubscriptionSupportAccuracyCertificationRow] {
        &self.rows
    }

    pub fn required_outputs(&self) -> &SubscriptionSupportAccuracyCertificationOutputs {
        &self.required_outputs
    }

    pub fn counter_snapshot(&self) -> SubscriptionSupportAccuracyCertificationCounterSnapshot {
        self.counter_snapshot
    }

    pub fn suite_digest(&self) -> &str {
        &self.suite_digest
    }
}

fn validate_suite_phase_artifacts(
    generic_report: &SupportGenericCertificationReport,
    domain_bundle: &SupportDomainCertificationBundle,
    handoff_report: &SupportCertificationHandoffReport,
) -> Result<(), SupportTrustFailure> {
    validate_handoff(handoff_report)?;
    validate_handoff_matches_phase_artifacts(generic_report, domain_bundle, handoff_report)?;
    Ok(())
}

fn sort_rows_for_suite(rows: &mut [SubscriptionSupportAccuracyCertificationRow]) {
    rows.sort_by_key(SubscriptionSupportAccuracyCertificationRow::row_kind);
}

fn build_required_outputs(
    evidence_bundle: &SupportCertificationEvidenceBundle,
) -> Result<SubscriptionSupportAccuracyCertificationOutputs, SupportTrustFailure> {
    SubscriptionSupportAccuracyCertificationOutputs::from_evidence_bundle(evidence_bundle)
}

fn build_suite_counter_snapshot(
    evidence_bundle: &SupportCertificationEvidenceBundle,
    certified_row_count: u64,
) -> Result<SubscriptionSupportAccuracyCertificationCounterSnapshot, SupportTrustFailure> {
    SubscriptionSupportAccuracyCertificationCounterSnapshot::new(
        SubscriptionSupportAccuracyCertificationRowKind::required().len() as u64,
        certified_row_count,
        evidence_bundle
            .counter_snapshot()
            .forbidden_exact_overclaim_count(),
        evidence_bundle.counter_snapshot().global_scan_debt_count(),
    )
}

fn assemble_suite(
    rows: Vec<SubscriptionSupportAccuracyCertificationRow>,
    required_outputs: SubscriptionSupportAccuracyCertificationOutputs,
    counter_snapshot: SubscriptionSupportAccuracyCertificationCounterSnapshot,
    generic_report: &SupportGenericCertificationReport,
    domain_bundle: &SupportDomainCertificationBundle,
    handoff_report: &SupportCertificationHandoffReport,
) -> Result<SubscriptionSupportAccuracyCertificationSuite, SupportTrustFailure> {
    let generic_certification_digest = require_non_empty(
        "generic certification digest",
        generic_report.generic_certification_digest(),
    )?;
    let domain_certification_digest = require_non_empty(
        "domain certification digest",
        domain_bundle.domain_certification_digest(),
    )?;
    let handoff_digest = require_non_empty("handoff digest", handoff_report.handoff_digest())?;
    let mut suite = SubscriptionSupportAccuracyCertificationSuite {
        suite_name: SUBSCRIPTION_SUPPORT_ACCURACY_CERTIFICATION_SUITE_NAME.to_string(),
        rows,
        required_outputs,
        counter_snapshot,
        generic_certification_digest,
        domain_certification_digest,
        handoff_digest,
        suite_digest: String::new(),
    };
    suite.suite_digest = build_suite_digest(&suite)?;
    Ok(suite)
}

fn build_suite_digest(
    suite: &SubscriptionSupportAccuracyCertificationSuite,
) -> Result<String, SupportTrustFailure> {
    let row_digests = suite
        .rows
        .iter()
        .map(|row| row.row_digest())
        .collect::<Vec<_>>();
    stable_digest(&SubscriptionSupportAccuracySuiteDigestBasis {
        suite_name: &suite.suite_name,
        row_digests: &row_digests,
        required_outputs: &suite.required_outputs,
        counter_snapshot: suite.counter_snapshot,
        generic_certification_digest: &suite.generic_certification_digest,
        domain_certification_digest: &suite.domain_certification_digest,
        handoff_digest: &suite.handoff_digest,
    })
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

#[derive(Serialize)]
struct SubscriptionSupportAccuracySuiteDigestBasis<'a> {
    suite_name: &'a str,
    row_digests: &'a [&'a str],
    required_outputs: &'a SubscriptionSupportAccuracyCertificationOutputs,
    counter_snapshot: SubscriptionSupportAccuracyCertificationCounterSnapshot,
    generic_certification_digest: &'a str,
    domain_certification_digest: &'a str,
    handoff_digest: &'a str,
}
