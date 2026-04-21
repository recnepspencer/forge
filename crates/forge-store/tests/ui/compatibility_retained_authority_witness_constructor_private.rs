use forge_store::{ArtifactFamilyId, RetainedAuthorityCompatibilityWitness};

fn main() {
    let _ = RetainedAuthorityCompatibilityWitness::new(ArtifactFamilyId::new("snapshot_record"));
}
