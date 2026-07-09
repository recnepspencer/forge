use worth_store::{
    ArtifactCompatibilityWindow, ArtifactFamilyId, ArtifactFormatVersion,
    ArtifactSemanticVersion, DerivedInvalidationPlan, DerivedInvalidationReason,
};

fn main() {
    let _ = DerivedInvalidationPlan::new(
        ArtifactFamilyId::new("snapshot_record"),
        ArtifactFormatVersion::new(1),
        ArtifactSemanticVersion::new(1),
        ArtifactCompatibilityWindow::native(1),
        DerivedInvalidationReason::SemanticWindowMismatch,
        "stale",
    );
}
