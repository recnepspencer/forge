use std::sync::Arc;

use sha2::{Digest, Sha256};

use super::{
    BridgeSubscriptionReferenceWorkloadDeclaration,
    BridgeSubscriptionReferenceWorkloadLaneArtifactSet,
    BridgeSubscriptionReferenceWorkloadRejection, BridgeSubscriptionReferenceWorkloadRejectionKind,
};
use crate::subscription::certification::{
    BridgeSubscriptionCertificationComparisonReport,
    BridgeSubscriptionCertificationCounterSnapshot, BridgeSubscriptionOfflineAuditOutcomeSummary,
    BridgeSubscriptionOfflineAuditReport, BridgeSubscriptionReferenceWorkloadCoverageReport,
    BridgeSubscriptionReferenceWorkloadLaneReport,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BridgeSubscriptionReferenceWorkloadRequiredCoverageFacet {
    Authoritative,
    Historical,
    BranchLocal,
    Preview,
    TimeOnly,
    AsyncBacked,
    SharedConsumer,
    Restart,
    Replay,
}

impl BridgeSubscriptionReferenceWorkloadRequiredCoverageFacet {
    pub const fn all() -> &'static [Self] {
        &[
            Self::Authoritative,
            Self::Historical,
            Self::BranchLocal,
            Self::Preview,
            Self::TimeOnly,
            Self::AsyncBacked,
            Self::SharedConsumer,
            Self::Restart,
            Self::Replay,
        ]
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Authoritative => "authoritative",
            Self::Historical => "historical",
            Self::BranchLocal => "branch-local",
            Self::Preview => "preview",
            Self::TimeOnly => "time-only",
            Self::AsyncBacked => "async-backed",
            Self::SharedConsumer => "shared-consumer",
            Self::Restart => "restart",
            Self::Replay => "replay",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionReferenceWorkloadCoverageProof {
    lane_artifact_set_digest: Arc<str>,
    coverage_report: BridgeSubscriptionReferenceWorkloadCoverageReport,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionReferenceWorkloadReport {
    manifest_digest: Arc<str>,
    declaration_digest: Arc<str>,
    lane_artifact_set_digest: Arc<str>,
    coverage_proof_digest: Arc<str>,
    fixture_evidence_digest: Arc<str>,
    lane_reports: Vec<BridgeSubscriptionReferenceWorkloadLaneReport>,
    comparison_reports: Vec<BridgeSubscriptionCertificationComparisonReport>,
    offline_audit_report: BridgeSubscriptionOfflineAuditReport,
    outcome_summary: BridgeSubscriptionOfflineAuditOutcomeSummary,
    coverage_report: BridgeSubscriptionReferenceWorkloadCoverageReport,
    counters: BridgeSubscriptionCertificationCounterSnapshot,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionReferenceWorkloadSufficiency {
    report: BridgeSubscriptionReferenceWorkloadReport,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionReferenceWorkloadCoverageProof {
    pub(crate) fn prove(
        lane_artifact_set: BridgeSubscriptionReferenceWorkloadLaneArtifactSet,
    ) -> Result<Self, BridgeSubscriptionReferenceWorkloadRejection> {
        let coverage_report =
            BridgeSubscriptionReferenceWorkloadCoverageReport::from_indexed_lane_and_comparison_reports(
                lane_artifact_set.lane_reports(),
                lane_artifact_set.comparison_reports(),
                lane_artifact_set.comparison_lane_slots(),
            );
        if !coverage_report.first_ship_lane_matrix_covered() {
            return Err(BridgeSubscriptionReferenceWorkloadRejection::new(
                BridgeSubscriptionReferenceWorkloadRejectionKind::CoverageProofRejected,
                None,
                "required first-ship lane matrix is incomplete",
            ));
        }
        if !coverage_report.required_phase_17_facets_covered() {
            return Err(BridgeSubscriptionReferenceWorkloadRejection::new(
                BridgeSubscriptionReferenceWorkloadRejectionKind::CoverageProofRejected,
                None,
                "phase 17 required coverage facets are incomplete",
            ));
        }
        if !coverage_report.required_hostile_lane_set_covered() {
            return Err(BridgeSubscriptionReferenceWorkloadRejection::new(
                BridgeSubscriptionReferenceWorkloadRejectionKind::CoverageProofRejected,
                None,
                "required hostile lane set is incomplete",
            ));
        }
        if !coverage_report.comparison_evidence_complete() {
            return Err(BridgeSubscriptionReferenceWorkloadRejection::new(
                BridgeSubscriptionReferenceWorkloadRejectionKind::CoverageProofRejected,
                None,
                "comparison evidence must be complete before sufficiency seals",
            ));
        }
        if !coverage_report.expected_lane_outcomes_covered() {
            return Err(BridgeSubscriptionReferenceWorkloadRejection::new(
                BridgeSubscriptionReferenceWorkloadRejectionKind::CoverageProofRejected,
                None,
                "lane evidence must match expected outcomes before sufficiency seals",
            ));
        }
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-reference-workload-coverage-proof|lane-artifacts={}|coverage={}",
            lane_artifact_set.digest(),
            coverage_report.digest(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Ok(Self {
            lane_artifact_set_digest: Arc::from(lane_artifact_set.digest()),
            coverage_report,
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-reference-workload-coverage-proof:sha256:{digest:x}"
            )),
        })
    }

    pub fn coverage_report(&self) -> &BridgeSubscriptionReferenceWorkloadCoverageReport {
        &self.coverage_report
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

impl BridgeSubscriptionReferenceWorkloadReport {
    fn from_parts(
        manifest: &crate::subscription::certification::BridgeSubscriptionReferenceWorkloadManifestSealed,
        declaration: &BridgeSubscriptionReferenceWorkloadDeclaration,
        lane_artifact_set: BridgeSubscriptionReferenceWorkloadLaneArtifactSet,
        coverage_report: BridgeSubscriptionReferenceWorkloadCoverageReport,
        coverage_proof_digest: &str,
        fixture_evidence_digest: &str,
    ) -> Self {
        let lane_reports = lane_artifact_set.lane_reports().to_vec();
        let comparison_reports = lane_artifact_set.comparison_reports().to_vec();
        let offline_audit_report = lane_artifact_set.offline_audit_report().clone();
        let outcome_summary = lane_artifact_set.outcome_summary().clone();
        let counters = BridgeSubscriptionCertificationCounterSnapshot::combine(
            lane_reports
                .iter()
                .map(|lane| *lane.counters())
                .chain(comparison_reports.iter().map(|report| *report.counters()))
                .chain([*offline_audit_report.counters()])
                .chain([*coverage_report.counters()])
                .chain([
                    BridgeSubscriptionCertificationCounterSnapshot::from_reference_workload(
                        lane_reports.len(),
                    ),
                ]),
        );
        let lane_digest_basis = lane_reports
            .iter()
            .map(BridgeSubscriptionReferenceWorkloadLaneReport::digest)
            .collect::<Vec<_>>()
            .join(",");
        let comparison_digest_basis = comparison_reports
            .iter()
            .map(BridgeSubscriptionCertificationComparisonReport::digest)
            .collect::<Vec<_>>()
            .join(",");
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-reference-workload-report|manifest={}|declaration={}|lane-artifacts={}|coverage-proof={}|fixture={}|lanes={lane_digest_basis}|comparisons={comparison_digest_basis}|audit={}|outcomes={}|coverage={}|counters={}",
            manifest.digest(),
            declaration.digest(),
            lane_artifact_set.digest(),
            coverage_proof_digest,
            fixture_evidence_digest,
            offline_audit_report.digest(),
            outcome_summary.digest(),
            coverage_report.digest(),
            counters.digest(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            manifest_digest: Arc::from(manifest.digest()),
            declaration_digest: Arc::from(declaration.digest()),
            lane_artifact_set_digest: Arc::from(lane_artifact_set.digest()),
            coverage_proof_digest: Arc::from(coverage_proof_digest),
            fixture_evidence_digest: Arc::from(fixture_evidence_digest),
            lane_reports,
            comparison_reports,
            offline_audit_report,
            outcome_summary,
            coverage_report,
            counters,
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-reference-workload-report:sha256:{digest:x}"
            )),
        }
    }

    pub(crate) fn seal(
        manifest: &crate::subscription::certification::BridgeSubscriptionReferenceWorkloadManifestSealed,
        declaration: &BridgeSubscriptionReferenceWorkloadDeclaration,
        lane_artifact_set: BridgeSubscriptionReferenceWorkloadLaneArtifactSet,
        coverage_proof: &BridgeSubscriptionReferenceWorkloadCoverageProof,
        fixture_evidence_digest: &str,
    ) -> BridgeSubscriptionReferenceWorkloadSufficiency {
        let report = Self::from_parts(
            manifest,
            declaration,
            lane_artifact_set,
            coverage_proof.coverage_report().clone(),
            coverage_proof.digest(),
            fixture_evidence_digest,
        );
        BridgeSubscriptionReferenceWorkloadSufficiency::seal(
            report,
            coverage_proof.digest(),
            fixture_evidence_digest,
        )
    }

    pub(crate) fn run(
        manifest: &crate::subscription::certification::BridgeSubscriptionReferenceWorkloadManifestSealed,
        lane_requests: Vec<
            crate::subscription::certification::BridgeSubscriptionReferenceWorkloadLaneRequest,
        >,
    ) -> Result<Self, BridgeSubscriptionReferenceWorkloadRejection> {
        let declaration =
            BridgeSubscriptionReferenceWorkloadDeclaration::plan(manifest, lane_requests)?;
        let lane_artifact_set =
            BridgeSubscriptionReferenceWorkloadLaneArtifactSet::admit(manifest, &declaration)?;
        let coverage_report =
            BridgeSubscriptionReferenceWorkloadCoverageReport::from_indexed_lane_and_comparison_reports(
                lane_artifact_set.lane_reports(),
                lane_artifact_set.comparison_reports(),
                lane_artifact_set.comparison_lane_slots(),
            );
        Ok(Self::from_parts(
            manifest,
            &declaration,
            lane_artifact_set,
            coverage_report,
            "reference-workload-report-without-phase-17-sufficiency-proof",
            "reference-workload-report-without-bound-fixture-evidence",
        ))
    }

    pub fn manifest_digest(&self) -> &str {
        self.manifest_digest.as_ref()
    }

    pub fn declaration_digest(&self) -> &str {
        self.declaration_digest.as_ref()
    }

    pub fn lane_artifact_set_digest(&self) -> &str {
        self.lane_artifact_set_digest.as_ref()
    }

    pub fn coverage_proof_digest(&self) -> &str {
        self.coverage_proof_digest.as_ref()
    }

    pub fn fixture_evidence_digest(&self) -> &str {
        self.fixture_evidence_digest.as_ref()
    }

    pub fn lane_reports(&self) -> &[BridgeSubscriptionReferenceWorkloadLaneReport] {
        &self.lane_reports
    }

    pub fn comparison_reports(&self) -> &[BridgeSubscriptionCertificationComparisonReport] {
        &self.comparison_reports
    }

    pub fn offline_audit_report(&self) -> &BridgeSubscriptionOfflineAuditReport {
        &self.offline_audit_report
    }

    pub fn outcome_summary(&self) -> &BridgeSubscriptionOfflineAuditOutcomeSummary {
        &self.outcome_summary
    }

    pub fn coverage_report(&self) -> &BridgeSubscriptionReferenceWorkloadCoverageReport {
        &self.coverage_report
    }

    pub fn counters(&self) -> &BridgeSubscriptionCertificationCounterSnapshot {
        &self.counters
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

impl BridgeSubscriptionReferenceWorkloadSufficiency {
    fn seal(
        report: BridgeSubscriptionReferenceWorkloadReport,
        coverage_proof_digest: &str,
        fixture_evidence_digest: &str,
    ) -> Self {
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-reference-workload-sufficiency|report={}|coverage-proof={coverage_proof_digest}|fixture={fixture_evidence_digest}",
            report.digest(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            report,
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-reference-workload-sufficiency:sha256:{digest:x}"
            )),
        }
    }

    pub fn report(&self) -> &BridgeSubscriptionReferenceWorkloadReport {
        &self.report
    }

    pub fn coverage_proof_digest(&self) -> &str {
        self.report.coverage_proof_digest()
    }

    pub fn fixture_evidence_digest(&self) -> &str {
        self.report.fixture_evidence_digest()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
