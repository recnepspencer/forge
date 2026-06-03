mod expected_lane_evidence;

use expected_lane_evidence::expected_evidence_for_lane;
use std::sync::Arc;

use sha2::{Digest, Sha256};

use super::{
    BridgeSubscriptionCertificationComparisonOutcome,
    BridgeSubscriptionCertificationComparisonReport,
    BridgeSubscriptionCertificationCounterSnapshot, BridgeSubscriptionCertificationFailureBoundary,
    BridgeSubscriptionReferenceWorkloadFamilyKind, BridgeSubscriptionReferenceWorkloadLaneKind,
    BridgeSubscriptionReferenceWorkloadLaneReport,
};

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
    fn control(lane_report: &BridgeSubscriptionReferenceWorkloadLaneReport) -> Self {
        Self::new(lane_report, None)
    }

    fn compared(
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionReferenceWorkloadCoverageReport {
    lane_kinds: Vec<BridgeSubscriptionReferenceWorkloadLaneKind>,
    family_kinds: Vec<BridgeSubscriptionReferenceWorkloadFamilyKind>,
    lane_coverage_rows: Vec<BridgeSubscriptionReferenceWorkloadLaneCoverageRow>,
    first_ship_lane_matrix_covered: bool,
    multi_family_covered: bool,
    comparison_evidence_complete: bool,
    expected_lane_outcomes_covered: bool,
    counters: BridgeSubscriptionCertificationCounterSnapshot,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionReferenceWorkloadCoverageReport {
    pub(crate) fn from_indexed_lane_and_comparison_reports(
        lane_reports: &[BridgeSubscriptionReferenceWorkloadLaneReport],
        comparison_reports: &[BridgeSubscriptionCertificationComparisonReport],
        comparison_lane_slots: &[usize],
    ) -> Self {
        let mut lane_kinds = lane_reports
            .iter()
            .map(BridgeSubscriptionReferenceWorkloadLaneReport::lane_kind)
            .collect::<Vec<_>>();
        lane_kinds.sort();
        lane_kinds.dedup();

        let mut family_kinds = lane_reports
            .iter()
            .map(BridgeSubscriptionReferenceWorkloadLaneReport::family_kind)
            .collect::<Vec<_>>();
        family_kinds.sort();
        family_kinds.dedup();

        let mut indexed_comparison_reports = vec![None; lane_reports.len()];
        let mut comparison_evidence_complete = true;
        if comparison_reports.len() != comparison_lane_slots.len() {
            comparison_evidence_complete = false;
        }
        for (comparison_report, lane_slot) in comparison_reports
            .iter()
            .zip(comparison_lane_slots.iter().copied())
        {
            if lane_slot >= lane_reports.len()
                || indexed_comparison_reports[lane_slot]
                    .replace(comparison_report)
                    .is_some()
            {
                comparison_evidence_complete = false;
            }
        }
        let lane_coverage_rows = lane_reports
            .iter()
            .enumerate()
            .map(|lane_report| {
                let (lane_slot, lane_report) = lane_report;
                if lane_report.lane_kind()
                    == BridgeSubscriptionReferenceWorkloadLaneKind::AuthoritativeLive
                {
                    if indexed_comparison_reports[lane_slot].is_some() {
                        comparison_evidence_complete = false;
                    }
                    BridgeSubscriptionReferenceWorkloadLaneCoverageRow::control(lane_report)
                } else if let Some(comparison_report) = indexed_comparison_reports[lane_slot] {
                    BridgeSubscriptionReferenceWorkloadLaneCoverageRow::compared(
                        lane_report,
                        comparison_report,
                    )
                } else {
                    comparison_evidence_complete = false;
                    BridgeSubscriptionReferenceWorkloadLaneCoverageRow::control(lane_report)
                }
            })
            .collect::<Vec<_>>();
        comparison_evidence_complete &= lane_coverage_rows
            .iter()
            .filter(|row| {
                row.lane_kind() != BridgeSubscriptionReferenceWorkloadLaneKind::AuthoritativeLive
            })
            .all(|row| row.comparison_outcome().is_some());

        let first_ship_lane_matrix_covered =
            BridgeSubscriptionReferenceWorkloadLaneKind::first_ship_matrix()
                .iter()
                .all(|required| lane_kinds.contains(required));
        let multi_family_covered = family_kinds.len() >= 2;
        let expected_lane_outcomes_covered =
            BridgeSubscriptionReferenceWorkloadLaneKind::first_ship_matrix()
                .iter()
                .all(|required| {
                    let mut matching_rows = lane_coverage_rows
                        .iter()
                        .filter(|row| row.lane_kind() == *required)
                        .peekable();
                    matching_rows.peek().is_some()
                        && matching_rows.all(
                            BridgeSubscriptionReferenceWorkloadLaneCoverageRow::matches_expected_evidence,
                        )
                });
        let counters =
            BridgeSubscriptionCertificationCounterSnapshot::from_reference_workload_coverage_report(
            );
        let lane_basis = lane_kinds
            .iter()
            .map(|lane_kind| lane_kind.as_str())
            .collect::<Vec<_>>()
            .join(",");
        let family_basis = family_kinds
            .iter()
            .map(|family_kind| family_kind.as_str())
            .collect::<Vec<_>>()
            .join(",");
        let coverage_row_basis = lane_coverage_rows
            .iter()
            .map(BridgeSubscriptionReferenceWorkloadLaneCoverageRow::digest)
            .collect::<Vec<_>>()
            .join(",");
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-reference-workload-coverage-report|lanes={lane_basis}|families={family_basis}|rows={coverage_row_basis}|first-ship-matrix={first_ship_lane_matrix_covered}|multi-family={multi_family_covered}|comparison-evidence-complete={comparison_evidence_complete}|expected-lane-outcomes={expected_lane_outcomes_covered}|counters={}",
            counters.digest(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            lane_kinds,
            family_kinds,
            lane_coverage_rows,
            first_ship_lane_matrix_covered,
            multi_family_covered,
            comparison_evidence_complete,
            expected_lane_outcomes_covered,
            counters,
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-reference-workload-coverage-report:sha256:{digest:x}"
            )),
        }
    }

    pub fn lane_kinds(&self) -> &[BridgeSubscriptionReferenceWorkloadLaneKind] {
        &self.lane_kinds
    }

    pub fn family_kinds(&self) -> &[BridgeSubscriptionReferenceWorkloadFamilyKind] {
        &self.family_kinds
    }

    pub fn lane_coverage_rows(&self) -> &[BridgeSubscriptionReferenceWorkloadLaneCoverageRow] {
        &self.lane_coverage_rows
    }

    pub fn first_ship_lane_matrix_covered(&self) -> bool {
        self.first_ship_lane_matrix_covered
    }

    pub fn multi_family_covered(&self) -> bool {
        self.multi_family_covered
    }

    pub fn comparison_evidence_complete(&self) -> bool {
        self.comparison_evidence_complete
    }

    pub fn expected_lane_outcomes_covered(&self) -> bool {
        self.expected_lane_outcomes_covered
    }

    pub fn counters(&self) -> &BridgeSubscriptionCertificationCounterSnapshot {
        &self.counters
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
