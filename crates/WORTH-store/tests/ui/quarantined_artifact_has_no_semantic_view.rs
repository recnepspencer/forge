use worth_store::QuarantinedDecodedArtifact;

fn leak_payload(artifact: &QuarantinedDecodedArtifact) {
    let _ = artifact.semantic_view();
}

fn main() {}
