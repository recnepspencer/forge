use forge_store::CompatibilityCheckedArtifact;

fn main() {
    semantic_view(artifact());
}

fn semantic_view(checked: CompatibilityCheckedArtifact) {
    let _ = checked.view();
}

fn artifact() -> CompatibilityCheckedArtifact {
    panic!("compile-fail fixture")
}
