use forge_store_layout_indexes::{
    integrity::OfflineReadmissionRequirement, AdmittedPhysicalArtifactFamily,
};
use forge_store_recovery_physics::RecoveryLayoutReadmissionIdentity;

fn misuse(family: AdmittedPhysicalArtifactFamily, identity: RecoveryLayoutReadmissionIdentity) {
    let _ = OfflineReadmissionRequirement { family, identity };
}

fn main() {}
