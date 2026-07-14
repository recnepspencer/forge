use worth_query::facade::runtime::WorthQueryLowerRuntimePhaseArtifact;
use worth_query::facade::certification::WorthQueryLowerRuntimePhaseManifestRow;

fn main() {
    let _ = WorthQueryLowerRuntimePhaseManifestRow::new(
        WorthQueryLowerRuntimePhaseArtifact::CapabilityRequest,
        "producer",
        "input",
        "consumer",
        "proof",
    );
}
