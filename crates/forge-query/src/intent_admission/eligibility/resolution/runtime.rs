use crate::runtime::{
    admit_authoritative_intent_declaration, admit_effect_triggered_intent_declaration,
    ForgeQueryAuthorityLane, ForgeQueryIntentAdmissionDenial, ForgeQueryIntentSourceLane,
};

use super::super::artifact::ForgeQueryIntentAdmissionPreDecisionPosture;
use super::super::facts::{
    ForgeQueryIntentAdmissionAuthorityLaneEligibility, ForgeQueryIntentAdmissionBasisEligibility,
    ForgeQueryIntentAdmissionCapabilityEligibility, ForgeQueryIntentAdmissionInvariantEligibility,
    ForgeQueryIntentAdmissionPolicyEligibility,
    ForgeQueryIntentAdmissionProjectionSourceEligibility,
    ForgeQueryIntentAdmissionRoutingSupportEligibility,
    ForgeQueryIntentAdmissionSourceLaneEligibility, ForgeQueryIntentAdmissionSupportEligibility,
};
use super::super::request::ForgeQueryRawIntentAdmissionRequest;
use super::super::seeds::ForgeQueryAuthoritativeMutationPreflight;
use crate::intent_admission::ForgeQueryIntentAdmissionExecutionSeam;

pub(super) fn resolve_authoritative_runtime_floor_eligibility(
    request: &ForgeQueryRawIntentAdmissionRequest,
) -> EligibilityFacts {
    resolve_implemented_runtime_floor_eligibility(
        request,
        ForgeQueryIntentSourceLane::UserAuthored,
        ForgeQueryAuthorityLane::AuthoritativeTruth,
        admit_authoritative_intent_declaration(
            request
                .runtime_declaration()
                .expect("covered runtime request must preserve declaration"),
        ),
    )
}

pub(super) fn resolve_effect_runtime_floor_eligibility(
    request: &ForgeQueryRawIntentAdmissionRequest,
) -> EligibilityFacts {
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

pub(super) fn resolve_scalar_write_floor_eligibility(
    request: &ForgeQueryRawIntentAdmissionRequest,
) -> EligibilityFacts {
    let seed = request
        .authoritative_mutation_seed()
        .expect("scalar write request must preserve mutation seed");
    let (capability_posture, pre_decision_posture) = match seed.preflight() {
        ForgeQueryAuthoritativeMutationPreflight::Admitted { .. } => (
            ForgeQueryIntentAdmissionCapabilityEligibility::Admitted,
            ForgeQueryIntentAdmissionPreDecisionPosture::Admitted,
        ),
        ForgeQueryAuthoritativeMutationPreflight::BindingDenied(_) => (
            ForgeQueryIntentAdmissionCapabilityEligibility::Violation {
                stage: "existing-truth-binding-admission",
                detail: "scalar-write-existing-truth-binding-denied",
            },
            ForgeQueryIntentAdmissionPreDecisionPosture::Violation {
                stage: "existing-truth-binding-admission",
                message: "scalar-write-existing-truth-binding-denied",
            },
        ),
        ForgeQueryAuthoritativeMutationPreflight::AssertionDenied(_) => (
            ForgeQueryIntentAdmissionCapabilityEligibility::Violation {
                stage: "verified-existing-truth-assertion",
                detail: "scalar-write-verified-existing-truth-assertion-denied",
            },
            ForgeQueryIntentAdmissionPreDecisionPosture::Violation {
                stage: "verified-existing-truth-assertion",
                message: "scalar-write-verified-existing-truth-assertion-denied",
            },
        ),
        ForgeQueryAuthoritativeMutationPreflight::ContinuityDenied(_) => (
            ForgeQueryIntentAdmissionCapabilityEligibility::Violation {
                stage: "mutation-continuity-admission",
                detail: "scalar-write-continuity-denied",
            },
            ForgeQueryIntentAdmissionPreDecisionPosture::Violation {
                stage: "mutation-continuity-admission",
                message: "scalar-write-continuity-denied",
            },
        ),
        ForgeQueryAuthoritativeMutationPreflight::NamingDenied(_) => (
            ForgeQueryIntentAdmissionCapabilityEligibility::Violation {
                stage: "mutation-naming-admission",
                detail: "scalar-write-naming-denied",
            },
            ForgeQueryIntentAdmissionPreDecisionPosture::Violation {
                stage: "mutation-naming-admission",
                message: "scalar-write-naming-denied",
            },
        ),
        ForgeQueryAuthoritativeMutationPreflight::TargetReferenceDenied(_) => (
            ForgeQueryIntentAdmissionCapabilityEligibility::Violation {
                stage: "mutation-target-reference-admission",
                detail: "scalar-write-target-reference-denied",
            },
            ForgeQueryIntentAdmissionPreDecisionPosture::Violation {
                stage: "mutation-target-reference-admission",
                message: "scalar-write-target-reference-denied",
            },
        ),
    };

    (
        ForgeQueryIntentAdmissionSupportEligibility::Admitted,
        capability_posture,
        ForgeQueryIntentAdmissionPolicyEligibility::NotApplicableForRuntimeIntentFloor,
        ForgeQueryIntentAdmissionBasisEligibility::NotApplicableForRuntimeIntentFloor,
        ForgeQueryIntentAdmissionInvariantEligibility::PreExecutionAuthorityRequired,
        ForgeQueryIntentAdmissionProjectionSourceEligibility::NotApplicableForRuntimeIntentFloor,
        ForgeQueryIntentAdmissionRoutingSupportEligibility::CoveredExecutionSeam(
            ForgeQueryIntentAdmissionExecutionSeam::BackendWriteAuthorityRoute,
        ),
        ForgeQueryIntentAdmissionSourceLaneEligibility::NotApplicableNonRuntimeFamily,
        ForgeQueryIntentAdmissionAuthorityLaneEligibility::NotApplicableNonRuntimeFamily,
        pre_decision_posture,
    )
}

pub(super) fn resolve_batch_write_floor_eligibility(
    request: &ForgeQueryRawIntentAdmissionRequest,
) -> EligibilityFacts {
    let commands = request
        .authoritative_mutation_batch_seed()
        .expect("batch write request must preserve mutation batch seed")
        .commands();
    let (capability_posture, pre_decision_posture) = if commands.is_empty() {
        (
            ForgeQueryIntentAdmissionCapabilityEligibility::Violation {
                stage: "mutation-batch-authoring",
                detail: "batch-write-requires-at-least-one-command",
            },
            ForgeQueryIntentAdmissionPreDecisionPosture::Violation {
                stage: "mutation-batch-authoring",
                message: "batch-write-requires-at-least-one-command",
            },
        )
    } else {
        (
            ForgeQueryIntentAdmissionCapabilityEligibility::Admitted,
            ForgeQueryIntentAdmissionPreDecisionPosture::Admitted,
        )
    };

    (
        ForgeQueryIntentAdmissionSupportEligibility::Admitted,
        capability_posture,
        ForgeQueryIntentAdmissionPolicyEligibility::NotApplicableForRuntimeIntentFloor,
        ForgeQueryIntentAdmissionBasisEligibility::NotApplicableForRuntimeIntentFloor,
        ForgeQueryIntentAdmissionInvariantEligibility::PreExecutionAuthorityRequired,
        ForgeQueryIntentAdmissionProjectionSourceEligibility::NotApplicableForRuntimeIntentFloor,
        ForgeQueryIntentAdmissionRoutingSupportEligibility::CoveredExecutionSeam(
            ForgeQueryIntentAdmissionExecutionSeam::BackendWriteAuthorityRoute,
        ),
        ForgeQueryIntentAdmissionSourceLaneEligibility::NotApplicableNonRuntimeFamily,
        ForgeQueryIntentAdmissionAuthorityLaneEligibility::NotApplicableNonRuntimeFamily,
        pre_decision_posture,
    )
}

pub(super) type EligibilityFacts = (
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
);

fn resolve_implemented_runtime_floor_eligibility(
    request: &ForgeQueryRawIntentAdmissionRequest,
    expected_source_lane: ForgeQueryIntentSourceLane,
    expected_target_lane: ForgeQueryAuthorityLane,
    admission: Result<(), ForgeQueryIntentAdmissionDenial>,
) -> EligibilityFacts {
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
