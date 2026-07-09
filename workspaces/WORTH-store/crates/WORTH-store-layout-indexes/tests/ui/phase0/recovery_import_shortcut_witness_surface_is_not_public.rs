use worth_store_contracts::{DurableArtifactFamilyId, StableDigest};
use worth_store_recovery_physics::recovery_readmission_layout_family;

fn main() {
    let family = DurableArtifactFamilyId::PhysicalPage;
    let digest = StableDigest::new("import-shortcut").unwrap();
    let _ = recovery_readmission_layout_family().admit_import_readmission_witness(family, &digest);
}
