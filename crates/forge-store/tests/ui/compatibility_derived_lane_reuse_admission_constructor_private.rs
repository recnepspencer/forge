use forge_store::{
    ArtifactFamilyId, DerivedCompatibilityLaneKind, DerivedLaneCompatibilityPosture,
    DerivedLaneReuseAdmission,
};

fn main() {
    let _ = DerivedLaneReuseAdmission::new(
        ArtifactFamilyId::new("snapshot_record"),
        DerivedCompatibilityLaneKind::SnapshotReuse,
        DerivedLaneCompatibilityPosture::ReuseAdmitted,
    );
}
