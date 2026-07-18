use std::path::Path;

use super::{finding, CleanupFailureMode, StructuralCleanupFinding, FILE_SIZE_CAP_LINES};
use crate::topology::workspace_source_inventory::WorkspaceSourceInventory;

pub(super) fn function_overload_findings(
    inventory: &WorkspaceSourceInventory,
) -> Vec<StructuralCleanupFinding> {
    let mut findings = Vec::new();
    let overload_targets = [
        (
            "O-01",
            "crates/worth-ui-runtime/src/runtime/planning/plan_allocation.rs",
            "plan_allocation_for_pending_activation",
            "plan_allocation",
        ),
        (
            "O-02",
            "crates/worth-ui-runtime/src/facade/entry/app.rs",
            "WorthUiApp",
            "inspect",
        ),
        (
            "O-03",
            "crates/worth-ui-runtime/src/runtime/matching/worth_ui_identity_match_graph_builder",
            "WorthUiIdentityMatchGraphBuilder",
            "build",
        ),
        (
            "O-04",
            "crates/worth-ui-runtime/src/evidence/measurement/projection/inspection_receipt",
            "inspection_receipt",
            "project_measurement_inspection",
        ),
    ];

    for (id, rel_path, owner, transition) in overload_targets {
        let path = inventory.absolute_path(rel_path);
        let line_count = max_rust_surface_lines(inventory, &path);
        if line_count > FILE_SIZE_CAP_LINES {
            findings.push(finding(
                id,
                CleanupFailureMode::FunctionOverload,
                &path,
                7,
                "function_decomposition_diff + parity_test",
                &format!(
                    "{owner} {transition} surface still has a step file spanning {line_count} lines"
                ),
            ));
        }
    }

    findings
}

pub(super) fn file_size_findings(
    inventory: &WorkspaceSourceInventory,
) -> Vec<StructuralCleanupFinding> {
    let hotspot_roots = [
        "crates/worth-ui-runtime/src/runtime/matching/worth_ui_identity_match_graph_builder",
        "crates/worth-ui-runtime/src/evidence/measurement/projection/inspection_receipt",
        "crates/worth-ui-runtime/src/graph/allocation_neighborhood/constraint_pipeline",
        "crates/worth-ui-runtime/src/runtime/launch",
    ];
    let mut offenders = Vec::new();
    for rel in hotspot_roots {
        let root = inventory.absolute_path(rel);
        if !inventory.contains(rel) {
            continue;
        }
        let paths = if inventory.source(&root).is_none() {
            inventory
                .rust_files_under(rel)
                .map(|source| source.absolute_path().to_path_buf())
                .collect()
        } else {
            vec![root]
        };
        for path in paths {
            if is_runtime_test_or_support_path(&path) {
                continue;
            }
            let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
            if name == "host_test_support.rs" {
                continue;
            }
            let lines = count_lines(inventory, &path);
            if lines > FILE_SIZE_CAP_LINES {
                offenders.push((lines, path));
            }
        }
    }
    offenders.sort_by(|left, right| right.0.cmp(&left.0).then_with(|| left.1.cmp(&right.1)));

    if offenders.is_empty() {
        return Vec::new();
    }

    let summary = offenders
        .iter()
        .take(8)
        .map(|(lines, path)| format!("{}:{lines}", path_relative(inventory.root(), path)))
        .collect::<Vec<_>>()
        .join(", ");

    vec![finding(
        "S-01",
        CleanupFailureMode::FileSize,
        inventory.absolute_path("crates/worth-ui-runtime/src/lib.rs"),
        7,
        "file_split_diff_or_exemption",
        &format!(
            "phase-7 decomposition hotspot files exceed {FILE_SIZE_CAP_LINES} lines: {summary}"
        ),
    )]
}

pub(super) fn test_bypass_findings(
    inventory: &WorkspaceSourceInventory,
) -> Vec<StructuralCleanupFinding> {
    let runtime_mod = inventory.absolute_path("crates/worth-ui-runtime/src/runtime/mod.rs");
    let text = read_file(inventory, &runtime_mod);
    let mut findings = Vec::new();

    if text.contains("pub use tests::support::touch_origin_certification_support") {
        findings.push(finding(
            "B-01",
            CleanupFailureMode::TestBypass,
            &runtime_mod,
            6,
            "test_support_visibility_fence",
            "runtime/mod.rs pub-uses touch_origin_certification_support under cfg(test)",
        ));
    }

    if text.contains("pub(crate) use runtime_test_modules::")
        || (text.contains("pub(crate) use tests::") && !text.contains("mod tests;"))
    {
        findings.push(finding(
            "B-02",
            CleanupFailureMode::TestBypass,
            &runtime_mod,
            6,
            "test_support_scope_narrowing",
            "runtime/mod.rs exports test support at crate-private scope from production root",
        ));
    }

    let cert_support =
        inventory.absolute_path("crates/worth-ui-runtime/src/certification_support/mod.rs");
    let cert_text = read_file(inventory, &cert_support);
    if cert_text.contains("include!(") {
        findings.push(finding(
            "B-04",
            CleanupFailureMode::TestBypass,
            &cert_support,
            6,
            "support_single_home_diff",
            "certification_support still include!s runtime/tests modules instead of owning fixtures",
        ));
    }

    let host_mod = inventory.absolute_path("crates/worth-ui-runtime/src/host/mod.rs");
    let host_text = read_file(inventory, &host_mod);
    if host_text.contains("collect_measurement_via_host_lane_for_test")
        && !host_text.contains("#[cfg(test)]")
    {
        findings.push(finding(
            "B-05",
            CleanupFailureMode::TestBypass,
            &host_mod,
            6,
            "host_test_reexport_fence",
            "host production root reexports for_test helpers outside cfg(test)",
        ));
    }

    let lib_rs = inventory.absolute_path("crates/worth-ui-runtime/src/lib.rs");
    let lib_text = read_file(inventory, &lib_rs);
    if lib_text.contains("pub mod certification_support")
        && !lib_text.contains("feature = \"certification-support\"")
    {
        findings.push(finding(
            "B-06",
            CleanupFailureMode::TestBypass,
            &lib_rs,
            6,
            "cert_support_feature_fence",
            "certification_support is unconditionally public on the production runtime crate",
        ));
    }

    findings
}

pub(super) fn read_file(inventory: &WorkspaceSourceInventory, path: &Path) -> String {
    inventory.text(path).to_owned()
}

pub(super) fn max_rust_surface_lines(inventory: &WorkspaceSourceInventory, path: &Path) -> usize {
    if inventory.source(path).is_none() {
        return max_lines_in_rust_dir(inventory, path);
    }
    if inventory.source(path).is_some() {
        return count_lines(inventory, path);
    }
    0
}

pub(super) fn count_same_level_rust_files(
    inventory: &WorkspaceSourceInventory,
    dir: &Path,
) -> usize {
    let relative = dir
        .strip_prefix(inventory.root())
        .expect("directory is inventoried");
    inventory
        .direct_entries_under(relative)
        .filter(|path| path.extension().is_some_and(|extension| extension == "rs"))
        .count()
}

pub(super) fn count_files_matching(
    inventory: &WorkspaceSourceInventory,
    dir: &Path,
    pattern: &str,
) -> usize {
    let relative = dir
        .strip_prefix(inventory.root())
        .expect("directory is inventoried");
    inventory
        .direct_entries_under(relative)
        .filter(|entry| {
            entry
                .file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| {
                    if let Some(prefix) = pattern.strip_suffix("*.rs") {
                        name.starts_with(prefix) && name.ends_with(".rs")
                    } else if let Some(suffix) = pattern.strip_prefix('*') {
                        name.ends_with(suffix)
                    } else {
                        name == pattern
                    }
                })
        })
        .count()
}

fn count_lines(inventory: &WorkspaceSourceInventory, path: &Path) -> usize {
    inventory.text(path).lines().count()
}

fn max_lines_in_rust_dir(inventory: &WorkspaceSourceInventory, dir: &Path) -> usize {
    let relative = dir
        .strip_prefix(inventory.root())
        .expect("directory is inventoried");
    inventory
        .rust_files_under(relative)
        .map(|source| source.text().lines().count())
        .max()
        .unwrap_or(0)
}

fn is_runtime_test_or_support_path(path: &Path) -> bool {
    let text = path.to_string_lossy().replace('\\', "/");
    text.contains("/tests/")
        || text.ends_with("_tests.rs")
        || text.ends_with("_test_support.rs")
        || text.contains("/tests.rs")
        || text.ends_with("/tests/mod.rs")
}

fn path_relative(workspace_root: &Path, path: &Path) -> String {
    path.strip_prefix(workspace_root)
        .unwrap_or(path)
        .display()
        .to_string()
}
