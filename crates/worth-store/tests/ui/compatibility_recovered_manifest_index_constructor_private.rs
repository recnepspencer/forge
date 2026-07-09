use worth_store::{CompatibilityManifestFrontier, CompatibilityRecoveredManifestIndex};

fn main() {
    let _ = CompatibilityRecoveredManifestIndex::new(Vec::new(), frontier());
}

fn frontier() -> CompatibilityManifestFrontier {
    panic!("compile-fail fixture")
}
