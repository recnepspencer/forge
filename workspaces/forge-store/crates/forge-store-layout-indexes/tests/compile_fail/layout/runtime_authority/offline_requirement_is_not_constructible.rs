use forge_store_layout_indexes::declarations::PhysicalArtifactFamily;
use forge_store_layout_indexes::integrity::OfflineReadmissionRequirement;
use forge_store_recovery_physics::RecoveryLayoutReadmissionIdentity;

fn misuse(family: PhysicalArtifactFamily, identity: RecoveryLayoutReadmissionIdentity) {
    let _ = OfflineReadmissionRequirement { family, identity };
}

fn main() {}
