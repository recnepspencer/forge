use worth_store::{ArtifactFamilyId, CompatibilityMaintenanceAdmissionWitness};

fn main() {
    let _ = CompatibilityMaintenanceAdmissionWitness::new(
        ArtifactFamilyId::new("snapshot_record"),
        "maintenance-lane",
    );
}
