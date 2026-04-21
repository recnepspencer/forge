use forge_store::CompatibilityCheckedArtifact;

fn main() {
    let _ = CompatibilityCheckedArtifact::new(unreachable!(), unreachable!());
}
