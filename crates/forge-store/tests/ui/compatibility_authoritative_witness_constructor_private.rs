use forge_store::{ArtifactFamilyId, AuthoritativeCompatibilityWitness};

fn main() {
    let _ = AuthoritativeCompatibilityWitness::new(ArtifactFamilyId::new("commit_envelope"));
}
