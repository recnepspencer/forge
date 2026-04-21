use forge_store::{ArtifactFamilyId, CompatibilityRebuildDebt};

fn main() {
    let _ = CompatibilityRebuildDebt::new(ArtifactFamilyId::new("snapshot_record"), 1);
}
