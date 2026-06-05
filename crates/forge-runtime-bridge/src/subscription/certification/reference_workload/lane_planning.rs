use crate::subscription::certification::{
    BridgeSubscriptionCertificationAssemblyRejectionKind,
    BridgeSubscriptionCertificationComparisonPlan,
    BridgeSubscriptionCertificationComparisonPlanRejectionKind,
    BridgeSubscriptionCertificationComparisonRelationship,
    BridgeSubscriptionCertificationCostProfileRejectionKind,
    BridgeSubscriptionCertificationDivergenceAxis, BridgeSubscriptionCertificationFailureBoundary,
    BridgeSubscriptionReferenceWorkloadFamilyKind, BridgeSubscriptionReferenceWorkloadLaneKind,
    BridgeSubscriptionReferenceWorkloadLaneRequest, BridgeSubscriptionSourceArtifactEvidence,
    BridgeSubscriptionSourceArtifactInput, BridgeSubscriptionSourceArtifactKind,
    BridgeSubscriptionSourceArtifactRole,
};

use super::{
    BridgeSubscriptionReferenceWorkloadRejection, BridgeSubscriptionReferenceWorkloadRejectionKind,
};

pub(super) fn lane_comparison_plan(
    lane_kind: BridgeSubscriptionReferenceWorkloadLaneKind,
) -> Result<
    BridgeSubscriptionCertificationComparisonPlan,
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
        BridgeSubscriptionReferenceWorkloadLaneKind::TimeOnlyRouting => (
            BridgeSubscriptionCertificationComparisonRelationship::ExpectedRejection,
            Some(BridgeSubscriptionCertificationFailureBoundary::DeliveryFamilyMismatch),
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
        BridgeSubscriptionReferenceWorkloadLaneKind::DivergentSharingRejection => (
            BridgeSubscriptionCertificationComparisonRelationship::ExpectedRejection,
            Some(BridgeSubscriptionCertificationFailureBoundary::IllegalSharingReuse),
            None,
        ),
        BridgeSubscriptionReferenceWorkloadLaneKind::StaleCheckpointRejection => (
            BridgeSubscriptionCertificationComparisonRelationship::ExpectedRejection,
            Some(BridgeSubscriptionCertificationFailureBoundary::CheckpointDivergence),
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
    BridgeSubscriptionCertificationComparisonPlan::admit(
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

pub(super) fn lane_source_inputs(
    request: BridgeSubscriptionReferenceWorkloadLaneRequest,
) -> Vec<BridgeSubscriptionSourceArtifactInput> {
    let strategy_role = match request.family_kind() {
        BridgeSubscriptionReferenceWorkloadFamilyKind::DetailExact => {
            BridgeSubscriptionSourceArtifactRole::ExactFieldLens
        }
        BridgeSubscriptionReferenceWorkloadFamilyKind::CollectionMembership => {
            BridgeSubscriptionSourceArtifactRole::CollectionMembershipIndex
        }
    };
    let strategy_role = if request.lane_kind()
        == BridgeSubscriptionReferenceWorkloadLaneKind::StrategyLoweringProvenance
    {
        BridgeSubscriptionSourceArtifactRole::HostileStrategyLowering
    } else {
        strategy_role
    };
    let fanout_role = if request.lane_kind()
        == BridgeSubscriptionReferenceWorkloadLaneKind::DivergentSharingRejection
    {
        BridgeSubscriptionSourceArtifactRole::DivergentFanout
    } else {
        BridgeSubscriptionSourceArtifactRole::SharedFanout
    };
    let continuation_role =
        if request.lane_kind() == BridgeSubscriptionReferenceWorkloadLaneKind::DeniedContinuation {
            BridgeSubscriptionSourceArtifactRole::DeniedContinuation
        } else {
            BridgeSubscriptionSourceArtifactRole::AdmittedContinuation
        };
    let checkpoint_role = if matches!(
        request.lane_kind(),
        BridgeSubscriptionReferenceWorkloadLaneKind::StaleCheckpointRejection
    ) {
        BridgeSubscriptionSourceArtifactRole::Stale
    } else {
        BridgeSubscriptionSourceArtifactRole::Fresh
    };
    let mut inputs = vec![
        lane_source_artifact(
            BridgeSubscriptionSourceArtifactKind::LaneIdentity,
            request,
            BridgeSubscriptionSourceArtifactRole::Stable,
        ),
        lane_family_source_artifact(
            BridgeSubscriptionSourceArtifactKind::Declaration,
            request.family_kind(),
            BridgeSubscriptionSourceArtifactRole::Stable,
        ),
        lane_family_source_artifact(
            BridgeSubscriptionSourceArtifactKind::AdmittedSubscription,
            request.family_kind(),
            BridgeSubscriptionSourceArtifactRole::Stable,
        ),
        lane_family_source_artifact(
            BridgeSubscriptionSourceArtifactKind::Lifecycle,
            request.family_kind(),
            BridgeSubscriptionSourceArtifactRole::Stable,
        ),
        lane_family_source_artifact(
            BridgeSubscriptionSourceArtifactKind::BasisBinding,
            request.family_kind(),
            BridgeSubscriptionSourceArtifactRole::Stable,
        ),
        lane_family_source_artifact(
            BridgeSubscriptionSourceArtifactKind::ActiveDelivery,
            request.family_kind(),
            BridgeSubscriptionSourceArtifactRole::Stable,
        ),
        lane_family_source_artifact(
            BridgeSubscriptionSourceArtifactKind::Fanout,
            request.family_kind(),
            fanout_role,
        ),
        lane_family_source_artifact(
            BridgeSubscriptionSourceArtifactKind::Continuation,
            request.family_kind(),
            continuation_role,
        ),
        lane_family_source_artifact(
            BridgeSubscriptionSourceArtifactKind::Checkpoint,
            request.family_kind(),
            checkpoint_role,
        ),
        lane_family_source_artifact(
            BridgeSubscriptionSourceArtifactKind::StrategyLowering,
            request.family_kind(),
            strategy_role,
        ),
    ];
    match request.lane_kind() {
        BridgeSubscriptionReferenceWorkloadLaneKind::HostileAdapterVariation => {
            inputs.push(lane_source_artifact(
                BridgeSubscriptionSourceArtifactKind::Failure,
                request,
                BridgeSubscriptionSourceArtifactRole::Hostile,
            ));
        }
        BridgeSubscriptionReferenceWorkloadLaneKind::HistoricalReplay
        | BridgeSubscriptionReferenceWorkloadLaneKind::RestartResume => {
            inputs.push(lane_source_artifact(
                BridgeSubscriptionSourceArtifactKind::RetainedReplay,
                request,
                BridgeSubscriptionSourceArtifactRole::Stable,
            ));
        }
        BridgeSubscriptionReferenceWorkloadLaneKind::TimeOnlyRouting => {
            inputs.push(lane_source_artifact(
                BridgeSubscriptionSourceArtifactKind::DeliveryWindow,
                request,
                BridgeSubscriptionSourceArtifactRole::Divergent,
            ));
        }
        BridgeSubscriptionReferenceWorkloadLaneKind::SharedFanout
        | BridgeSubscriptionReferenceWorkloadLaneKind::DivergentSharingRejection => {}
        BridgeSubscriptionReferenceWorkloadLaneKind::StaleCheckpointRejection => {}
        BridgeSubscriptionReferenceWorkloadLaneKind::Continuation => {
            inputs.push(lane_source_artifact(
                BridgeSubscriptionSourceArtifactKind::Continuation,
                request,
                BridgeSubscriptionSourceArtifactRole::Divergent,
            ));
        }
        BridgeSubscriptionReferenceWorkloadLaneKind::DeniedContinuation => {}
        BridgeSubscriptionReferenceWorkloadLaneKind::PreviewDiscard
        | BridgeSubscriptionReferenceWorkloadLaneKind::PreviewPromotion => {
            inputs.push(lane_source_artifact(
                BridgeSubscriptionSourceArtifactKind::Preview,
                request,
                BridgeSubscriptionSourceArtifactRole::Divergent,
            ));
        }
        BridgeSubscriptionReferenceWorkloadLaneKind::BranchLocal => {
            inputs.push(lane_source_artifact(
                BridgeSubscriptionSourceArtifactKind::Declaration,
                request,
                BridgeSubscriptionSourceArtifactRole::Divergent,
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

fn lane_source_artifact(
    artifact_kind: BridgeSubscriptionSourceArtifactKind,
    request: BridgeSubscriptionReferenceWorkloadLaneRequest,
    role: BridgeSubscriptionSourceArtifactRole,
) -> BridgeSubscriptionSourceArtifactInput {
    BridgeSubscriptionSourceArtifactInput::from_evidence(
        BridgeSubscriptionSourceArtifactEvidence::reference_workload_lane(
            artifact_kind,
            request,
            role,
        ),
    )
}

fn lane_family_source_artifact(
    artifact_kind: BridgeSubscriptionSourceArtifactKind,
    family_kind: BridgeSubscriptionReferenceWorkloadFamilyKind,
    role: BridgeSubscriptionSourceArtifactRole,
) -> BridgeSubscriptionSourceArtifactInput {
    BridgeSubscriptionSourceArtifactInput::from_evidence(
        BridgeSubscriptionSourceArtifactEvidence::reference_workload_family(
            artifact_kind,
            family_kind,
            role,
        ),
    )
}

pub(super) fn cost_profile_rejection_detail(
    kind: BridgeSubscriptionCertificationCostProfileRejectionKind,
) -> &'static str {
    kind.as_str()
}

pub(super) fn assembly_rejection_detail(
    kind: BridgeSubscriptionCertificationAssemblyRejectionKind,
) -> &'static str {
    kind.as_str()
}

fn comparison_plan_rejection_detail(
    kind: BridgeSubscriptionCertificationComparisonPlanRejectionKind,
) -> &'static str {
    kind.as_str()
}
