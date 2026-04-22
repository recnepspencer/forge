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
    BridgeSubscriptionCertificationScratch, BridgeSubscriptionOfflineAuditBundleIndex,
    BridgeSubscriptionOfflineAuditOutcomeSummary, BridgeSubscriptionOfflineAuditReport,
    BridgeSubscriptionReferenceWorkloadCoverageReport,
    BridgeSubscriptionReferenceWorkloadFamilyKind, BridgeSubscriptionReferenceWorkloadLaneKind,
    BridgeSubscriptionReferenceWorkloadLaneReport, BridgeSubscriptionReferenceWorkloadLaneRequest,
    BridgeSubscriptionReferenceWorkloadManifestSealed, BridgeSubscriptionSourceArtifactIndex,
    BridgeSubscriptionSourceArtifactInput, BridgeSubscriptionSourceArtifactKind,
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
        let assembly_plan = super::BridgeSubscriptionCertificationAssemblyPlan::plan(
            manifest,
            &source_artifact_index,
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
        let mut bundle = draft.seal();
        if request.lane_kind() == BridgeSubscriptionReferenceWorkloadLaneKind::BundleInsufficiency {
            let required_field_count = bundle.fields().len() + 1;
            bundle = bundle.with_required_field_count_for_certification(required_field_count);
        }
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

fn lane_comparison_plan(
    lane_kind: BridgeSubscriptionReferenceWorkloadLaneKind,
) -> Result<
    super::BridgeSubscriptionCertificationComparisonPlan,
    BridgeSubscriptionReferenceWorkloadRejection,
> {
    let (relationship, expected_failure_boundary, divergence_axis) = match lane_kind {
        BridgeSubscriptionReferenceWorkloadLaneKind::AuthoritativeLive => (
            BridgeSubscriptionCertificationComparisonRelationship::SemanticEquivalence,
            None,
            None,
        ),
        BridgeSubscriptionReferenceWorkloadLaneKind::CanonicalOrderingHostility => (
            BridgeSubscriptionCertificationComparisonRelationship::SemanticEquivalence,
            None,
            None,
        ),
        BridgeSubscriptionReferenceWorkloadLaneKind::HistoricalBasisReplay
        | BridgeSubscriptionReferenceWorkloadLaneKind::SharedFanout => (
            BridgeSubscriptionCertificationComparisonRelationship::SemanticEquivalence,
            None,
            None,
        ),
        BridgeSubscriptionReferenceWorkloadLaneKind::DiagnosticsTierVariation => (
            BridgeSubscriptionCertificationComparisonRelationship::DiagnosticsOnlyVariation,
            None,
            None,
        ),
        BridgeSubscriptionReferenceWorkloadLaneKind::HostileAdapterVariation => (
            BridgeSubscriptionCertificationComparisonRelationship::ExpectedRejection,
            Some(BridgeSubscriptionCertificationFailureBoundary::MissingRequiredRetainedArtifact),
            None,
        ),
        BridgeSubscriptionReferenceWorkloadLaneKind::HistoricalReplay
        | BridgeSubscriptionReferenceWorkloadLaneKind::RestartResume => (
            BridgeSubscriptionCertificationComparisonRelationship::ExpectedRejection,
            Some(BridgeSubscriptionCertificationFailureBoundary::ReplayMismatch),
            None,
        ),
        BridgeSubscriptionReferenceWorkloadLaneKind::BranchLocal => (
            BridgeSubscriptionCertificationComparisonRelationship::IntentionalDivergence,
            None,
            Some(BridgeSubscriptionCertificationDivergenceAxis::DeclarationFamily),
        ),
        BridgeSubscriptionReferenceWorkloadLaneKind::IncompatibleSharingRejection => (
            BridgeSubscriptionCertificationComparisonRelationship::ExpectedRejection,
            Some(BridgeSubscriptionCertificationFailureBoundary::IllegalSharingReuse),
            None,
        ),
        BridgeSubscriptionReferenceWorkloadLaneKind::StaleCheckpointRejection => (
            BridgeSubscriptionCertificationComparisonRelationship::ExpectedRejection,
            Some(BridgeSubscriptionCertificationFailureBoundary::CheckpointIncompatibility),
            None,
        ),
        BridgeSubscriptionReferenceWorkloadLaneKind::Continuation => (
            BridgeSubscriptionCertificationComparisonRelationship::IntentionalDivergence,
            None,
            Some(BridgeSubscriptionCertificationDivergenceAxis::ContinuationDecision),
        ),
        BridgeSubscriptionReferenceWorkloadLaneKind::DeniedContinuation => (
            BridgeSubscriptionCertificationComparisonRelationship::ExpectedRejection,
            Some(BridgeSubscriptionCertificationFailureBoundary::ContinuationDenialOrAmbiguity),
            None,
        ),
        BridgeSubscriptionReferenceWorkloadLaneKind::StrategyLoweringProvenance => (
            BridgeSubscriptionCertificationComparisonRelationship::IntentionalDivergence,
            None,
            Some(BridgeSubscriptionCertificationDivergenceAxis::StrategyLowering),
        ),
        BridgeSubscriptionReferenceWorkloadLaneKind::BundleInsufficiency => (
            BridgeSubscriptionCertificationComparisonRelationship::BundleCompleteness,
            None,
            None,
        ),
        BridgeSubscriptionReferenceWorkloadLaneKind::PreviewDiscard
        | BridgeSubscriptionReferenceWorkloadLaneKind::PreviewPromotion => (
            BridgeSubscriptionCertificationComparisonRelationship::IntentionalDivergence,
            None,
            Some(BridgeSubscriptionCertificationDivergenceAxis::PreviewOutcome),
        ),
    };
    super::BridgeSubscriptionCertificationComparisonPlan::admit(
        relationship,
        expected_failure_boundary,
        divergence_axis,
    )
    .map_err(|rejection| {
        BridgeSubscriptionReferenceWorkloadRejection::new(
            BridgeSubscriptionReferenceWorkloadRejectionKind::ComparisonPlanRejected,
            Some(lane_kind),
            comparison_plan_rejection_detail(rejection.rejection_kind()),
        )
    })
}

fn lane_source_inputs(
    request: BridgeSubscriptionReferenceWorkloadLaneRequest,
) -> Vec<BridgeSubscriptionSourceArtifactInput> {
    let family = request.family_kind().as_str();
    let lane = request.lane_kind().as_str();
    let strategy = match request.family_kind() {
        BridgeSubscriptionReferenceWorkloadFamilyKind::DetailExact => "exact-field-lens",
        BridgeSubscriptionReferenceWorkloadFamilyKind::CollectionMembership => {
            "collection-membership-index"
        }
    };
    let strategy = if request.lane_kind()
        == BridgeSubscriptionReferenceWorkloadLaneKind::StrategyLoweringProvenance
    {
        "hostile-strategy-lowering-provenance"
    } else {
        strategy
    };
    let fanout_digest = if request.lane_kind()
        == BridgeSubscriptionReferenceWorkloadLaneKind::IncompatibleSharingRejection
    {
        format!("digest:fanout:incompatible:{family}")
    } else {
        format!("digest:fanout:shared:{family}")
    };
    let continuation_digest =
        if request.lane_kind() == BridgeSubscriptionReferenceWorkloadLaneKind::DeniedContinuation {
            format!("digest:continuation:denied:{family}")
        } else {
            format!("digest:continuation:admitted:{family}")
        };
    let checkpoint_digest = if matches!(
        request.lane_kind(),
        BridgeSubscriptionReferenceWorkloadLaneKind::StaleCheckpointRejection
    ) {
        format!("digest:checkpoint:stale:{family}")
    } else {
        format!("digest:checkpoint:fresh:{family}")
    };
    let mut inputs = vec![
        BridgeSubscriptionSourceArtifactInput::new(
            BridgeSubscriptionSourceArtifactKind::LaneIdentity,
            format!("lane:{lane}:{family}"),
            format!("digest:lane:{lane}:{family}"),
        ),
        BridgeSubscriptionSourceArtifactInput::new(
            BridgeSubscriptionSourceArtifactKind::Declaration,
            format!("declaration:{family}"),
            format!("digest:declaration:{family}"),
        ),
        BridgeSubscriptionSourceArtifactInput::new(
            BridgeSubscriptionSourceArtifactKind::AdmittedSubscription,
            format!("admitted:{family}"),
            format!("digest:admitted:{family}"),
        ),
        BridgeSubscriptionSourceArtifactInput::new(
            BridgeSubscriptionSourceArtifactKind::Lifecycle,
            format!("lifecycle:{family}"),
            format!("digest:lifecycle:{family}"),
        ),
        BridgeSubscriptionSourceArtifactInput::new(
            BridgeSubscriptionSourceArtifactKind::BasisBinding,
            format!("basis:{family}"),
            format!("digest:basis:retained:{family}"),
        ),
        BridgeSubscriptionSourceArtifactInput::new(
            BridgeSubscriptionSourceArtifactKind::ActiveDelivery,
            format!("delivery:{family}"),
            format!("digest:delivery:{family}"),
        ),
        BridgeSubscriptionSourceArtifactInput::new(
            BridgeSubscriptionSourceArtifactKind::Fanout,
            format!("fanout:{family}"),
            fanout_digest,
        ),
        BridgeSubscriptionSourceArtifactInput::new(
            BridgeSubscriptionSourceArtifactKind::Continuation,
            format!("continuation:{family}"),
            continuation_digest,
        ),
        BridgeSubscriptionSourceArtifactInput::new(
            BridgeSubscriptionSourceArtifactKind::Checkpoint,
            format!("checkpoint:{family}"),
            checkpoint_digest,
        ),
        BridgeSubscriptionSourceArtifactInput::new(
            BridgeSubscriptionSourceArtifactKind::StrategyLowering,
            format!("strategy:{family}"),
            format!("digest:strategy:{strategy}"),
        ),
    ];
    match request.lane_kind() {
        BridgeSubscriptionReferenceWorkloadLaneKind::HostileAdapterVariation => {
            inputs.push(BridgeSubscriptionSourceArtifactInput::new(
                BridgeSubscriptionSourceArtifactKind::Failure,
                format!("failure:{lane}:{family}"),
                format!("digest:failure:{lane}:{family}"),
            ));
        }
        BridgeSubscriptionReferenceWorkloadLaneKind::HistoricalReplay
        | BridgeSubscriptionReferenceWorkloadLaneKind::RestartResume => {
            inputs.push(BridgeSubscriptionSourceArtifactInput::new(
                BridgeSubscriptionSourceArtifactKind::RetainedReplay,
                format!("replay:{lane}:{family}"),
                format!("digest:replay:{lane}:{family}"),
            ));
        }
        BridgeSubscriptionReferenceWorkloadLaneKind::SharedFanout
        | BridgeSubscriptionReferenceWorkloadLaneKind::IncompatibleSharingRejection => {}
        BridgeSubscriptionReferenceWorkloadLaneKind::StaleCheckpointRejection => {}
        BridgeSubscriptionReferenceWorkloadLaneKind::Continuation => {
            inputs.push(BridgeSubscriptionSourceArtifactInput::new(
                BridgeSubscriptionSourceArtifactKind::Continuation,
                format!("continuation:{lane}:{family}"),
                format!("digest:continuation:{lane}:{family}"),
            ));
        }
        BridgeSubscriptionReferenceWorkloadLaneKind::DeniedContinuation => {}
        BridgeSubscriptionReferenceWorkloadLaneKind::PreviewDiscard
        | BridgeSubscriptionReferenceWorkloadLaneKind::PreviewPromotion => {
            inputs.push(BridgeSubscriptionSourceArtifactInput::new(
                BridgeSubscriptionSourceArtifactKind::Preview,
                format!("preview:{lane}:{family}"),
                format!("digest:preview:{lane}:{family}"),
            ));
        }
        BridgeSubscriptionReferenceWorkloadLaneKind::BranchLocal => {
            inputs.push(BridgeSubscriptionSourceArtifactInput::new(
                BridgeSubscriptionSourceArtifactKind::Declaration,
                format!("branch-scope:{lane}:{family}"),
                format!("digest:branch-scope:{lane}:{family}"),
            ));
        }
        BridgeSubscriptionReferenceWorkloadLaneKind::AuthoritativeLive
        | BridgeSubscriptionReferenceWorkloadLaneKind::HistoricalBasisReplay
        | BridgeSubscriptionReferenceWorkloadLaneKind::DiagnosticsTierVariation
        | BridgeSubscriptionReferenceWorkloadLaneKind::CanonicalOrderingHostility
        | BridgeSubscriptionReferenceWorkloadLaneKind::StrategyLoweringProvenance
        | BridgeSubscriptionReferenceWorkloadLaneKind::BundleInsufficiency => {}
    }
    inputs
}

fn cost_profile_rejection_detail(
    kind: BridgeSubscriptionCertificationCostProfileRejectionKind,
) -> &'static str {
    kind.as_str()
}

fn assembly_rejection_detail(
    kind: BridgeSubscriptionCertificationAssemblyRejectionKind,
) -> &'static str {
    kind.as_str()
}

fn comparison_plan_rejection_detail(
    kind: BridgeSubscriptionCertificationComparisonPlanRejectionKind,
) -> &'static str {
    kind.as_str()
}
