use forge_runtime_bridge::facade::{
    BridgeCausalEnvelopeAssemblyRequest, BridgeCausalEvidenceReference,
    BridgeCausalInspectionAdmissionSummary,
};

fn main() {
    let references: Vec<BridgeCausalEvidenceReference> = Vec::new();
    let admission_summary =
        BridgeCausalInspectionAdmissionSummary::admitted("admission", "anchor").unwrap();

    let _ = BridgeCausalEnvelopeAssemblyRequest {
        admission_summary,
        references: references.into(),
        request_digest: "request".into(),
    };
}
