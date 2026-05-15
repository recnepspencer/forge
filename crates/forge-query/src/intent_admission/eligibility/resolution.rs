use crate::basis_lifecycle::{evaluate_basis_observation_eligibility, DeniedBasisCapabilityKind};
use crate::projection_consumption::{
    evaluate_projection_consumption_eligibility, ProjectionConsumptionEligibility,
};
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
                admit_authoritative_intent_declaration(
                    request
                        .runtime_declaration()
                        .expect("runtime floor request must preserve declaration"),
                ),
            )
        }
        ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteNextEffectWriteIntent => {
            resolve_implemented_runtime_floor_eligibility(
                request,
                ForgeQueryIntentSourceLane::EffectTriggered,
                ForgeQueryAuthorityLane::AuthoritativeTruth,
                admit_effect_triggered_intent_declaration(
                    request
                        .runtime_declaration()
                        .expect("effect floor request must preserve declaration"),
                ),
            )
        }
        ForgeQueryIntentAdmissionCoveredEntrypoint::BasisObservation => {
            resolve_basis_observation_eligibility(request)
        }
        ForgeQueryIntentAdmissionCoveredEntrypoint::ProjectionConsumption => {
            resolve_projection_consumption_eligibility(request)
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

fn resolve_basis_observation_eligibility(
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
    let normalized = request
        .basis_observation()
        .expect("basis observation request must preserve normalized basis intent")
        .clone();
    match evaluate_basis_observation_eligibility(normalized) {
        Ok(_) => (
            ForgeQueryIntentAdmissionSupportEligibility::Admitted,
            ForgeQueryIntentAdmissionCapabilityEligibility::Admitted,
            ForgeQueryIntentAdmissionPolicyEligibility::NotApplicableForRuntimeIntentFloor,
            ForgeQueryIntentAdmissionBasisEligibility::ObservationLifecycleAdmitted,
            ForgeQueryIntentAdmissionInvariantEligibility::PreExecutionAuthorityRequired,
            ForgeQueryIntentAdmissionProjectionSourceEligibility::NotApplicableForRuntimeIntentFloor,
            ForgeQueryIntentAdmissionRoutingSupportEligibility::NoExecutionHandoff(
                "basis-observation-admitted-plan-scopes-to-lower-runtime-evidence-without-query-execution-handoff",
            ),
            ForgeQueryIntentAdmissionSourceLaneEligibility::NotApplicableNonRuntimeFamily,
            ForgeQueryIntentAdmissionAuthorityLaneEligibility::NotApplicableNonRuntimeFamily,
            ForgeQueryIntentAdmissionPreDecisionPosture::Admitted,
        ),
        Err(denial) => {
            let detail = match denial.denial_kind() {
                DeniedBasisCapabilityKind::PolicyMasked => "basis-policy-masked",
                DeniedBasisCapabilityKind::PreviewDrifted => "basis-preview-drifted",
                DeniedBasisCapabilityKind::TenantMismatched => "basis-tenant-schema-mismatch",
                DeniedBasisCapabilityKind::LowerRuntimeBindingMissing => {
                    "basis-lower-runtime-binding-required"
                }
                DeniedBasisCapabilityKind::LowerRuntimeCapabilityUnsupported => {
                    "basis-unsupported-lane"
                }
                DeniedBasisCapabilityKind::Inaccessible => "basis-visibility-denied",
                DeniedBasisCapabilityKind::SchemaIncompatible => "basis-schema-incompatible",
                DeniedBasisCapabilityKind::OperationIneligible => "basis-operation-ineligible",
                DeniedBasisCapabilityKind::HistoricalReplayUnsupported => {
                    "basis-historical-replay-unsupported"
                }
                DeniedBasisCapabilityKind::BridgeAuthorityMismatch => {
                    "basis-bridge-authority-mismatch"
                }
                DeniedBasisCapabilityKind::RelationalAuthorityMismatch => {
                    "basis-relational-authority-mismatch"
                }
                DeniedBasisCapabilityKind::SignalObservationMissing => {
                    "basis-signal-observation-missing"
                }
                DeniedBasisCapabilityKind::RuntimeSnapshotStale => "basis-runtime-snapshot-stale",
                DeniedBasisCapabilityKind::DurableOverclaim => "basis-durable-overclaim",
                DeniedBasisCapabilityKind::StoreBackedDeferred => "basis-store-backed-deferred",
            };
            (
                ForgeQueryIntentAdmissionSupportEligibility::Admitted,
                ForgeQueryIntentAdmissionCapabilityEligibility::Violation {
                    stage: "basis-observation-eligibility",
                    detail,
                },
                ForgeQueryIntentAdmissionPolicyEligibility::NotApplicableForRuntimeIntentFloor,
                ForgeQueryIntentAdmissionBasisEligibility::ObservationLifecycleViolation(detail),
                ForgeQueryIntentAdmissionInvariantEligibility::PreExecutionAuthorityRequired,
                ForgeQueryIntentAdmissionProjectionSourceEligibility::NotApplicableForRuntimeIntentFloor,
                ForgeQueryIntentAdmissionRoutingSupportEligibility::NoExecutionHandoff(
                    "basis-observation-admitted-plan-scopes-to-lower-runtime-evidence-without-query-execution-handoff",
                ),
                ForgeQueryIntentAdmissionSourceLaneEligibility::NotApplicableNonRuntimeFamily,
                ForgeQueryIntentAdmissionAuthorityLaneEligibility::NotApplicableNonRuntimeFamily,
                ForgeQueryIntentAdmissionPreDecisionPosture::Violation {
                    stage: "basis-observation-eligibility",
                    message: detail,
                },
            )
        }
    }
}

fn resolve_projection_consumption_eligibility(
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
    let declaration = request
        .projection_consumption_declaration()
        .expect("projection request must preserve declaration");
    match evaluate_projection_consumption_eligibility(declaration) {
        ProjectionConsumptionEligibility::Admitted(_) => (
            ForgeQueryIntentAdmissionSupportEligibility::Admitted,
            ForgeQueryIntentAdmissionCapabilityEligibility::Admitted,
            ForgeQueryIntentAdmissionPolicyEligibility::NotApplicableForRuntimeIntentFloor,
            ForgeQueryIntentAdmissionBasisEligibility::NotApplicableForRuntimeIntentFloor,
            ForgeQueryIntentAdmissionInvariantEligibility::PreExecutionAuthorityRequired,
            ForgeQueryIntentAdmissionProjectionSourceEligibility::ProjectionConsumptionAdmitted,
            ForgeQueryIntentAdmissionRoutingSupportEligibility::NoExecutionHandoff(
                "projection-consumption-admitted-plan-binds-contract-without-query-execution-handoff",
            ),
            ForgeQueryIntentAdmissionSourceLaneEligibility::NotApplicableNonRuntimeFamily,
            ForgeQueryIntentAdmissionAuthorityLaneEligibility::NotApplicableNonRuntimeFamily,
            ForgeQueryIntentAdmissionPreDecisionPosture::Admitted,
        ),
        ProjectionConsumptionEligibility::AdmittedWithWarnings(_, _) => (
            ForgeQueryIntentAdmissionSupportEligibility::Admitted,
            ForgeQueryIntentAdmissionCapabilityEligibility::Admitted,
            ForgeQueryIntentAdmissionPolicyEligibility::NotApplicableForRuntimeIntentFloor,
            ForgeQueryIntentAdmissionBasisEligibility::NotApplicableForRuntimeIntentFloor,
            ForgeQueryIntentAdmissionInvariantEligibility::PreExecutionAuthorityRequired,
            ForgeQueryIntentAdmissionProjectionSourceEligibility::ProjectionConsumptionAdmittedWithWarnings(
                "projection-consumption-warning-bearing-admission",
            ),
            ForgeQueryIntentAdmissionRoutingSupportEligibility::NoExecutionHandoff(
                "projection-consumption-admitted-plan-binds-contract-without-query-execution-handoff",
            ),
            ForgeQueryIntentAdmissionSourceLaneEligibility::NotApplicableNonRuntimeFamily,
            ForgeQueryIntentAdmissionAuthorityLaneEligibility::NotApplicableNonRuntimeFamily,
            ForgeQueryIntentAdmissionPreDecisionPosture::Admitted,
        ),
        ProjectionConsumptionEligibility::Deferred(_) => (
            ForgeQueryIntentAdmissionSupportEligibility::Deferred("projection-consumption-deferred"),
            ForgeQueryIntentAdmissionCapabilityEligibility::Admitted,
            ForgeQueryIntentAdmissionPolicyEligibility::DeferredNeighbor("projection-consumption-deferred"),
            ForgeQueryIntentAdmissionBasisEligibility::DeferredNeighbor("projection-consumption-deferred"),
            ForgeQueryIntentAdmissionInvariantEligibility::DeferredNeighbor("projection-consumption-deferred"),
            ForgeQueryIntentAdmissionProjectionSourceEligibility::DeferredNeighbor(
                "projection-consumption-deferred",
            ),
            ForgeQueryIntentAdmissionRoutingSupportEligibility::NoExecutionHandoff(
                "projection-consumption-admitted-plan-binds-contract-without-query-execution-handoff",
            ),
            ForgeQueryIntentAdmissionSourceLaneEligibility::NotApplicableNonRuntimeFamily,
            ForgeQueryIntentAdmissionAuthorityLaneEligibility::NotApplicableNonRuntimeFamily,
            ForgeQueryIntentAdmissionPreDecisionPosture::Deferred {
                stage: "support-deferred",
                message: "projection-consumption-deferred",
            },
        ),
        ProjectionConsumptionEligibility::Denied(_)
        | ProjectionConsumptionEligibility::SourceMismatch(_) => (
            ForgeQueryIntentAdmissionSupportEligibility::Admitted,
            ForgeQueryIntentAdmissionCapabilityEligibility::Violation {
                stage: "projection-consumption-eligibility",
                detail: "projection-consumption-violation",
            },
            ForgeQueryIntentAdmissionPolicyEligibility::NotApplicableForRuntimeIntentFloor,
            ForgeQueryIntentAdmissionBasisEligibility::NotApplicableForRuntimeIntentFloor,
            ForgeQueryIntentAdmissionInvariantEligibility::PreExecutionAuthorityRequired,
            ForgeQueryIntentAdmissionProjectionSourceEligibility::ProjectionConsumptionViolation(
                "projection-consumption-violation",
            ),
            ForgeQueryIntentAdmissionRoutingSupportEligibility::NoExecutionHandoff(
                "projection-consumption-admitted-plan-binds-contract-without-query-execution-handoff",
            ),
            ForgeQueryIntentAdmissionSourceLaneEligibility::NotApplicableNonRuntimeFamily,
            ForgeQueryIntentAdmissionAuthorityLaneEligibility::NotApplicableNonRuntimeFamily,
            ForgeQueryIntentAdmissionPreDecisionPosture::Violation {
                stage: "projection-consumption-eligibility",
                message: "projection-consumption-violation",
            },
        ),
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
    let actual = request
        .runtime_declaration()
        .expect("lane source posture is runtime-only")
        .source_lane();
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
    let actual = request
        .runtime_declaration()
        .expect("lane authority posture is runtime-only")
        .target_lane();
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
        "source-lane-admission" => match request
            .runtime_declaration()
            .expect("capability violation source lane is runtime-only")
            .source_lane()
        {
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
        "authority-admission" => match request
            .runtime_declaration()
            .expect("capability violation authority lane is runtime-only")
            .target_lane()
        {
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
