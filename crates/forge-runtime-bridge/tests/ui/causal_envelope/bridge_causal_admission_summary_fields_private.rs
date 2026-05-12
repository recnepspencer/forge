use forge_runtime_bridge::facade::{
    BridgeCausalInspectionAdmissionSummary, BridgeCausalInspectionAdmissionSummaryKind,
};

fn main() {
    let _ = BridgeCausalInspectionAdmissionSummary {
        kind: BridgeCausalInspectionAdmissionSummaryKind::Admitted,
        query_admission_digest: "admission".into(),
        causal_observation_anchor_digest: "anchor".into(),
        summary_digest: "summary".into(),
    };
}
