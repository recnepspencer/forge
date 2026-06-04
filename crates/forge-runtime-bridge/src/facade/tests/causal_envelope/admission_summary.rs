use super::{bridge_route_reference, query_observation_reference, runtime, BridgeRuntimePolicy};
use crate::facade::{
    BridgeCausalEnvelopeAssemblyRequest, BridgeCausalEvidenceReferenceIdentity,
    BridgeCausalInspectionAdmissionSummary, BridgeCausalInspectionAdmissionSummaryKind,
};

#[test]
fn causal_envelope_request_carries_advisory_query_admission_summary() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let routed = runtime
        .route(crate::facade::TruthCommitIdentity::new(
            "commit-causal-advisory-summary",
        ))
        .expect("route should succeed");
    let admission_summary = BridgeCausalInspectionAdmissionSummary::advisory(
        "query-admission:advisory",
        "anchor:advisory",
    )
    .expect("advisory summary should be valid");
    let request = BridgeCausalEnvelopeAssemblyRequest::from_query_admission(
        admission_summary,
        vec![
            query_observation_reference(
                BridgeCausalEvidenceReferenceIdentity::query_observation(
                    "query-observation:advisory",
                )
                .expect("query observation reference identity should be valid"),
            ),
            bridge_route_reference(routed.result().result_summary()),
        ],
    )
    .expect("request should be valid");

    assert_eq!(
        request.admission_summary().kind(),
        BridgeCausalInspectionAdmissionSummaryKind::Advisory
    );
    assert_eq!(request.query_admission_digest(), "query-admission:advisory");
    assert_eq!(
        request.causal_observation_anchor_digest(),
        "anchor:advisory"
    );
    assert!(request
        .admission_summary()
        .summary_digest()
        .starts_with("bridge-causal-inspection-admission-summary:sha256:"));
    let admission_summary_digest = request.admission_summary().summary_digest().to_string();

    let envelope = runtime
        .diagnostics()
        .assemble_causal_explanation_envelope(request)
        .expect("advisory query admission summary should assemble");

    assert_eq!(
        envelope.admission_summary_kind(),
        BridgeCausalInspectionAdmissionSummaryKind::Advisory
    );
    assert_eq!(
        envelope.admission_summary_digest(),
        admission_summary_digest
    );
    assert_eq!(envelope.counters().bridge_retained_lookup_count(), 1);
    assert_eq!(envelope.counters().external_authority_reference_count(), 1);
}
