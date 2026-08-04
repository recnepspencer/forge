use std::path::Path;

use super::super::repository_root;

#[test]
fn production_identifiers_do_not_encode_the_c7_milestone_or_compatibility_lane() {
    let source_root =
        repository_root().join("workspaces/worth-store/crates/worth-store/src/physical_runtime");
    let mut pending = vec![source_root];
    while let Some(directory) = pending.pop() {
        for entry in std::fs::read_dir(&directory)
            .unwrap_or_else(|error| panic!("cannot inspect {}: {error}", directory.display()))
        {
            let path = entry.expect("inspect physical runtime entry").path();
            if path.is_dir() {
                pending.push(path);
            } else if path.extension().and_then(|value| value.to_str()) == Some("rs") {
                assert_semantic_source(&path);
            }
        }
    }
}

fn assert_semantic_source(path: &Path) {
    let source = std::fs::read_to_string(path)
        .unwrap_or_else(|error| panic!("cannot read {}: {error}", path.display()));
    for forbidden in [
        "C7Durability",
        "C7Mutation",
        "C7Checkpoint",
        "PhysicalDurabilityCompatibility",
        "LegacyPhysicalDurabilityAlias",
    ] {
        assert!(
            !source.contains(forbidden),
            "production source {} contains forbidden milestone or compatibility identifier `{forbidden}`",
            path.display()
        );
    }
}
