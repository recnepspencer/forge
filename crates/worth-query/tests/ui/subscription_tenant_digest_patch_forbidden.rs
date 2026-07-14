use worth_query::facade::runtime::LiveQueryAdmissionArtifact;

fn main() {}

fn patch_tenant_digest_after_admission(artifact: LiveQueryAdmissionArtifact) {
    let _patched = artifact.with_tenant_digest("tenant-v2");
}
