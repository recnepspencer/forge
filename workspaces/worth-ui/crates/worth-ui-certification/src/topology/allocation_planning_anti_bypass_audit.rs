use std::fs;
use std::path::{Path, PathBuf};

const FORBIDDEN_PLANNING_SEMANTIC_CALLS: &[&str] = &[
    "admit_allocation_neighborhood_from_graph(",
    "admit_allocation_constraint_set(",
    "WorthUiAllocationPlanning::new(",
];

pub fn audit_allocation_planning_anti_bypass_boundaries(
    workspace_root: &Path,
) -> Vec<String> {
    let mut violations = raw_planning_admission_visibility_violations(workspace_root);
    violations.extend(host_adapter_planning_semantics_violations(workspace_root));
    violations.extend(non_owner_runtime_planning_semantics_violations(workspace_root));
    violations.extend(evidence_local_planning_semantics_violations(workspace_root));
    violations.sort();
    violations.dedup();
    violations
}

fn raw_planning_admission_visibility_violations(workspace_root: &Path) -> Vec<String> {
    let checks = [
        (
            workspace_root.join(
                "crates/worth-ui-runtime/src/graph/allocation_neighborhood/projection.rs",
            ),
            "pub fn admit_allocation_neighborhood(",
        ),
        (
            workspace_root.join(
                "crates/worth-ui-runtime/src/graph/allocation_neighborhood/projection.rs",
            ),
            "pub fn admit_allocation_neighborhood_from_graph(",
        ),
        (
            workspace_root.join(
                "crates/worth-ui-runtime/src/graph/allocation_neighborhood/constraint_projection.rs",
            ),
            "pub fn admit_allocation_constraint_set(",
        ),
        (
            workspace_root.join(
                "crates/worth-ui-runtime/src/obligations/selection/selected_obligation_set.rs",
            ),
            "pub fn admit_allocation_neighborhood(",
        ),
    ];
    checks
        .into_iter()
        .filter_map(|(path, forbidden_signature)| {
            let text = fs::read_to_string(&path).expect("runtime source should decode");
            text.contains(forbidden_signature).then(|| {
                format!(
                    "{} still exposes `{forbidden_signature}` publicly; raw planning admission must stay sealed behind the runtime planning owner lane",
                    path.display()
                )
            })
        })
        .collect()
}

fn host_adapter_planning_semantics_violations(workspace_root: &Path) -> Vec<String> {
    let host_root = workspace_root.join("crates/worth-ui-host-egui/src");
    let rust_files = collect_rust_files(&host_root);
    let forbidden_tokens = [
        "WorthUiAllocationPlanning",
        "UiAllocationConstraintSet",
        "UiAllocationNeighborhood",
        "UiAllocationPlanningCostReceipt",
        "UiAllocationSolveTrace",
    ];
    rust_files
        .into_iter()
        .flat_map(|path| {
            let text = fs::read_to_string(&path).expect("host source should decode");
            forbidden_tokens.into_iter().filter_map(move |token| {
                text.contains(token).then(|| {
                    format!(
                        "{} reaches planning semantics through `{token}`; host adapters own native mechanics only",
                        path.display()
                    )
                })
            })
        })
        .collect()
}

fn non_owner_runtime_planning_semantics_violations(workspace_root: &Path) -> Vec<String> {
    let runtime_root = workspace_root.join("crates/worth-ui-runtime/src/runtime");
    let rust_files = collect_rust_files(&runtime_root);
    rust_files
        .into_iter()
        .filter_map(|path| {
            let relative = relative_runtime_path(workspace_root, &path);
            if is_allowed_planning_owner(&relative) || is_test_file(&relative) {
                return None;
            }
            let text = fs::read_to_string(&path).expect("runtime source should decode");
            FORBIDDEN_PLANNING_SEMANTIC_CALLS
                .iter()
                .find(|needle| text.contains(**needle))
                .map(|needle| {
                    format!(
                        "{} mints planning semantics through `{needle}` outside the admitted planning owner lane",
                        relative.display()
                    )
                })
        })
        .collect()
}

fn evidence_local_planning_semantics_violations(workspace_root: &Path) -> Vec<String> {
    let evidence_root = workspace_root.join("crates/worth-ui-runtime/src/evidence");
    let rust_files = collect_rust_files(&evidence_root);
    rust_files
        .into_iter()
        .filter_map(|path| {
            let relative = path
                .strip_prefix(workspace_root.join("crates/worth-ui-runtime/src"))
                .expect("evidence file should sit under worth-ui-runtime/src")
                .to_path_buf();
            if is_test_file(&relative) {
                return None;
            }
            let text = fs::read_to_string(&path).expect("evidence source should decode");
            FORBIDDEN_PLANNING_SEMANTIC_CALLS
                .iter()
                .find(|needle| text.contains(**needle))
                .map(|needle| {
                    format!(
                        "{} mints planning semantics through `{needle}` inside runtime evidence; certification must consume the runtime planning owner lane instead",
                        relative.display()
                    )
                })
        })
        .collect()
}

fn is_allowed_planning_owner(relative: &Path) -> bool {
    let normalized = relative.to_string_lossy().replace('\\', "/");
    normalized == "runtime/host.rs"
        || normalized == "runtime/launch/planning_transition.rs"
        || normalized == "runtime/planning/plan_allocation.rs"
        || normalized.starts_with("runtime/allocation_planning/")
}

fn is_test_file(relative: &Path) -> bool {
    relative
        .file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| name.ends_with("_tests.rs") || name.ends_with("_test_support.rs"))
}

fn relative_runtime_path(workspace_root: &Path, path: &Path) -> PathBuf {
    path.strip_prefix(workspace_root.join("crates/worth-ui-runtime/src"))
        .expect("runtime file should sit under worth-ui-runtime/src")
        .to_path_buf()
}

fn collect_rust_files(root: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    collect_rust_files_into(root, &mut files);
    files
}

fn collect_rust_files_into(root: &Path, files: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(root).expect("directory should be readable") {
        let entry = entry.expect("directory entry should read");
        let path = entry.path();
        if path.is_dir() {
            collect_rust_files_into(&path, files);
        } else if path.extension().and_then(|ext| ext.to_str()) == Some("rs") {
            files.push(path);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{audit_allocation_planning_anti_bypass_boundaries, is_allowed_planning_owner};
    use std::path::Path;

    #[test]
    fn planning_owner_allowlist_is_narrow() {
        assert!(is_allowed_planning_owner(Path::new("runtime/host.rs")));
        assert!(is_allowed_planning_owner(Path::new(
            "runtime/allocation_planning/planner.rs"
        )));
        assert!(!is_allowed_planning_owner(Path::new(
            "runtime/plan_topology/assembler.rs"
        )));
    }

    #[test]
    fn workspace_currently_passes_planning_anti_bypass_audit() {
        let workspace_root =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let violations = audit_allocation_planning_anti_bypass_boundaries(&workspace_root);
        assert!(
            violations.is_empty(),
            "allocation planning anti-bypass audit failed: {violations:#?}"
        );
    }
}
