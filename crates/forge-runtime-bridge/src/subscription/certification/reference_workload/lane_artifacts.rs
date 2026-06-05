use std::sync::Arc;

use sha2::{Digest, Sha256};

use super::lane_planning::{
    assembly_rejection_detail, cost_profile_rejection_detail, lane_comparison_plan,
    lane_source_inputs,
};
use super::{
    BridgeSubscriptionReferenceWorkloadDeclaration, BridgeSubscriptionReferenceWorkloadRejection,
    BridgeSubscriptionReferenceWorkloadRejectionKind,
};
use crate::subscription::certification::{
    BridgeSubscriptionCertificationBundleDraft, BridgeSubscriptionCertificationBundleSealed,
    BridgeSubscriptionCertificationComparisonReport, BridgeSubscriptionCertificationCostProfile,
    BridgeSubscriptionCertificationCounterSnapshot, BridgeSubscriptionCertificationDensityPosture,
    BridgeSubscriptionCertificationFieldExpectation, BridgeSubscriptionCertificationScratch,
    BridgeSubscriptionOfflineAuditBundleIndex, BridgeSubscriptionOfflineAuditOutcomeSummary,
    BridgeSubscriptionOfflineAuditReport, BridgeSubscriptionReferenceWorkloadLaneKind,
    BridgeSubscriptionReferenceWorkloadLaneReport,
    BridgeSubscriptionReferenceWorkloadManifestSealed, BridgeSubscriptionSourceArtifactIndex,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionReferenceWorkloadLaneArtifactSet {
    manifest_digest: Arc<str>,
    declaration_digest: Arc<str>,
    lane_reports: Vec<BridgeSubscriptionReferenceWorkloadLaneReport>,
    comparison_reports: Vec<BridgeSubscriptionCertificationComparisonReport>,
    comparison_lane_slots: Vec<usize>,
    offline_audit_report: BridgeSubscriptionOfflineAuditReport,
    outcome_summary: BridgeSubscriptionOfflineAuditOutcomeSummary,
    counters: BridgeSubscriptionCertificationCounterSnapshot,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

struct BuiltLane {
    request: crate::subscription::certification::BridgeSubscriptionReferenceWorkloadLaneRequest,
    bundle: BridgeSubscriptionCertificationBundleSealed,
    report: BridgeSubscriptionReferenceWorkloadLaneReport,
}

impl BridgeSubscriptionReferenceWorkloadLaneArtifactSet {
    pub(crate) fn admit(
        manifest: &BridgeSubscriptionReferenceWorkloadManifestSealed,
        declaration: &BridgeSubscriptionReferenceWorkloadDeclaration,
    ) -> Result<Self, BridgeSubscriptionReferenceWorkloadRejection> {
        let mut built_lanes = Vec::new();
        for request in declaration.lane_requests() {
            built_lanes.push(build_lane(manifest, *request)?);
        }

        let control_slot = built_lanes
            .iter()
            .position(|lane| {
                lane.request.lane_kind()
                    == BridgeSubscriptionReferenceWorkloadLaneKind::AuthoritativeLive
            })
            .expect("declaration phase proved authoritative live control lane");
        let control_bundle = &built_lanes[control_slot].bundle;

        let mut comparison_reports = Vec::new();
        let mut comparison_lane_slots = Vec::new();
        for (slot, lane) in built_lanes
            .iter()
            .enumerate()
            .filter(|(slot, _)| *slot != control_slot)
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
        let audit_plan =
            crate::subscription::certification::BridgeSubscriptionOfflineAuditPlan::admit(
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
        let offline_audit_report =
            crate::subscription::certification::BridgeSubscriptionOfflineAuditReport::audit(
                audit_plan,
            );
        let outcome_summary = offline_audit_report.outcome_summary().clone();
        let lane_reports = built_lanes
            .into_iter()
            .map(|lane| lane.report)
            .collect::<Vec<_>>();
        let counters = BridgeSubscriptionCertificationCounterSnapshot::combine(
            lane_reports
                .iter()
                .map(|lane| *lane.counters())
                .chain(comparison_reports.iter().map(|report| *report.counters()))
                .chain([*offline_audit_report.counters()])
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
            "bridge-subscription-reference-workload-lane-artifact-set|manifest={}|declaration={}|lanes={lane_digest_basis}|comparisons={comparison_digest_basis}|audit={}|outcomes={}|counters={}",
            manifest.digest(),
            declaration.digest(),
            offline_audit_report.digest(),
            outcome_summary.digest(),
            counters.digest(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Ok(Self {
            manifest_digest: Arc::from(manifest.digest()),
            declaration_digest: Arc::from(declaration.digest()),
            lane_reports,
            comparison_reports,
            comparison_lane_slots,
            offline_audit_report,
            outcome_summary,
            counters,
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-reference-workload-lane-artifact-set:sha256:{digest:x}"
            )),
        })
    }

    pub fn lane_reports(&self) -> &[BridgeSubscriptionReferenceWorkloadLaneReport] {
        &self.lane_reports
    }

    pub fn comparison_reports(&self) -> &[BridgeSubscriptionCertificationComparisonReport] {
        &self.comparison_reports
    }

    pub(crate) fn comparison_lane_slots(&self) -> &[usize] {
        &self.comparison_lane_slots
    }

    pub fn offline_audit_report(&self) -> &BridgeSubscriptionOfflineAuditReport {
        &self.offline_audit_report
    }

    pub fn outcome_summary(&self) -> &BridgeSubscriptionOfflineAuditOutcomeSummary {
        &self.outcome_summary
    }

    pub fn counters(&self) -> &BridgeSubscriptionCertificationCounterSnapshot {
        &self.counters
    }

    pub fn manifest_digest(&self) -> &str {
        self.manifest_digest.as_ref()
    }

    pub fn declaration_digest(&self) -> &str {
        self.declaration_digest.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

fn build_lane(
    manifest: &BridgeSubscriptionReferenceWorkloadManifestSealed,
    request: crate::subscription::certification::BridgeSubscriptionReferenceWorkloadLaneRequest,
) -> Result<BuiltLane, BridgeSubscriptionReferenceWorkloadRejection> {
    let source_artifact_index =
        BridgeSubscriptionSourceArtifactIndex::build(lane_source_inputs(request));
    let assembly_plan =
        crate::subscription::certification::BridgeSubscriptionCertificationAssemblyPlan::plan_with_field_expectation(
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
    let draft =
        BridgeSubscriptionCertificationBundleDraft::assemble(assembly_plan, cost_profile, scratch)
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
        | BridgeSubscriptionReferenceWorkloadLaneKind::TimeOnlyRouting
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
