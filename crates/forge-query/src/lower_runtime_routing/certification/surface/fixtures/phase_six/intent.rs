use crate::evidence_identity::{
    ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
};
use crate::intent_admission::certification_runtime;
use crate::lower_runtime_routing::{
    ForgeQueryLowerRuntimeAuthorityOwner, ForgeQueryLowerRuntimeBoundaryEnvelope,
    ForgeQueryLowerRuntimeBoundaryExecutionReceipt, ForgeQueryLowerRuntimeCapabilityEligibility,
    ForgeQueryLowerRuntimeCapabilityRequest, ForgeQueryLowerRuntimeRouteKind,
    ForgeQueryLowerRuntimeRoutePlan, ForgeQueryLowerRuntimeSeamKey,
};
use crate::runtime::{ForgeQueryIntentDeclaration, ForgeQueryIntentInput};

use super::super::{ForgeQueryLowerRuntimeRepresentativeEvidenceSource, RepresentativeArtifacts};

pub(crate) fn representative_runtime_intent_authority_row() -> RepresentativeArtifacts {
    let receipt = certification_intent_receipt();
    route_planned_row(
        ForgeQueryLowerRuntimeSeamKey::RuntimeIntentAuthorityAdapter,
        "Runtime intent authority seam",
        ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
            .field_value(
                ForgeQueryEvidenceTag::new("strategy"),
                receipt.strategy_identity(),
            )
            .field_value(
                ForgeQueryEvidenceTag::new("outcome"),
                receipt.outcome_digest(),
            )
            .field_value(
                ForgeQueryEvidenceTag::new("provenance"),
                receipt.execution_provenance_chain_digest(),
            )
            .seal(),
        ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
            .field_value(
                ForgeQueryEvidenceTag::new("execution_binding"),
                receipt.execution_binding_digest(),
            )
            .seal(),
        ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
            .field_value(
                ForgeQueryEvidenceTag::new("execution_provenance"),
                receipt.execution_provenance_chain_digest(),
            )
            .seal(),
    )
}

pub(crate) fn representative_intent_runtime_execution_row() -> RepresentativeArtifacts {
    let receipt = certification_intent_receipt();
    route_planned_row(
        ForgeQueryLowerRuntimeSeamKey::IntentRuntimeExecution,
        "Intent runtime execution",
        ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
            .field_shape(ForgeQueryEvidenceTag::new("intent"), receipt.intent_name())
            .field_evidence_identity(
                ForgeQueryEvidenceTag::new("commit"),
                receipt.commit_evidence_identity(),
            )
            .field_evidence_identity(
                ForgeQueryEvidenceTag::new("snapshot"),
                receipt.snapshot_evidence_identity(),
            )
            .field_value(
                ForgeQueryEvidenceTag::new("receipt"),
                receipt.receipt_digest(),
            )
            .seal(),
        ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
            .field_value(
                ForgeQueryEvidenceTag::new("admission_decision"),
                receipt.admission_decision_digest(),
            )
            .seal(),
        receipt.receipt_identity().clone(),
    )
}

fn certification_intent_receipt() -> crate::runtime::ForgeQueryIntentReceipt {
    let mut runtime = certification_runtime();
    runtime
        .execute_intent(ForgeQueryIntentDeclaration::strategy_commit(
            "lower-runtime-certification-intent",
            "Task",
            "v1",
            "lower-runtime-certification-intent-input",
            ForgeQueryIntentInput::object([
                ("collection", ForgeQueryIntentInput::string("Task")),
                (
                    "entity_identity",
                    ForgeQueryIntentInput::string("intent-task-1"),
                ),
                ("title", ForgeQueryIntentInput::string("Intent fixture")),
            ]),
        ))
        .expect("intent runtime fixture should execute")
}

fn route_planned_row(
    seam_key: ForgeQueryLowerRuntimeSeamKey,
    capability_label: &str,
    subject_identity: ForgeQueryEvidenceIdentity,
    route_identity: ForgeQueryEvidenceIdentity,
    retained_evidence_source: ForgeQueryEvidenceIdentity,
) -> RepresentativeArtifacts {
    let request = ForgeQueryLowerRuntimeCapabilityRequest::new(
        seam_key,
        ForgeQueryLowerRuntimeRouteKind::RoutePlanning,
        ForgeQueryLowerRuntimeAuthorityOwner::Query,
        capability_label,
        crate::lower_runtime_routing::ForgeQueryLowerRuntimeSubjectIdentity::compose(
            "phase-six-intent-route-subject",
        )
        .field_evidence_identity(ForgeQueryEvidenceTag::new("subject"), &subject_identity)
        .seal(),
    );
    let eligibility = ForgeQueryLowerRuntimeCapabilityEligibility::admitted_with_evidence_identity(
        request.clone(),
        &retained_evidence_source,
    );
    let route_plan = ForgeQueryLowerRuntimeRoutePlan::new(
        eligibility.clone(),
        crate::lower_runtime_routing::ForgeQueryLowerRuntimeRouteSubjectIdentity::from_evidence_identity(
            "phase-six-intent-route",
            &route_identity,
        ),
    );
    let retained_evidence_identity =
        crate::lower_runtime_routing::forge_query_lower_runtime_retained_evidence_identity(
            "phase-six-intent-route",
            &retained_evidence_source,
        );
    let boundary_receipt = ForgeQueryLowerRuntimeBoundaryExecutionReceipt::from_route_plan(
        &route_plan,
        &retained_evidence_identity,
    );
    let envelope = ForgeQueryLowerRuntimeBoundaryEnvelope::from_route_plan(
        seam_key,
        &route_plan,
        &boundary_receipt,
        &retained_evidence_identity,
    );
    RepresentativeArtifacts {
        seam_key,
        request,
        eligibility,
        route_plan: Some(route_plan),
        boundary_receipt,
        envelope,
        evidence_source: ForgeQueryLowerRuntimeRepresentativeEvidenceSource::RuntimeBackedFixture,
    }
}
