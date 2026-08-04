use std::path::Path;

#[test]
fn documented_wal_examples_are_exact_external_compile_specimens() {
    let crate_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let document = std::fs::read_to_string(
        crate_root.join("../../../../_docs/worth-store/physical-wal-append.md"),
    )
    .unwrap();
    let specimen = std::fs::read_to_string(
        crate_root.join("tests/physical_runtime_authority/physical_wal_append_examples.rs"),
    )
    .unwrap();
    let specimen = normalized_rust(&specimen);
    let blocks = document
        .split("```rust")
        .skip(1)
        .map(|tail| tail.split("```").next().unwrap())
        .collect::<Vec<_>>();

    assert_eq!(
        blocks.len(),
        2,
        "every documented Rust block is inventoried"
    );
    for block in blocks {
        assert!(
            specimen.contains(&normalized_rust(block)),
            "a documented WAL example drifted from its external compile specimen:\n{block}",
        );
    }
}

fn normalized_rust(source: &str) -> String {
    let mut normalized = String::new();
    for (index, segment) in source.split('"').enumerate() {
        if index > 0 {
            normalized.push('"');
        }
        if index % 2 == 0 {
            normalized.extend(
                segment
                    .chars()
                    .filter(|character| !character.is_whitespace()),
            );
        } else {
            normalized.push_str(segment);
        }
    }
    normalized.replace(",}", "}")
}
