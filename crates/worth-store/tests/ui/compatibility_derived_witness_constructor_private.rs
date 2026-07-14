use worth_store::{ArtifactFamilyId, DerivedCompatibilityWitness};

fn main() {
    let _ = DerivedCompatibilityWitness::new(ArtifactFamilyId::new("snapshot_record"));
}
