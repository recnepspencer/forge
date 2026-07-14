use worth_query::facade::certification::{BasisLifecyclePhaseArtifact, BasisLifecyclePhaseManifestRow};

fn main() {
    let _ = BasisLifecyclePhaseManifestRow {
        artifact: BasisLifecyclePhaseArtifact::RawIntent,
        producer: "producer",
        required_input: "input",
        next_consumer: "consumer",
        enforcement_proof: "basis_lifecycle_private_row",
        row_digest: String::new(),
    };
}
