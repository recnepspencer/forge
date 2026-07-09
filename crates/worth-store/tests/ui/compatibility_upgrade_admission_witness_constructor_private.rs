use worth_store::{ArtifactFamilyId, UpgradeAdmissionWitness};

fn main() {
    let _ = UpgradeAdmissionWitness::new(ArtifactFamilyId::new("commit_envelope"));
}
