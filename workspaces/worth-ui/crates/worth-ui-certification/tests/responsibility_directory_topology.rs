use std::collections::BTreeMap;
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

const RESPONSIBILITY_ROOTS: &[&str] = &[
    "crates/worth-ui-runtime/src/runtime/allocation_receipt",
    "crates/worth-ui-runtime/src/graph/allocation_neighborhood",
    "crates/worth-ui-runtime/src/runtime/invalidation_narrowing",
    "crates/worth-ui-runtime/src/runtime/planning/plan_topology/region",
    "crates/worth-ui-runtime/src/runtime/execution/canvas_spatial_lane",
    "crates/worth-ui-runtime/src/runtime/execution/realtime_overlay_lane",
];

const REQUIRED_RESPONSIBILITY_HOMES: &[&str] = &[
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
    "crates/worth-ui-runtime/src/runtime/planning/plan_topology/region/identity_index",
    "crates/worth-ui-runtime/src/runtime/planning/plan_topology/region/slot_index",
    "crates/worth-ui-runtime/src/runtime/planning/plan_topology/region/executable_schema",
    "crates/worth-ui-runtime/src/runtime/planning/plan_topology/region/persistent_storage",
    "crates/worth-ui-runtime/src/runtime/planning/plan_topology/region/replacement_transition",
    "crates/worth-ui-runtime/src/runtime/planning/plan_topology/region/successor_construction",
    "crates/worth-ui-runtime/src/runtime/execution/canvas_spatial_lane/plan_contract",
    "crates/worth-ui-runtime/src/runtime/execution/canvas_spatial_lane/frame_execution",
    "crates/worth-ui-runtime/src/runtime/execution/canvas_spatial_lane/spatial_request",
    "crates/worth-ui-runtime/src/runtime/execution/realtime_overlay_lane/plan_contract",
    "crates/worth-ui-runtime/src/runtime/execution/realtime_overlay_lane/frame_execution",
    "crates/worth-ui-runtime/src/runtime/execution/realtime_overlay_lane/renderer_surface",
];

#[test]
fn responsibility_directories_remain_bounded_and_semantic() {
    let inventory = super::workspace_source_inventory();
    let mut rust_files_by_directory: BTreeMap<PathBuf, Vec<PathBuf>> = BTreeMap::new();
    for root in RESPONSIBILITY_ROOTS {
        for source in inventory.rust_files_under(root) {
            if source
                .relative_path()
                .components()
                .any(|component| component.as_os_str() == "tests")
            {
                continue;
            }
            let directory = source
                .relative_path()
                .parent()
                .expect("Rust source should have a parent")
                .to_path_buf();
            rust_files_by_directory
                .entry(directory)
                .or_default()
                .push(source.relative_path().to_path_buf());
        }
    }

    let mut findings = Vec::new();
    for (directory, rust_files) in rust_files_by_directory {
        if rust_files.len() > MAX_DIRECT_RUST_FILES {
            findings.push(format!(
                "{} contains {} direct Rust files; at most {} are permitted",
                inventory.absolute_path(&directory).display(),
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
                    inventory.absolute_path(&path).display()
                ));
            }
        }
    }
    assert!(findings.is_empty(), "{}", findings.join("\n"));
}

#[test]
fn required_responsibility_homes_exist() {
    let inventory = super::workspace_source_inventory();
    let missing = REQUIRED_RESPONSIBILITY_HOMES
        .iter()
        .map(Path::new)
        .filter(|path| !inventory.contains(path))
        .map(|path| inventory.absolute_path(path).display().to_string())
        .collect::<Vec<_>>();
    assert!(
        missing.is_empty(),
        "missing responsibility homes:\n{}",
        missing.join("\n")
    );
}
