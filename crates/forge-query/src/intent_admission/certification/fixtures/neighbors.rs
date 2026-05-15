use super::authoritative_declaration;
use super::bridge::certification_bridge;
use super::runtime::certification_runtime;
use crate::facade::runtime::{
    admit_runtime_intent_request, CausalEvidenceFamily, CausalInspection,
    CausalInspectionMaterializationPolicy, CausalInspectionRedactionPolicy,
    ForgeQueryIntentAdmissionDecision, ForgeQueryIntentViolationDecision,
    ForgeQueryRawIntentAdmissionRequest, QueryObservationReceipt,
};
use crate::intent_admission::dx::ForgeQueryRuntimeIntentAdmissionReviewData;
use crate::intent_admission::{
    ForgeQueryIntentAdmissionCoveredEntrypoint, ForgeQueryIntentDecisionTraceEnvelope,
};
use crate::runtime::CausalInspectionBoundaryAudit;

#[derive(Clone)]
pub(in crate::intent_admission::certification) struct CertifiedDeferredIntentFixture {
    pub(in crate::intent_admission::certification) request: ForgeQueryRawIntentAdmissionRequest,
    pub(in crate::intent_admission::certification) decision: ForgeQueryIntentAdmissionDecision,
    pub(in crate::intent_admission::certification) trace: ForgeQueryIntentDecisionTraceEnvelope,
}

#[derive(Clone)]
pub(in crate::intent_admission::certification) struct CertifiedUnsupportedIntentFixture {
    pub(in crate::intent_admission::certification) request: ForgeQueryRawIntentAdmissionRequest,
    pub(in crate::intent_admission::certification) decision: ForgeQueryIntentAdmissionDecision,
    pub(in crate::intent_admission::certification) trace: ForgeQueryIntentDecisionTraceEnvelope,
}

#[derive(Clone)]
pub(in crate::intent_admission::certification) struct CertifiedInspectionAdvisoryRedactionFixture {
    pub(in crate::intent_admission::certification) full_artifact_digest: String,
    pub(in crate::intent_admission::certification) redacted_artifact_digest: String,
    pub(in crate::intent_admission::certification) causal_identity_digest: String,
    pub(in crate::intent_admission::certification) boundary_audit_digest: String,
}

pub(in crate::intent_admission::certification) fn certified_deferred_intent_fixture(
) -> CertifiedDeferredIntentFixture {
    let declaration = authoritative_declaration("certification-deferred-intent");
    let request = ForgeQueryRawIntentAdmissionRequest::deferred_neighbor(
        ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteReadNeighborDeferred,
        declaration,
    )
    .expect("deferred certification request should build");
    let decision = admit_runtime_intent_request(request.clone());
    let trace = review_trace(&request, &decision, "deferred");
    CertifiedDeferredIntentFixture {
        request,
        decision,
        trace,
    }
}

pub(in crate::intent_admission::certification) fn certified_unsupported_intent_fixture(
) -> CertifiedUnsupportedIntentFixture {
    let declaration = authoritative_declaration("certification-unsupported-intent");
    let request = ForgeQueryRawIntentAdmissionRequest::deferred_neighbor(
        ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteInspectionNeighborDeferred,
        declaration,
    )
    .expect("unsupported certification request should build");
    let eligibility = crate::intent_admission::ForgeQueryIntentAdmissionEligibility::from_request(
        request.clone(),
    );
    let violation = ForgeQueryIntentViolationDecision::from_eligibility_denial(
        &eligibility,
        "support-unsupported",
        "neighbor-unsupported-until-coverage",
    );
    let decision = ForgeQueryIntentAdmissionDecision::Violation(violation);
    let trace = review_trace(&request, &decision, "unsupported");
    CertifiedUnsupportedIntentFixture {
        request,
        decision,
        trace,
    }
}

pub(in crate::intent_admission::certification) fn certified_inspection_advisory_redaction_fixture(
) -> CertifiedInspectionAdvisoryRedactionFixture {
    let bridge = certification_bridge();
    let declaration = authoritative_declaration("certification-inspection-advisory-redaction");
    let mut runtime = certification_runtime();
    let receipt = runtime
        .execute_intent(declaration)
        .expect("inspection certification intent should execute");
    let inspection = runtime
        .inspect_intent_receipt(&receipt)
        .expect("inspection certification receipt should inspect");
    let observation = QueryObservationReceipt::from_intent_receipt_inspection(&inspection);

    let full = CausalInspection::for_observation(observation.clone())
        .why_changed()
        .materialized_detail()
        .evidence_families([
            CausalEvidenceFamily::QueryInspection,
            CausalEvidenceFamily::RelationalAuthority,
        ])
        .redaction(CausalInspectionRedactionPolicy::PreserveDetail)
        .materialization(CausalInspectionMaterializationPolicy::OfflineInterpretableArtifact)
        .plan()
        .expect("full inspection plan should build")
        .materialize_with_bridge(&bridge)
        .expect("full inspection artifact should materialize");

    let redacted = CausalInspection::for_observation(observation)
        .why_changed()
        .materialized_detail()
        .evidence_families([
            CausalEvidenceFamily::QueryInspection,
            CausalEvidenceFamily::RelationalAuthority,
        ])
        .redaction(CausalInspectionRedactionPolicy::DigestOnly)
        .materialization(CausalInspectionMaterializationPolicy::DigestReferenceOnly)
        .plan()
        .expect("redacted inspection plan should build")
        .materialize_with_bridge(&bridge)
        .expect("redacted inspection artifact should materialize");

    let boundary_audit =
        CausalInspectionBoundaryAudit::from_query_artifact_public_surface(&redacted);
    CertifiedInspectionAdvisoryRedactionFixture {
        full_artifact_digest: full.artifact_digest().to_string(),
        redacted_artifact_digest: redacted.artifact_digest().to_string(),
        causal_identity_digest: redacted.causal_identity_digest().to_string(),
        boundary_audit_digest: boundary_audit.audit_digest().to_string(),
    }
}

fn review_trace(
    request: &ForgeQueryRawIntentAdmissionRequest,
    decision: &ForgeQueryIntentAdmissionDecision,
    lane: &str,
) -> ForgeQueryIntentDecisionTraceEnvelope {
    ForgeQueryRuntimeIntentAdmissionReviewData::from_decision(request.clone(), decision.clone())
        .decision_trace_envelope()
        .unwrap_or_else(|| panic!("{lane} certification review should preserve a trace"))
        .clone()
}
