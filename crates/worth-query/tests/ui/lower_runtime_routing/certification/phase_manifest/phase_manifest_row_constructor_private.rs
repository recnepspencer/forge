use worth_query::facade::{
    WorthQueryLowerRuntimePhaseArtifact, WorthQueryLowerRuntimePhaseManifestRow,
};

fn main() {
    let _ = WorthQueryLowerRuntimePhaseManifestRow::new(
        WorthQueryLowerRuntimePhaseArtifact::CapabilityRequest,
        "producer",
        "input",
        "consumer",
        "proof",
    );
}
