use serde_json::json;

use crate::identity::hash_parts;
use crate::intent_admission::certification_runtime;
use crate::lower_runtime_routing::{
    ForgeQueryLowerRuntimeAuthorityOwner, ForgeQueryLowerRuntimeBoundaryEnvelope,
    ForgeQueryLowerRuntimeBoundaryExecutionReceipt, ForgeQueryLowerRuntimeCapabilityEligibility,
    ForgeQueryLowerRuntimeCapabilityRequest, ForgeQueryLowerRuntimeRouteKind,
    ForgeQueryLowerRuntimeRoutePlan, ForgeQueryLowerRuntimeSeamKey,
};
use crate::runtime::ForgeQueryIntentDeclaration;

use super::super::{ForgeQueryLowerRuntimeRepresentativeEvidenceSource, RepresentativeArtifacts};

pub(crate) fn representative_runtime_intent_authority_row() -> RepresentativeArtifacts {
    let receipt = certification_intent_receipt();
    route_planned_row(
        ForgeQueryLowerRuntimeSeamKey::RuntimeIntentAuthorityAdapter,
        "Runtime intent authority seam",
        &[
            "runtime_intent_authority_subject_v1".to_string(),
            format!("strategy:{}", receipt.strategy_identity()),
            format!("outcome:{}", receipt.outcome_digest()),
            format!("provenance:{}", receipt.execution_provenance_chain_digest()),
        ],
        receipt.execution_binding_digest().to_string(),
        receipt.execution_provenance_chain_digest().to_string(),
    )
}

pub(crate) fn representative_intent_runtime_execution_row() -> RepresentativeArtifacts {
    let receipt = certification_intent_receipt();
    route_planned_row(
        ForgeQueryLowerRuntimeSeamKey::IntentRuntimeExecution,
        "Intent runtime execution",
        &[
            "intent_runtime_execution_subject_v1".to_string(),
            format!("intent:{}", receipt.intent_name()),
            format!("commit:{}", receipt.commit_identity()),
            format!("snapshot:{}", receipt.snapshot_token()),
            format!("receipt:{}", receipt.receipt_digest()),
        ],
        receipt.admission_decision_digest().to_string(),
        receipt.receipt_digest().to_string(),
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
            json!({
                "collection": "Task",
                "entity_identity": "intent-task-1",
                "title": "Intent fixture"
            }),
        ))
        .expect("intent runtime fixture should execute")
}

fn route_planned_row(
    seam_key: ForgeQueryLowerRuntimeSeamKey,
    capability_label: &str,
    subject_parts: &[String],
    support_label: String,
    retained_evidence_digest: String,
) -> RepresentativeArtifacts {
    let request = ForgeQueryLowerRuntimeCapabilityRequest::new(
        seam_key,
        ForgeQueryLowerRuntimeRouteKind::RoutePlanning,
        ForgeQueryLowerRuntimeAuthorityOwner::Query,
        capability_label,
        hash_parts(subject_parts),
    );
    let eligibility = ForgeQueryLowerRuntimeCapabilityEligibility::admitted(
        request.clone(),
        retained_evidence_digest.clone(),
    );
    let route_plan = ForgeQueryLowerRuntimeRoutePlan::new(eligibility.clone(), support_label);
    let boundary_receipt = ForgeQueryLowerRuntimeBoundaryExecutionReceipt::from_route_plan(
        &route_plan,
        retained_evidence_digest.clone(),
    );
    let envelope = ForgeQueryLowerRuntimeBoundaryEnvelope::from_route_plan(
        seam_key,
        &route_plan,
        &boundary_receipt,
        &retained_evidence_digest,
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
