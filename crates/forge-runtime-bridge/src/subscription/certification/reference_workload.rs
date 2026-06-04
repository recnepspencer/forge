mod lane_planning;

use lane_planning::{
    assembly_rejection_detail, cost_profile_rejection_detail, lane_comparison_plan,
    lane_source_inputs,
};
use std::sync::Arc;

use sha2::{Digest, Sha256};

use super::{
    BridgeSubscriptionCertificationAssemblyRejectionKind,
    BridgeSubscriptionCertificationBundleDraft, BridgeSubscriptionCertificationBundleSealed,
    BridgeSubscriptionCertificationComparisonPlanRejectionKind,
    BridgeSubscriptionCertificationComparisonRelationship,
    BridgeSubscriptionCertificationComparisonReport, BridgeSubscriptionCertificationCostProfile,
    BridgeSubscriptionCertificationCostProfileRejectionKind,
    BridgeSubscriptionCertificationCounterSnapshot, BridgeSubscriptionCertificationDensityPosture,
    BridgeSubscriptionCertificationDivergenceAxis, BridgeSubscriptionCertificationFailureBoundary,
    BridgeSubscriptionCertificationFieldExpectation, BridgeSubscriptionCertificationScratch,
    BridgeSubscriptionOfflineAuditBundleIndex, BridgeSubscriptionOfflineAuditOutcomeSummary,
    BridgeSubscriptionOfflineAuditReport, BridgeSubscriptionReferenceWorkloadCoverageReport,
    BridgeSubscriptionReferenceWorkloadFamilyKind, BridgeSubscriptionReferenceWorkloadLaneKind,
    BridgeSubscriptionReferenceWorkloadLaneReport, BridgeSubscriptionReferenceWorkloadLaneRequest,
    BridgeSubscriptionReferenceWorkloadManifestSealed, BridgeSubscriptionSourceArtifactEvidence,
    BridgeSubscriptionSourceArtifactIndex, BridgeSubscriptionSourceArtifactInput,
    BridgeSubscriptionSourceArtifactKind, BridgeSubscriptionSourceArtifactRole,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeSubscriptionReferenceWorkloadRejectionKind {
    InsufficientLaneSet,
    MissingAuthoritativeControlLane,
    LaneNotDeclaredByManifest,
    CostProfileRejected,
    BundleAssemblyRejected,
    ComparisonPlanRejected,
    OfflineAuditRejected,
}

impl BridgeSubscriptionReferenceWorkloadRejectionKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InsufficientLaneSet => "insufficient_lane_set",
            Self::MissingAuthoritativeControlLane => "missing_authoritative_control_lane",
            Self::LaneNotDeclaredByManifest => "lane_not_declared_by_manifest",
            Self::CostProfileRejected => "cost_profile_rejected",
            Self::BundleAssemblyRejected => "bundle_assembly_rejected",
            Self::ComparisonPlanRejected => "comparison_plan_rejected",
            Self::OfflineAuditRejected => "offline_audit_rejected",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionReferenceWorkloadRejection {
    rejection_kind: BridgeSubscriptionReferenceWorkloadRejectionKind,
    lane_kind: Option<BridgeSubscriptionReferenceWorkloadLaneKind>,
    detail: Arc<str>,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionReferenceWorkloadRejection {
    fn new(
        rejection_kind: BridgeSubscriptionReferenceWorkloadRejectionKind,
        lane_kind: Option<BridgeSubscriptionReferenceWorkloadLaneKind>,
        detail: impl Into<Arc<str>>,
    ) -> Self {
        let detail = detail.into();
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-reference-workload-rejection|kind={}|lane={}|detail={detail}",
            rejection_kind.as_str(),
            lane_kind
                .map(BridgeSubscriptionReferenceWorkloadLaneKind::as_str)
                .unwrap_or("none"),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            rejection_kind,
            lane_kind,
            detail,
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-reference-workload-rejection:sha256:{digest:x}"
            )),
        }
    }

    pub fn rejection_kind(&self) -> BridgeSubscriptionReferenceWorkloadRejectionKind {
        self.rejection_kind
    }

    pub fn lane_kind(&self) -> Option<BridgeSubscriptionReferenceWorkloadLaneKind> {
        self.lane_kind
    }

    pub fn detail(&self) -> &str {
        self.detail.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionReferenceWorkloadReport {
    manifest_digest: Arc<str>,
    lane_reports: Vec<BridgeSubscriptionReferenceWorkloadLaneReport>,
    comparison_reports: Vec<BridgeSubscriptionCertificationComparisonReport>,
    offline_audit_report: BridgeSubscriptionOfflineAuditReport,
    outcome_summary: BridgeSubscriptionOfflineAuditOutcomeSummary,
    coverage_report: BridgeSubscriptionReferenceWorkloadCoverageReport,
    counters: BridgeSubscriptionCertificationCounterSnapshot,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

struct BuiltLane {
    request: BridgeSubscriptionReferenceWorkloadLaneRequest,
    bundle: BridgeSubscriptionCertificationBundleSealed,
    report: BridgeSubscriptionReferenceWorkloadLaneReport,
}

impl BridgeSubscriptionReferenceWorkloadReport {
    pub(crate) fn run(
        manifest: &BridgeSubscriptionReferenceWorkloadManifestSealed,
        lane_requests: Vec<BridgeSubscriptionReferenceWorkloadLaneRequest>,
    ) -> Result<Self, BridgeSubscriptionReferenceWorkloadRejection> {
        let mut lane_requests = lane_requests;
        lane_requests.sort_by(|left, right| {
            left.lane_kind()
                .cmp(&right.lane_kind())
                .then_with(|| left.family_kind().cmp(&right.family_kind()))
        });
        lane_requests.dedup();
        if lane_requests.len() < 2 {
            return Err(BridgeSubscriptionReferenceWorkloadRejection::new(
                BridgeSubscriptionReferenceWorkloadRejectionKind::InsufficientLaneSet,
                None,
                "at least two unique lanes are required for cross-lane certification",
            ));
        }
        Self::validate_manifest_lanes(manifest, &lane_requests)?;
        if !lane_requests.iter().any(|lane| {
            lane.lane_kind() == BridgeSubscriptionReferenceWorkloadLaneKind::AuthoritativeLive
        }) {
            return Err(BridgeSubscriptionReferenceWorkloadRejection::new(
                BridgeSubscriptionReferenceWorkloadRejectionKind::MissingAuthoritativeControlLane,
                None,
                "reference workloads require the authoritative live lane as the comparison control",
            ));
        }

        let mut built_lanes = Vec::new();
        for request in lane_requests {
            built_lanes.push(Self::build_lane(manifest, request)?);
        }

        let control_slot = built_lanes
            .iter()
            .position(|lane| {
                lane.request.lane_kind()
                    == BridgeSubscriptionReferenceWorkloadLaneKind::AuthoritativeLive
            })
            .expect("validated reference workload requests include authoritative live control");
        let control_bundle = &built_lanes[control_slot].bundle;
        let mut comparison_reports = Vec::new();
        let mut comparison_lane_slots = Vec::new();
        for (slot, lane) in built_lanes
            .iter()
            .enumerate()
            .filter(|(slot, _lane)| *slot != control_slot)
        {
            let comparison_plan = lane_comparison_plan(lane.request.lane_kind())?;
            comparison_lane_slots.push(slot);
            comparison_reports.push(BridgeSubscriptionCertificationComparisonReport::compare(
                comparison_plan,
                control_bundle,
                &lane.bundle,
            ));
        }

        let bundle_index = BridgeSubscriptionOfflineAuditBundleIndex::build(
            built_lanes.iter().map(|lane| &lane.bundle).collect(),
        );
        let audit_plan = super::BridgeSubscriptionOfflineAuditPlan::admit(
            &bundle_index,
            comparison_reports.iter().collect(),
            false,
            false,
        )
        .map_err(|rejection| {
            BridgeSubscriptionReferenceWorkloadRejection::new(
                BridgeSubscriptionReferenceWorkloadRejectionKind::OfflineAuditRejected,
                None,
                rejection.rejection_kind().as_str(),
            )
        })?;
        let offline_audit_report = super::BridgeSubscriptionOfflineAuditReport::audit(audit_plan);
        let outcome_summary = offline_audit_report.outcome_summary().clone();
        let lane_reports = built_lanes
            .into_iter()
            .map(|lane| lane.report)
            .collect::<Vec<_>>();
        let coverage_report =
            BridgeSubscriptionReferenceWorkloadCoverageReport::from_indexed_lane_and_comparison_reports(
                &lane_reports,
                &comparison_reports,
                &comparison_lane_slots,
            );
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
            "bridge-subscription-reference-workload-report|manifest={}|lanes={lane_digest_basis}|comparisons={comparison_digest_basis}|audit={}|outcomes={}|coverage={}|counters={}",
            manifest.digest(),
            offline_audit_report.digest(),
            outcome_summary.digest(),
            coverage_report.digest(),
            counters.digest(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Ok(Self {
            manifest_digest: Arc::from(manifest.digest()),
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
        })
    }

    fn validate_manifest_lanes(
        manifest: &BridgeSubscriptionReferenceWorkloadManifestSealed,
        lane_requests: &[BridgeSubscriptionReferenceWorkloadLaneRequest],
    ) -> Result<(), BridgeSubscriptionReferenceWorkloadRejection> {
        for request in lane_requests {
            if !manifest
                .lane_ids()
                .iter()
                .any(|lane_id| lane_id.as_ref() == request.lane_kind().as_str())
            {
                return Err(BridgeSubscriptionReferenceWorkloadRejection::new(
                    BridgeSubscriptionReferenceWorkloadRejectionKind::LaneNotDeclaredByManifest,
                    Some(request.lane_kind()),
                    "requested lane is absent from the sealed manifest",
                ));
            }
        }
        Ok(())
    }

    fn build_lane(
        manifest: &BridgeSubscriptionReferenceWorkloadManifestSealed,
        request: BridgeSubscriptionReferenceWorkloadLaneRequest,
    ) -> Result<BuiltLane, BridgeSubscriptionReferenceWorkloadRejection> {
        let source_artifact_index =
            BridgeSubscriptionSourceArtifactIndex::build(lane_source_inputs(request));
        let assembly_plan =
            super::BridgeSubscriptionCertificationAssemblyPlan::plan_with_field_expectation(
                manifest,
                &source_artifact_index,
                field_expectation_for_lane(request.lane_kind()),
            );
        let cost_profile = BridgeSubscriptionCertificationCostProfile::admit(
            BridgeSubscriptionCertificationDensityPosture::SparseCertificationWindow,
            16,
            16,
            32,
            matches!(
                request.lane_kind(),
                BridgeSubscriptionReferenceWorkloadLaneKind::DiagnosticsTierVariation
            ),
        )
        .map_err(|rejection| {
            BridgeSubscriptionReferenceWorkloadRejection::new(
                BridgeSubscriptionReferenceWorkloadRejectionKind::CostProfileRejected,
                Some(request.lane_kind()),
                cost_profile_rejection_detail(rejection.rejection_kind()),
            )
        })?;
        let scratch = BridgeSubscriptionCertificationScratch::prepare(&cost_profile);
        let draft = BridgeSubscriptionCertificationBundleDraft::assemble(
            assembly_plan,
            cost_profile,
            scratch,
        )
        .map_err(|rejection| {
            BridgeSubscriptionReferenceWorkloadRejection::new(
                BridgeSubscriptionReferenceWorkloadRejectionKind::BundleAssemblyRejected,
                Some(request.lane_kind()),
                assembly_rejection_detail(rejection.rejection_kind()),
            )
        })?;
        let bundle = draft.seal();
        let report = BridgeSubscriptionReferenceWorkloadLaneReport::from_bundle(
            request,
            &source_artifact_index,
            &bundle,
        );
        Ok(BuiltLane {
            request,
            bundle,
            report,
        })
    }

    pub fn manifest_digest(&self) -> &str {
        self.manifest_digest.as_ref()
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

fn field_expectation_for_lane(
    lane_kind: BridgeSubscriptionReferenceWorkloadLaneKind,
) -> BridgeSubscriptionCertificationFieldExpectation {
    match lane_kind {
        BridgeSubscriptionReferenceWorkloadLaneKind::BundleInsufficiency => {
            BridgeSubscriptionCertificationFieldExpectation::RetainedArtifactCompletenessRequirement
        }
        BridgeSubscriptionReferenceWorkloadLaneKind::AuthoritativeLive
        | BridgeSubscriptionReferenceWorkloadLaneKind::HistoricalReplay
        | BridgeSubscriptionReferenceWorkloadLaneKind::HistoricalBasisReplay
        | BridgeSubscriptionReferenceWorkloadLaneKind::RestartResume
        | BridgeSubscriptionReferenceWorkloadLaneKind::BranchLocal
        | BridgeSubscriptionReferenceWorkloadLaneKind::SharedFanout
        | BridgeSubscriptionReferenceWorkloadLaneKind::DivergentSharingRejection
        | BridgeSubscriptionReferenceWorkloadLaneKind::DiagnosticsTierVariation
        | BridgeSubscriptionReferenceWorkloadLaneKind::HostileAdapterVariation
        | BridgeSubscriptionReferenceWorkloadLaneKind::StaleCheckpointRejection
        | BridgeSubscriptionReferenceWorkloadLaneKind::Continuation
        | BridgeSubscriptionReferenceWorkloadLaneKind::DeniedContinuation
        | BridgeSubscriptionReferenceWorkloadLaneKind::StrategyLoweringProvenance
        | BridgeSubscriptionReferenceWorkloadLaneKind::PreviewDiscard
        | BridgeSubscriptionReferenceWorkloadLaneKind::PreviewPromotion
        | BridgeSubscriptionReferenceWorkloadLaneKind::CanonicalOrderingHostility => {
            BridgeSubscriptionCertificationFieldExpectation::CompleteReferenceBundle
        }
    }
}
