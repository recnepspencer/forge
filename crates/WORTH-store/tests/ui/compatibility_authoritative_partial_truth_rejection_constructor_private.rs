use worth_store::{
    ArtifactFamilyId, ArtifactSemanticVersion, AuthoritativePartialTruthRejection,
};

fn main() {
    let _ = AuthoritativePartialTruthRejection::new(
        ArtifactFamilyId::new("commit_envelope"),
        ArtifactSemanticVersion::new(1),
        "unknown meaning",
    );
}
