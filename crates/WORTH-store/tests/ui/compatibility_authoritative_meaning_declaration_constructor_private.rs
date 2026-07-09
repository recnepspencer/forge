use worth_store::{
    ArtifactFamilyId, ArtifactSemanticVersion, AuthoritativeMeaningDeclaration,
};

fn main() {
    let _ = AuthoritativeMeaningDeclaration::new(
        ArtifactFamilyId::new("commit_envelope"),
        ArtifactSemanticVersion::new(1),
        "commit-envelope-v1",
    );
}
