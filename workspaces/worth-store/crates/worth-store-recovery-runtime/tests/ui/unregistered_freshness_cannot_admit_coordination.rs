use worth_store::physical_runtime::{
    AdmittedRecoveryFilesystemMedia, PhysicalRecoveryCoordinationCapacity,
    PhysicalRecoveryFreshnessAuthority,
};

fn cannot_coordinate(
    freshness: PhysicalRecoveryFreshnessAuthority,
    media: &AdmittedRecoveryFilesystemMedia,
    capacity: PhysicalRecoveryCoordinationCapacity,
) {
    let _ = freshness.admit_coordination(media, capacity);
}

fn main() {}
