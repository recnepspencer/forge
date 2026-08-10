use std::sync::Arc;

use sha2::{Digest, Sha256};

use super::super::{
    BridgeSubscriptionCertificationComparisonOutcome,
    BridgeSubscriptionCertificationComparisonReport,
    BridgeSubscriptionCertificationFailureBoundary, BridgeSubscriptionReferenceWorkloadFamilyKind,
    BridgeSubscriptionReferenceWorkloadLaneKind, BridgeSubscriptionReferenceWorkloadLaneReport,
};
use super::expected_lane_evidence::expected_evidence_for_lane;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeSubscriptionReferenceWorkloadLaneCoverageRole {
    Control,
    Compared,
}

impl BridgeSubscriptionReferenceWorkloadLaneCoverageRole {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Control => "control",
            Self::Compared => "compared",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionReferenceWorkloadLaneCoverageRow {
    lane_kind: BridgeSubscriptionReferenceWorkloadLaneKind,
    family_kind: BridgeSubscriptionReferenceWorkloadFamilyKind,
    coverage_role: BridgeSubscriptionReferenceWorkloadLaneCoverageRole,
    lane_report_digest: Arc<str>,
    comparison_report_digest: Arc<str>,
    comparison_outcome: Option<BridgeSubscriptionCertificationComparisonOutcome>,
    primary_failure_boundary: Option<BridgeSubscriptionCertificationFailureBoundary>,
    expected_outcome: Option<BridgeSubscriptionCertificationComparisonOutcome>,
    expected_primary_failure_boundary: Option<BridgeSubscriptionCertificationFailureBoundary>,
    matches_expected_evidence: bool,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionReferenceWorkloadLaneCoverageRow {
    pub(super) fn control(lane_report: &BridgeSubscriptionReferenceWorkloadLaneReport) -> Self {
        Self::new(lane_report, None)
    }

    pub(super) fn compared(
        lane_report: &BridgeSubscriptionReferenceWorkloadLaneReport,
        comparison_report: &BridgeSubscriptionCertificationComparisonReport,
    ) -> Self {
        Self::new(lane_report, Some(comparison_report))
    }

    fn new(
        lane_report: &BridgeSubscriptionReferenceWorkloadLaneReport,
        comparison_report: Option<&BridgeSubscriptionCertificationComparisonReport>,
    ) -> Self {
        let expected = expected_evidence_for_lane(lane_report.lane_kind());
        let coverage_role = if comparison_report.is_some() {
            BridgeSubscriptionReferenceWorkloadLaneCoverageRole::Compared
        } else {
            BridgeSubscriptionReferenceWorkloadLaneCoverageRole::Control
        };
        let comparison_outcome =
            comparison_report.map(BridgeSubscriptionCertificationComparisonReport::outcome);
        let primary_failure_boundary = comparison_report
            .and_then(BridgeSubscriptionCertificationComparisonReport::primary_failure_boundary);
        let comparison_report_digest = Arc::<str>::from(
            comparison_report
                .map(BridgeSubscriptionCertificationComparisonReport::digest)
                .unwrap_or("control-lane-no-comparison-report"),
        );
        let matches_expected_evidence = expected
            .map(|expected| {
                comparison_outcome == expected.outcome
                    && primary_failure_boundary == expected.primary_failure_boundary
            })
            .unwrap_or_else(|| comparison_report.is_none());
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-reference-workload-lane-coverage-row|lane={}|family={}|role={}|lane-report={}|comparison-report={}|outcome={}|primary={}|expected-outcome={}|expected-primary={}|matches={matches_expected_evidence}",
            lane_report.lane_kind().as_str(),
            lane_report.family_kind().as_str(),
            coverage_role.as_str(),
            lane_report.digest(),
            comparison_report_digest.as_ref(),
            comparison_outcome
                .map(BridgeSubscriptionCertificationComparisonOutcome::as_str)
                .unwrap_or("control"),
            primary_failure_boundary
                .map(BridgeSubscriptionCertificationFailureBoundary::as_str)
                .unwrap_or("none"),
            expected
                .and_then(|expected| expected.outcome)
                .map(BridgeSubscriptionCertificationComparisonOutcome::as_str)
                .unwrap_or("control"),
            expected
                .and_then(|expected| expected.primary_failure_boundary)
                .map(BridgeSubscriptionCertificationFailureBoundary::as_str)
                .unwrap_or("none"),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            lane_kind: lane_report.lane_kind(),
            family_kind: lane_report.family_kind(),
            coverage_role,
            lane_report_digest: Arc::from(lane_report.digest()),
            comparison_report_digest,
            comparison_outcome,
            primary_failure_boundary,
            expected_outcome: expected.and_then(|expected| expected.outcome),
            expected_primary_failure_boundary: expected
                .and_then(|expected| expected.primary_failure_boundary),
            matches_expected_evidence,
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-reference-workload-lane-coverage-row:sha256:{digest:x}"
            )),
        }
    }

    pub fn lane_kind(&self) -> BridgeSubscriptionReferenceWorkloadLaneKind {
        self.lane_kind
    }

    pub fn family_kind(&self) -> BridgeSubscriptionReferenceWorkloadFamilyKind {
        self.family_kind
    }

    pub fn coverage_role(&self) -> BridgeSubscriptionReferenceWorkloadLaneCoverageRole {
        self.coverage_role
    }

    pub fn lane_report_digest(&self) -> &str {
        self.lane_report_digest.as_ref()
    }

    pub fn comparison_report_digest(&self) -> &str {
        self.comparison_report_digest.as_ref()
    }

    pub fn comparison_outcome(&self) -> Option<BridgeSubscriptionCertificationComparisonOutcome> {
        self.comparison_outcome
    }

    pub fn primary_failure_boundary(
        &self,
    ) -> Option<BridgeSubscriptionCertificationFailureBoundary> {
        self.primary_failure_boundary
    }

    pub fn expected_outcome(&self) -> Option<BridgeSubscriptionCertificationComparisonOutcome> {
        self.expected_outcome
    }

    pub fn expected_primary_failure_boundary(
        &self,
    ) -> Option<BridgeSubscriptionCertificationFailureBoundary> {
        self.expected_primary_failure_boundary
    }

    pub fn matches_expected_evidence(&self) -> bool {
        self.matches_expected_evidence
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
