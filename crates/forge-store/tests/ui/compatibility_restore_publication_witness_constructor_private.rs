use forge_store::{ArtifactFamilyId, RestorePublicationWitness};

fn main() {
    let _ = RestorePublicationWitness::new(ArtifactFamilyId::new("snapshot_record"));
}
