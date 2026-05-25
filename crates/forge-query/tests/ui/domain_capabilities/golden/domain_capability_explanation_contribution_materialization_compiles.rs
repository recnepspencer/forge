use forge_query::facade::runtime::{
    forge_query_domain, CausalEvidenceFamily, CausalEvidenceReferenceSet,
    CausalInspectionMaterializationPolicy, CausalInspectionRedactionPolicy,
    CausalInspectionTarget, ForgeQueryLowerRuntimeBoundaryEnvelope,
    ForgeQueryLowerRuntimeExplanationRequest,
    QueryCausalInspectionArtifact,
};
use forge_runtime_bridge::facade::BridgeCausalExplanationEnvelope;

fn explanation_common_lane(
    envelope: &ForgeQueryLowerRuntimeBoundaryEnvelope,
    reference_set: CausalEvidenceReferenceSet,
    target: CausalInspectionTarget,
    bridge_envelope: BridgeCausalExplanationEnvelope,
) -> QueryCausalInspectionArtifact {
    forge_query_domain("worth.spatial")
        .for_lower_runtime_boundary_envelope(envelope)
        .explains_cross_runtime_fallback(
            "routing.cross_runtime_fallback",
            ForgeQueryLowerRuntimeExplanationRequest::explains_cross_runtime_fallback(
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
