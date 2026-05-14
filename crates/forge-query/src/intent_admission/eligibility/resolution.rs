use crate::runtime::{
    admit_authoritative_intent_declaration, admit_effect_triggered_intent_declaration,
    ForgeQueryAuthorityLane, ForgeQueryIntentAdmissionDenial, ForgeQueryIntentSourceLane,
};

use super::artifact::ForgeQueryIntentAdmissionPreDecisionPosture;
use super::facts::{
    ForgeQueryIntentAdmissionAuthorityLaneEligibility, ForgeQueryIntentAdmissionBasisEligibility,
    ForgeQueryIntentAdmissionCapabilityEligibility, ForgeQueryIntentAdmissionInvariantEligibility,
    ForgeQueryIntentAdmissionPolicyEligibility,
    ForgeQueryIntentAdmissionProjectionSourceEligibility,
    ForgeQueryIntentAdmissionRoutingSupportEligibility,
    ForgeQueryIntentAdmissionSourceLaneEligibility, ForgeQueryIntentAdmissionSupportEligibility,
};
use super::request::ForgeQueryRawIntentAdmissionRequest;
use crate::intent_admission::{
    ForgeQueryIntentAdmissionCoveredEntrypoint, ForgeQueryIntentAdmissionExecutionSeam,
};

pub(super) fn resolve_eligibility_facts(
    request: &ForgeQueryRawIntentAdmissionRequest,
) -> (
    ForgeQueryIntentAdmissionSupportEligibility,
    ForgeQueryIntentAdmissionCapabilityEligibility,
    ForgeQueryIntentAdmissionPolicyEligibility,
    ForgeQueryIntentAdmissionBasisEligibility,
    ForgeQueryIntentAdmissionInvariantEligibility,
    ForgeQueryIntentAdmissionProjectionSourceEligibility,
    ForgeQueryIntentAdmissionRoutingSupportEligibility,
    ForgeQueryIntentAdmissionSourceLaneEligibility,
    ForgeQueryIntentAdmissionAuthorityLaneEligibility,
    ForgeQueryIntentAdmissionPreDecisionPosture,
) {
    match request.entrypoint() {
        ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteIntent => {
            resolve_implemented_runtime_floor_eligibility(
                request,
                ForgeQueryIntentSourceLane::UserAuthored,
                ForgeQueryAuthorityLane::AuthoritativeTruth,
                admit_authoritative_intent_declaration(request.declaration()),
            )
        }
        ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteNextEffectWriteIntent => {
            resolve_implemented_runtime_floor_eligibility(
                request,
                ForgeQueryIntentSourceLane::EffectTriggered,
                ForgeQueryAuthorityLane::AuthoritativeTruth,
                admit_effect_triggered_intent_declaration(request.declaration()),
            )
        }
        ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteReadNeighborDeferred => {
            resolve_deferred_neighbor_eligibility("read-execution-neighbor-deferred-until-covered")
        }
        ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteInspectionNeighborDeferred => {
            resolve_deferred_neighbor_eligibility(
                "inspection-materialization-neighbor-deferred-until-covered",
            )
        }
    }
}

fn resolve_implemented_runtime_floor_eligibility(
    request: &ForgeQueryRawIntentAdmissionRequest,
    expected_source_lane: ForgeQueryIntentSourceLane,
    expected_target_lane: ForgeQueryAuthorityLane,
    admission: Result<(), ForgeQueryIntentAdmissionDenial>,
) -> (
    ForgeQueryIntentAdmissionSupportEligibility,
    ForgeQueryIntentAdmissionCapabilityEligibility,
    ForgeQueryIntentAdmissionPolicyEligibility,
    ForgeQueryIntentAdmissionBasisEligibility,
    ForgeQueryIntentAdmissionInvariantEligibility,
    ForgeQueryIntentAdmissionProjectionSourceEligibility,
    ForgeQueryIntentAdmissionRoutingSupportEligibility,
    ForgeQueryIntentAdmissionSourceLaneEligibility,
    ForgeQueryIntentAdmissionAuthorityLaneEligibility,
    ForgeQueryIntentAdmissionPreDecisionPosture,
) {
    let source_lane_posture = lane_source_posture(request, expected_source_lane);
    let authority_lane_posture = lane_authority_posture(request, expected_target_lane);
    let capability_posture = match admission {
        Ok(()) => ForgeQueryIntentAdmissionCapabilityEligibility::Admitted,
        Err(denial) => capability_violation_for_denial(request, denial.stage()),
    };
    let pre_decision_posture = match capability_posture {
        ForgeQueryIntentAdmissionCapabilityEligibility::Admitted => {
            ForgeQueryIntentAdmissionPreDecisionPosture::Admitted
        }
        ForgeQueryIntentAdmissionCapabilityEligibility::Violation { stage, detail } => {
            ForgeQueryIntentAdmissionPreDecisionPosture::Violation {
                stage,
                message: detail,
            }
        }
    };

    (
        ForgeQueryIntentAdmissionSupportEligibility::Admitted,
        capability_posture,
        ForgeQueryIntentAdmissionPolicyEligibility::NotApplicableForRuntimeIntentFloor,
        ForgeQueryIntentAdmissionBasisEligibility::NotApplicableForRuntimeIntentFloor,
        ForgeQueryIntentAdmissionInvariantEligibility::PreExecutionAuthorityRequired,
        ForgeQueryIntentAdmissionProjectionSourceEligibility::NotApplicableForRuntimeIntentFloor,
        ForgeQueryIntentAdmissionRoutingSupportEligibility::CoveredExecutionSeam(
            ForgeQueryIntentAdmissionExecutionSeam::BackendIntentAuthorityRoute,
        ),
        source_lane_posture,
        authority_lane_posture,
        pre_decision_posture,
    )
}

fn resolve_deferred_neighbor_eligibility(
    detail: &'static str,
) -> (
    ForgeQueryIntentAdmissionSupportEligibility,
    ForgeQueryIntentAdmissionCapabilityEligibility,
    ForgeQueryIntentAdmissionPolicyEligibility,
    ForgeQueryIntentAdmissionBasisEligibility,
    ForgeQueryIntentAdmissionInvariantEligibility,
    ForgeQueryIntentAdmissionProjectionSourceEligibility,
    ForgeQueryIntentAdmissionRoutingSupportEligibility,
    ForgeQueryIntentAdmissionSourceLaneEligibility,
    ForgeQueryIntentAdmissionAuthorityLaneEligibility,
    ForgeQueryIntentAdmissionPreDecisionPosture,
) {
    (
        ForgeQueryIntentAdmissionSupportEligibility::Deferred(detail),
        ForgeQueryIntentAdmissionCapabilityEligibility::Admitted,
        ForgeQueryIntentAdmissionPolicyEligibility::DeferredNeighbor(detail),
        ForgeQueryIntentAdmissionBasisEligibility::DeferredNeighbor(detail),
        ForgeQueryIntentAdmissionInvariantEligibility::DeferredNeighbor(detail),
        ForgeQueryIntentAdmissionProjectionSourceEligibility::DeferredNeighbor(detail),
        ForgeQueryIntentAdmissionRoutingSupportEligibility::DeferredNeighbor(detail),
        ForgeQueryIntentAdmissionSourceLaneEligibility::DeferredNeighbor(detail),
        ForgeQueryIntentAdmissionAuthorityLaneEligibility::DeferredNeighbor(detail),
        ForgeQueryIntentAdmissionPreDecisionPosture::Deferred {
            stage: "support-deferred",
            message: detail,
        },
    )
}

fn lane_source_posture(
    request: &ForgeQueryRawIntentAdmissionRequest,
    expected: ForgeQueryIntentSourceLane,
) -> ForgeQueryIntentAdmissionSourceLaneEligibility {
    let actual = request.declaration().source_lane();
    if actual == expected {
        ForgeQueryIntentAdmissionSourceLaneEligibility::MatchesExpected(expected)
    } else {
        ForgeQueryIntentAdmissionSourceLaneEligibility::Mismatch { expected, actual }
    }
}

fn lane_authority_posture(
    request: &ForgeQueryRawIntentAdmissionRequest,
    expected: ForgeQueryAuthorityLane,
) -> ForgeQueryIntentAdmissionAuthorityLaneEligibility {
    let actual = request.declaration().target_lane();
    if actual == expected {
        ForgeQueryIntentAdmissionAuthorityLaneEligibility::MatchesExpected(expected)
    } else {
        ForgeQueryIntentAdmissionAuthorityLaneEligibility::Mismatch { expected, actual }
    }
}

fn capability_violation_for_denial(
    request: &ForgeQueryRawIntentAdmissionRequest,
    stage: &'static str,
) -> ForgeQueryIntentAdmissionCapabilityEligibility {
    let detail = match stage {
        "source-lane-admission" => match request.declaration().source_lane() {
            ForgeQueryIntentSourceLane::EffectTriggered => {
                "covered-runtime-entrypoint-rejects-effect-triggered-source-lane"
            }
            ForgeQueryIntentSourceLane::PreviewLocal => {
                "covered-runtime-entrypoint-rejects-preview-local-source-lane"
            }
            ForgeQueryIntentSourceLane::BranchLocal => {
                "covered-runtime-entrypoint-rejects-branch-local-source-lane"
            }
            ForgeQueryIntentSourceLane::DerivedRuntime => {
                "covered-runtime-entrypoint-rejects-derived-runtime-source-lane"
            }
            ForgeQueryIntentSourceLane::UserAuthored => {
                "declared-source-lane-is-not-admitted-for-this-covered-entrypoint"
            }
        },
        "authority-admission" => match request.declaration().target_lane() {
            ForgeQueryAuthorityLane::BranchLocalTruth => {
                "covered-runtime-entrypoint-rejects-branch-local-truth-target"
            }
            ForgeQueryAuthorityLane::PreviewTruth => {
                "covered-runtime-entrypoint-rejects-preview-truth-target"
            }
            ForgeQueryAuthorityLane::DerivedRuntimeState => {
                "covered-runtime-entrypoint-rejects-derived-runtime-state-target"
            }
            ForgeQueryAuthorityLane::EffectDeliveryState => {
                "covered-runtime-entrypoint-rejects-effect-delivery-state-target"
            }
            ForgeQueryAuthorityLane::PendingWriteIntent => {
                "covered-runtime-entrypoint-rejects-pending-write-intent-target"
            }
            ForgeQueryAuthorityLane::BridgeExternalState => {
                "covered-runtime-entrypoint-rejects-bridge-external-state-target"
            }
            ForgeQueryAuthorityLane::TemporalExecutionState => {
                "covered-runtime-entrypoint-rejects-temporal-execution-state-target"
            }
            ForgeQueryAuthorityLane::AsyncResourceState => {
                "covered-runtime-entrypoint-rejects-async-resource-state-target"
            }
            ForgeQueryAuthorityLane::AuthoritativeTruth => {
                "declared-authority-lane-is-not-admitted-for-this-covered-entrypoint"
            }
        },
        _ => "runtime-intent-floor-capability-violation",
    };
    ForgeQueryIntentAdmissionCapabilityEligibility::Violation { stage, detail }
}
