#[path = "../support/installed_domain.rs"]
mod installed_domain;

use worth_query::facade::domain::WorthQueryLowerRuntimeExplanationRequest;
use worth_query::facade::runtime::{CausalEvidenceFamily, CausalEvidenceReferenceSet, CausalInspectionMaterializationPolicy, CausalInspectionRedactionPolicy, CausalInspectionTarget, WorthQueryLowerRuntimeBoundaryEnvelope, QueryCausalInspectionArtifact};
use worth_runtime_bridge::facade::BridgeCausalExplanationEnvelope;

fn explanation_common_lane(
    envelope: &WorthQueryLowerRuntimeBoundaryEnvelope,
    reference_set: CausalEvidenceReferenceSet,
    target: CausalInspectionTarget,
    bridge_envelope: BridgeCausalExplanationEnvelope,
) -> QueryCausalInspectionArtifact {
    installed_domain::install("explanation-golden")
        .contributions()
        .for_lower_runtime_boundary_envelope(envelope).expect("installed contribution authority must remain current")
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
