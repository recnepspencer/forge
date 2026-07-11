use std::path::{Path, PathBuf};

#[test]
fn raw_transition_issuance_is_confined_to_owner_definition_modules() {
    let source_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let sources = rust_sources(&source_root);

    assert_only_file_contains(&sources, "S8LayoutProductionTransition::new(", "fact.rs");
    assert_paths_match(
        &sources,
        "S8TransitionedResult",
        |_| false,
        "the generic payload/fact pairing lane must remain deleted",
    );
    assert_paths_match(
        &sources,
        "define_owner_outcome_family!(",
        |_| false,
        "parallel owner catalogs must remain deleted",
    );
    assert!(sources.iter().all(|path| {
        path.file_name().and_then(|name| name.to_str()) != Some("production_transitions.rs")
    }));
}

#[test]
fn displaced_transition_catalogs_and_layout_readiness_lane_are_absent() {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR"));
    let layout_sources = rust_sources(&manifest.join("src"));
    let isolation_sources = rust_sources(&manifest.join("../forge-store-physical-isolation/src"));
    let combined = layout_sources
        .iter()
        .chain(isolation_sources.iter())
        .map(|path| std::fs::read_to_string(path).expect("phase source remains readable"))
        .collect::<String>();

    for displaced in [
        "OWNER_OUTCOME_FACTS",
        "static OUTCOME_FACTS",
        "define_owner_transition_facts",
        "CompactionCutoverOutcomeKind",
        "CompactionMutationLaneReceiptKind::ALL",
        "LsmCompactionCutoverAdmission::production_transitions",
    ] {
        assert!(
            !combined.contains(displaced),
            "displaced transition authority survived: {displaced}"
        );
    }
    let registry = std::fs::read_to_string(manifest.join("src/strategy_registry/registry.rs"))
        .expect("layout admission registry remains readable");
    assert!(!registry.contains("fn admit_ready("));
    assert!(!registry.contains("S8LayoutAdmissionOutcome::Deferred"));
}

fn assert_paths_match(
    sources: &[PathBuf],
    needle: &str,
    allowed: impl Fn(&Path) -> bool,
    message: &str,
) {
    let violating = sources
        .iter()
        .filter(|path| {
            std::fs::read_to_string(path)
                .expect("phase source remains readable")
                .contains(needle)
                && !allowed(path)
        })
        .collect::<Vec<_>>();
    assert!(violating.is_empty(), "{message}: {violating:?}");
}

fn assert_only_file_contains(sources: &[PathBuf], needle: &str, allowed_file: &str) {
    let owners = sources
        .iter()
        .filter(|path| {
            std::fs::read_to_string(path)
                .expect("phase source remains readable")
                .contains(needle)
        })
        .collect::<Vec<_>>();
    assert_eq!(owners.len(), 1, "{needle} must have exactly one owner");
    assert_eq!(
        owners[0].file_name().and_then(|name| name.to_str()),
        Some(allowed_file),
        "{needle} escaped its owner module"
    );
}

fn rust_sources(root: &Path) -> Vec<PathBuf> {
    let mut pending = vec![root.to_path_buf()];
    let mut sources = Vec::new();
    while let Some(directory) = pending.pop() {
        for entry in std::fs::read_dir(directory).expect("phase source directory remains readable")
        {
            let path = entry.expect("phase source entry remains readable").path();
            if path.is_dir() {
                pending.push(path);
            } else if path.extension().and_then(|extension| extension.to_str()) == Some("rs") {
                sources.push(path);
            }
        }
    }
    sources
}
