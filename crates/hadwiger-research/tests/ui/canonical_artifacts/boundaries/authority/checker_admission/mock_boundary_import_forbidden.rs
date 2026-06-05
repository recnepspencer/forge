use hadwiger_research::facade::HadwigerMockBoundaryCausalEvidence;

fn main() {
    let _ = HadwigerMockBoundaryCausalEvidence::new(
        "truth-view",
        "route",
        "evaluation",
        "diagnostics",
        "replay",
    );
}
