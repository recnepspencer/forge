use worth_query::facade::runtime::{CausalEvidenceFamily, CausalEvidenceReferenceSet, CausalInspectionExplanationFamily, CausalInspectionRequest, CausalInspectionRichness, CausalInspectionTarget};

fn main() {
    let reference_set: CausalEvidenceReferenceSet = todo!();
    let target: CausalInspectionTarget = todo!();

    let _ = CausalInspectionRequest {
        reference_set,
        target,
        explanation_family: CausalInspectionExplanationFamily::CrossRuntimeCausalExplanation,
        requested_richness: CausalInspectionRichness::ReferenceOnly,
        requested_evidence_families: vec![CausalEvidenceFamily::BridgeRoute],
        request_digest: "worthd-request".to_string(),
    };
}
