use worth_query::facade::runtime::LiveQueryAdmissionArtifact;

fn main() {}

fn patch_relationship_proof_digest_after_admission(artifact: LiveQueryAdmissionArtifact) {
    let _patched = artifact.with_relationship_proof_digest("proof-v2");
}
