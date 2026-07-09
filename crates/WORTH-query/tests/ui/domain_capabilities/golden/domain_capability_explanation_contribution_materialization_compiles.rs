use worth_query::facade::runtime::{
    worth_query_domain, CausalEvidenceFamily, CausalEvidenceReferenceSet,
    CausalInspectionMaterializationPolicy, CausalInspectionRedactionPolicy,
    CausalInspectionTarget, WorthQueryLowerRuntimeBoundaryEnvelope,
    WorthQueryLowerRuntimeExplanationRequest,
    QueryCausalInspectionArtifact,
};
use worth_runtime_bridge::facade::BridgeCausalExplanationEnvelope;

fn explanation_common_lane(
    envelope: &WorthQueryLowerRuntimeBoundaryEnvelope,
    reference_set: CausalEvidenceReferenceSet,
    target: CausalInspectionTarget,
    bridge_envelope: BridgeCausalExplanationEnvelope,
) -> QueryCausalInspectionArtifact {
    worth_query_domain("worth.spatial")
        .for_lower_runtime_boundary_envelope(envelope)
        .explains_cross_runtime_fallback(
            "routing.cross_runtime_fallback",
            WorthQueryLowerRuntimeExplanationRequest::explains_cross_runtime_fallback(
                reference_set,
                target,
                vec![CausalEvidenceFamily::BridgeRoute],
                bridge_envelope,
                CausalInspectionRedactionPolicy::PreserveDetail,
                CausalInspectionMaterializationPolicy::OfflineInterpretableArtifact,
            ),
        )
        .because("lower-runtime routing should preserve bridge-backed causal context")
        .materialize_artifact()
        .expect("explanation common lane should materialize")
}

fn main() {}
