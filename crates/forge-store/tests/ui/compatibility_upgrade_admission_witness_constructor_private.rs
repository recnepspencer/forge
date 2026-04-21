use forge_store::{ArtifactFamilyId, UpgradeAdmissionWitness};

fn main() {
    let _ = UpgradeAdmissionWitness::new(ArtifactFamilyId::new("commit_envelope"));
}
