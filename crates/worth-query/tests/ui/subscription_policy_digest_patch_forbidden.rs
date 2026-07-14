use worth_query::facade::runtime::LiveQueryAdmissionArtifact;

fn main() {}

fn patch_policy_digest_after_admission(artifact: LiveQueryAdmissionArtifact) {
    let _patched = artifact.with_policy_digest("policy-v2");
}
