use worth_ui::facade::WorthUiCandidateArtifactBundle;

fn main() {
    let _bundle = WorthUiCandidateArtifactBundle {
        artifact: uninitialized_field(),
        artifact_digest: uninitialized_field(),
        artifact_digest_report: uninitialized_field(),
        dependency_metadata: uninitialized_field(),
        lowering_basis: uninitialized_field(),
    };
}

fn uninitialized_field<T>() -> T {
    unimplemented!()
}
