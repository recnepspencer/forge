mod coverage_row;
mod expected_lane_evidence;
use std::sync::Arc;

use sha2::{Digest, Sha256};

use super::{
    BridgeSubscriptionCertificationComparisonReport,
    BridgeSubscriptionCertificationCounterSnapshot, BridgeSubscriptionReferenceWorkloadFamilyKind,
    BridgeSubscriptionReferenceWorkloadLaneKind, BridgeSubscriptionReferenceWorkloadLaneReport,
    BridgeSubscriptionReferenceWorkloadRequiredCoverageFacet,
};

pub use coverage_row::{
    BridgeSubscriptionReferenceWorkloadLaneCoverageRole,
    BridgeSubscriptionReferenceWorkloadLaneCoverageRow,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionReferenceWorkloadCoverageReport {
    lane_kinds: Vec<BridgeSubscriptionReferenceWorkloadLaneKind>,
    family_kinds: Vec<BridgeSubscriptionReferenceWorkloadFamilyKind>,
    covered_required_facets: Vec<BridgeSubscriptionReferenceWorkloadRequiredCoverageFacet>,
    lane_coverage_rows: Vec<BridgeSubscriptionReferenceWorkloadLaneCoverageRow>,
    first_ship_lane_matrix_covered: bool,
    required_phase_17_facets_covered: bool,
    required_hostile_lane_set_covered: bool,
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
        let mut covered_required_facets =
            BridgeSubscriptionReferenceWorkloadRequiredCoverageFacet::all()
                .iter()
                .copied()
                .filter(|facet| facet_is_covered(*facet, &lane_kinds))
                .collect::<Vec<_>>();
        covered_required_facets.sort();
        covered_required_facets.dedup();
        let required_phase_17_facets_covered = covered_required_facets.len()
            == BridgeSubscriptionReferenceWorkloadRequiredCoverageFacet::all().len();
        let required_hostile_lane_set_covered = [
            BridgeSubscriptionReferenceWorkloadLaneKind::HostileAdapterVariation,
            BridgeSubscriptionReferenceWorkloadLaneKind::DivergentSharingRejection,
            BridgeSubscriptionReferenceWorkloadLaneKind::StaleCheckpointRejection,
            BridgeSubscriptionReferenceWorkloadLaneKind::DeniedContinuation,
            BridgeSubscriptionReferenceWorkloadLaneKind::BundleInsufficiency,
        ]
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
        let facet_basis = covered_required_facets
            .iter()
            .map(|facet| facet.as_str())
            .collect::<Vec<_>>()
            .join(",");
        let coverage_row_basis = lane_coverage_rows
            .iter()
            .map(BridgeSubscriptionReferenceWorkloadLaneCoverageRow::digest)
            .collect::<Vec<_>>()
            .join(",");
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-reference-workload-coverage-report|lanes={lane_basis}|families={family_basis}|facets={facet_basis}|rows={coverage_row_basis}|first-ship-matrix={first_ship_lane_matrix_covered}|required-phase-17-facets={required_phase_17_facets_covered}|required-hostile-lanes={required_hostile_lane_set_covered}|multi-family={multi_family_covered}|comparison-evidence-complete={comparison_evidence_complete}|expected-lane-outcomes={expected_lane_outcomes_covered}|counters={}",
            counters.digest(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            lane_kinds,
            family_kinds,
            covered_required_facets,
            lane_coverage_rows,
            first_ship_lane_matrix_covered,
            required_phase_17_facets_covered,
            required_hostile_lane_set_covered,
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

    pub fn covered_required_facets(
        &self,
    ) -> &[BridgeSubscriptionReferenceWorkloadRequiredCoverageFacet] {
        &self.covered_required_facets
    }

    pub fn lane_coverage_rows(&self) -> &[BridgeSubscriptionReferenceWorkloadLaneCoverageRow] {
        &self.lane_coverage_rows
    }

    pub fn first_ship_lane_matrix_covered(&self) -> bool {
        self.first_ship_lane_matrix_covered
    }

    pub fn required_phase_17_facets_covered(&self) -> bool {
        self.required_phase_17_facets_covered
    }

    pub fn required_hostile_lane_set_covered(&self) -> bool {
        self.required_hostile_lane_set_covered
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

fn facet_is_covered(
    facet: BridgeSubscriptionReferenceWorkloadRequiredCoverageFacet,
    lane_kinds: &[BridgeSubscriptionReferenceWorkloadLaneKind],
) -> bool {
    use BridgeSubscriptionReferenceWorkloadLaneKind as Lane;
    match facet {
        BridgeSubscriptionReferenceWorkloadRequiredCoverageFacet::Authoritative => {
            lane_kinds.contains(&Lane::AuthoritativeLive)
        }
        BridgeSubscriptionReferenceWorkloadRequiredCoverageFacet::Historical => {
            lane_kinds.contains(&Lane::HistoricalReplay)
                || lane_kinds.contains(&Lane::HistoricalBasisReplay)
        }
        BridgeSubscriptionReferenceWorkloadRequiredCoverageFacet::BranchLocal => {
            lane_kinds.contains(&Lane::BranchLocal)
        }
        BridgeSubscriptionReferenceWorkloadRequiredCoverageFacet::Preview => {
            lane_kinds.contains(&Lane::PreviewDiscard)
                && lane_kinds.contains(&Lane::PreviewPromotion)
        }
        BridgeSubscriptionReferenceWorkloadRequiredCoverageFacet::TimeOnly => {
            lane_kinds.contains(&Lane::TimeOnlyRouting)
        }
        BridgeSubscriptionReferenceWorkloadRequiredCoverageFacet::AsyncBacked => {
            lane_kinds.contains(&Lane::Continuation)
                || lane_kinds.contains(&Lane::DeniedContinuation)
        }
        BridgeSubscriptionReferenceWorkloadRequiredCoverageFacet::SharedConsumer => {
            lane_kinds.contains(&Lane::SharedFanout)
                || lane_kinds.contains(&Lane::DivergentSharingRejection)
        }
        BridgeSubscriptionReferenceWorkloadRequiredCoverageFacet::Restart => {
            lane_kinds.contains(&Lane::RestartResume)
                || lane_kinds.contains(&Lane::StaleCheckpointRejection)
        }
        BridgeSubscriptionReferenceWorkloadRequiredCoverageFacet::Replay => {
            lane_kinds.contains(&Lane::HistoricalReplay)
        }
    }
}
