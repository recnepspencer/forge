pub(super) fn assert_bounded_physical_record_access_examples_are_compile_bound() {
    assert_document_examples_are_compile_bound(
        "../../../../_docs/worth-store/bounded-physical-record-access.md",
        "tests/physical_runtime_authority/bounded_physical_record_access_examples.rs",
        5,
    );
}

fn assert_document_examples_are_compile_bound(
    document_path: &str,
    specimen_path: &str,
    expected_blocks: usize,
) {
    let crate_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let document = std::fs::read_to_string(crate_root.join(document_path)).unwrap();
    let specimen = std::fs::read_to_string(crate_root.join(specimen_path)).unwrap();
    let specimen = super::normalized_rust(&specimen);
    let blocks = document
        .split("```rust")
        .skip(1)
        .map(|tail| tail.split("```").next().unwrap())
        .collect::<Vec<_>>();

    assert_eq!(
        blocks.len(),
        expected_blocks,
        "every public Rust block must be inventoried"
    );
    for block in blocks {
        let normalized = super::normalized_rust(block);
        assert!(
            specimen.contains(&normalized),
            "a public Rust block drifted from its compiler specimen:\n{block}",
        );
    }
}
