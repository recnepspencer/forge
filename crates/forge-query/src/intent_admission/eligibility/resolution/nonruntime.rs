use crate::basis_lifecycle::{evaluate_basis_observation_eligibility, DeniedBasisCapabilityKind};
use crate::projection_consumption::{
    evaluate_projection_consumption_eligibility, ProjectionConsumptionEligibility,
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

pub(super) fn resolve_basis_observation_eligibility(
    request: &ForgeQueryRawIntentAdmissionRequest,
) -> EligibilityFacts {
    let normalized = request
        .basis_observation()
        .expect("basis observation request must preserve normalized basis intent")
        .clone();
    match evaluate_basis_observation_eligibility(normalized) {
        Ok(_) => admitted_basis_observation_facts(),
        Err(denial) => denied_basis_observation_facts(match denial.denial_kind() {
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
            DeniedBasisCapabilityKind::BridgeAuthorityMismatch => "basis-bridge-authority-mismatch",
            DeniedBasisCapabilityKind::RelationalAuthorityMismatch => {
                "basis-relational-authority-mismatch"
            }
            DeniedBasisCapabilityKind::SignalObservationMissing => {
                "basis-signal-observation-missing"
            }
            DeniedBasisCapabilityKind::RuntimeSnapshotStale => "basis-runtime-snapshot-stale",
            DeniedBasisCapabilityKind::DurableOverclaim => "basis-durable-overclaim",
            DeniedBasisCapabilityKind::StoreBackedDeferred => "basis-store-backed-deferred",
        }),
    }
}

pub(super) fn resolve_projection_consumption_eligibility(
    request: &ForgeQueryRawIntentAdmissionRequest,
) -> EligibilityFacts {
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
            no_execution_handoff_projection(),
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
            no_execution_handoff_projection(),
            ForgeQueryIntentAdmissionSourceLaneEligibility::NotApplicableNonRuntimeFamily,
            ForgeQueryIntentAdmissionAuthorityLaneEligibility::NotApplicableNonRuntimeFamily,
            ForgeQueryIntentAdmissionPreDecisionPosture::Admitted,
        ),
        ProjectionConsumptionEligibility::Deferred(_) => deferred_projection_facts(),
        ProjectionConsumptionEligibility::Denied(_)
        | ProjectionConsumptionEligibility::SourceMismatch(_) => violated_projection_facts(),
    }
}

pub(super) fn resolve_deferred_neighbor_eligibility(detail: &'static str) -> EligibilityFacts {
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

type EligibilityFacts = (
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

fn admitted_basis_observation_facts() -> EligibilityFacts {
    (
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
    )
}

fn denied_basis_observation_facts(detail: &'static str) -> EligibilityFacts {
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

fn deferred_projection_facts() -> EligibilityFacts {
    (
        ForgeQueryIntentAdmissionSupportEligibility::Deferred("projection-consumption-deferred"),
        ForgeQueryIntentAdmissionCapabilityEligibility::Admitted,
        ForgeQueryIntentAdmissionPolicyEligibility::DeferredNeighbor(
            "projection-consumption-deferred",
        ),
        ForgeQueryIntentAdmissionBasisEligibility::DeferredNeighbor(
            "projection-consumption-deferred",
        ),
        ForgeQueryIntentAdmissionInvariantEligibility::DeferredNeighbor(
            "projection-consumption-deferred",
        ),
        ForgeQueryIntentAdmissionProjectionSourceEligibility::DeferredNeighbor(
            "projection-consumption-deferred",
        ),
        no_execution_handoff_projection(),
        ForgeQueryIntentAdmissionSourceLaneEligibility::NotApplicableNonRuntimeFamily,
        ForgeQueryIntentAdmissionAuthorityLaneEligibility::NotApplicableNonRuntimeFamily,
        ForgeQueryIntentAdmissionPreDecisionPosture::Deferred {
            stage: "support-deferred",
            message: "projection-consumption-deferred",
        },
    )
}

fn violated_projection_facts() -> EligibilityFacts {
    (
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
        no_execution_handoff_projection(),
        ForgeQueryIntentAdmissionSourceLaneEligibility::NotApplicableNonRuntimeFamily,
        ForgeQueryIntentAdmissionAuthorityLaneEligibility::NotApplicableNonRuntimeFamily,
        ForgeQueryIntentAdmissionPreDecisionPosture::Violation {
            stage: "projection-consumption-eligibility",
            message: "projection-consumption-violation",
        },
    )
}

fn no_execution_handoff_projection() -> ForgeQueryIntentAdmissionRoutingSupportEligibility {
    ForgeQueryIntentAdmissionRoutingSupportEligibility::NoExecutionHandoff(
        "projection-consumption-admitted-plan-binds-contract-without-query-execution-handoff",
    )
}
