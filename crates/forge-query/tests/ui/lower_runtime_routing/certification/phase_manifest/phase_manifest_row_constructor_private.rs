use forge_query::facade::{
    ForgeQueryLowerRuntimePhaseArtifact, ForgeQueryLowerRuntimePhaseManifestRow,
};

fn main() {
    let _ = ForgeQueryLowerRuntimePhaseManifestRow::new(
        ForgeQueryLowerRuntimePhaseArtifact::CapabilityRequest,
        "producer",
        "input",
        "consumer",
        "proof",
    );
}
