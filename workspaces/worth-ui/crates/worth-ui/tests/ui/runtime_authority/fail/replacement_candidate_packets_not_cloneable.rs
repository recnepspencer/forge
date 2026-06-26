use worth_ui::facade::{
    WorthUiCandidateArtifactBundle, WorthUiCandidateDependencyMetadata, WorthUiReplacementCandidate,
};

fn requires_clone<T: Clone>() {}

fn main() {
    requires_clone::<WorthUiReplacementCandidate>();
    requires_clone::<WorthUiCandidateArtifactBundle>();
    requires_clone::<WorthUiCandidateDependencyMetadata>();
}
