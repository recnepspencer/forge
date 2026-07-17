use std::fs;
use std::path::{Path, PathBuf};

const MAX_DIRECT_RUST_FILES: usize = 10;
const BUCKET_FILE_NAMES: &[&str] = &[
    "common.rs",
    "helpers.rs",
    "model.rs",
    "shared.rs",
    "support.rs",
    "types.rs",
    "util.rs",
];

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("certification crate parent")
        .parent()
        .expect("Worth UI workspace root")
        .to_path_buf()
}

fn responsibility_roots() -> Vec<PathBuf> {
    let root = workspace_root();
    vec![
        root.join("crates/worth-ui-runtime/src/runtime/allocation_receipt"),
        root.join("crates/worth-ui-runtime/src/graph/allocation_neighborhood"),
        root.join("crates/worth-ui-runtime/src/runtime/invalidation_narrowing"),
        root.join("crates/worth-ui-query-binding/src/prerequisites"),
    ]
}

fn visit_production_directories(root: &Path, visit: &mut impl FnMut(&Path)) {
    if root.file_name().and_then(|name| name.to_str()) == Some("tests") {
        return;
    }
    visit(root);
    for entry in fs::read_dir(root).unwrap_or_else(|error| {
        panic!(
            "failed to read responsibility directory {}: {error}",
            root.display()
        )
    }) {
        let path = entry.expect("responsibility directory entry").path();
        if path.is_dir() {
            visit_production_directories(&path, visit);
        }
    }
}

#[test]
fn phase_18_responsibility_directories_remain_bounded_and_semantic() {
    let mut findings = Vec::new();
    for root in responsibility_roots() {
        visit_production_directories(&root, &mut |directory| {
            let rust_files = fs::read_dir(directory)
                .expect("responsibility directory")
                .filter_map(Result::ok)
                .map(|entry| entry.path())
                .filter(|path| {
                    path.is_file()
                        && path.extension().and_then(|value| value.to_str()) == Some("rs")
                })
                .collect::<Vec<_>>();
            if rust_files.len() > MAX_DIRECT_RUST_FILES {
                findings.push(format!(
                    "{} contains {} direct Rust files; Phase 18 permits at most {}",
                    directory.display(),
                    rust_files.len(),
                    MAX_DIRECT_RUST_FILES
                ));
            }
            for path in rust_files {
                let Some(name) = path.file_name().and_then(|value| value.to_str()) else {
                    continue;
                };
                if BUCKET_FILE_NAMES.contains(&name) {
                    findings.push(format!(
                        "{} uses bucket filename `{name}` instead of naming its semantic responsibility",
                        path.display()
                    ));
                }
            }
        });
    }
    assert!(findings.is_empty(), "{}", findings.join("\n"));
}

#[test]
fn phase_18_required_responsibility_homes_exist() {
    let root = workspace_root();
    let required = [
        "crates/worth-ui-runtime/src/runtime/allocation_receipt/committed_truth",
        "crates/worth-ui-runtime/src/runtime/allocation_receipt/transaction",
        "crates/worth-ui-runtime/src/runtime/allocation_receipt/reuse",
        "crates/worth-ui-runtime/src/runtime/allocation_receipt/report_freshness",
        "crates/worth-ui-runtime/src/runtime/allocation_receipt/ledger_lifecycle",
        "crates/worth-ui-runtime/src/graph/allocation_neighborhood/admission",
        "crates/worth-ui-runtime/src/graph/allocation_neighborhood/membership",
        "crates/worth-ui-runtime/src/graph/allocation_neighborhood/constraint_authority",
        "crates/worth-ui-runtime/src/graph/allocation_neighborhood/replan_selection",
        "crates/worth-ui-runtime/src/graph/allocation_neighborhood/activation_handoff",
        "crates/worth-ui-runtime/src/runtime/invalidation_narrowing/authority",
        "crates/worth-ui-runtime/src/runtime/invalidation_narrowing/sources",
        "crates/worth-ui-runtime/src/runtime/invalidation_narrowing/selection",
        "crates/worth-ui-query-binding/src/prerequisites/basis",
        "crates/worth-ui-query-binding/src/prerequisites/measurement",
        "crates/worth-ui-query-binding/src/prerequisites/allocation",
    ];
    let missing = required
        .iter()
        .map(|path| root.join(path))
        .filter(|path| !path.is_dir())
        .map(|path| path.display().to_string())
        .collect::<Vec<_>>();
    assert!(
        missing.is_empty(),
        "missing Phase 18 responsibility homes:\n{}",
        missing.join("\n")
    );
}
