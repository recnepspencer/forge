const CANONICAL_DIRECT_POOL_ROOTS: &[&str] = &[
    "crates/worth-store/",
    "crates/worth-store-buffer-pool/",
    "crates/worth-store-io-scheduler/",
];

pub(super) fn is_direct_pool_consumer(path: &str, source: &str) -> bool {
    path.starts_with("crates/")
        && !CANONICAL_DIRECT_POOL_ROOTS
            .iter()
            .any(|root| path.starts_with(root))
        && contains_direct_pool_reference(path, source)
}

fn contains_direct_pool_reference(path: &str, source: &str) -> bool {
    if path.ends_with("Cargo.toml") {
        return source
            .lines()
            .any(|line| line.trim_start().starts_with("worth-store-buffer-pool"));
    }
    source.contains("worth_store_buffer_pool::")
}

#[test]
fn direct_pool_classifier_distinguishes_consumers_from_names_and_owners() {
    assert!(is_direct_pool_consumer(
        "crates/worth-store-certification/src/new_consumer.rs",
        "use worth_store_buffer_pool::PhysicalResidencyCounters;",
    ));
    assert!(is_direct_pool_consumer(
        "crates/worth-store-physical-certification/Cargo.toml",
        "worth-store-buffer-pool.workspace = true",
    ));
    assert!(!is_direct_pool_consumer(
        "crates/worth-store-contracts/src/artifact_family.rs",
        r#"Self::BufferPool => "worth-store-buffer-pool""#,
    ));
    assert!(!is_direct_pool_consumer(
        "crates/worth-store/src/physical_runtime/new_owner.rs",
        "use worth_store_buffer_pool::PhysicalResidencyCounters;",
    ));
}
