use worth_store::{
    CompatibilityManifestFrontier, CompatibilityManifestPublicationReceipt,
    CompatibilityManifestPublicationRecord,
};

fn main() {
    let _ = CompatibilityManifestPublicationReceipt::new(record(), frontier());
}

fn record() -> CompatibilityManifestPublicationRecord {
    panic!("compile-fail fixture")
}

fn frontier() -> CompatibilityManifestFrontier {
    panic!("compile-fail fixture")
}
