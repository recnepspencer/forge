use forge_store::{
    ArtifactFamilyId, ArtifactSemanticVersion, DerivedRebuildCompatibilityPlan,
};

fn main() {
    let _ = DerivedRebuildCompatibilityPlan::new(
        ArtifactFamilyId::new("snapshot_record"),
        ArtifactSemanticVersion::new(1),
        ArtifactSemanticVersion::new(2),
        "maintenance-lane",
    );
}
