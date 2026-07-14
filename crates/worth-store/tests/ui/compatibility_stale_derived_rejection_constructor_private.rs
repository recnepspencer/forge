use worth_store::{ArtifactFamilyId, ArtifactSemanticVersion, StaleDerivedVersionRejection};

fn main() {
    let _ = StaleDerivedVersionRejection::new(
        ArtifactFamilyId::new("snapshot_record"),
        ArtifactSemanticVersion::new(1),
    );
}
