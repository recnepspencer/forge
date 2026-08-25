use std::path::{Path, PathBuf};

use super::workspace_source_inventory::WorkspaceSourceInventory;

const FORBIDDEN_PLANNING_SEMANTIC_CALLS: &[&str] = &[
    "admit_allocation_neighborhood_from_graph(",
    "admit_allocation_constraint_set(",
    "WorthUiAllocationPlanning::new(",
];

pub fn audit_allocation_planning_anti_bypass_boundaries(
    inventory: &WorkspaceSourceInventory,
) -> Vec<String> {
    let mut violations = raw_planning_admission_visibility_violations(inventory);
    violations.extend(host_adapter_planning_semantics_violations(inventory));
    violations.extend(non_owner_runtime_planning_semantics_violations(inventory));
    violations.extend(evidence_local_planning_semantics_violations(inventory));
    violations.sort();
    violations.dedup();
    violations
}

fn raw_planning_admission_visibility_violations(
    inventory: &WorkspaceSourceInventory,
) -> Vec<String> {
    let checks = [
        (
            inventory.absolute_path(
                "crates/worth-ui-runtime/src/graph/allocation_neighborhood/activation_handoff/projection.rs",
            ),
            "pub fn admit_allocation_neighborhood(",
        ),
        (
            inventory.absolute_path(
                "crates/worth-ui-runtime/src/graph/allocation_neighborhood/activation_handoff/projection.rs",
            ),
            "pub fn admit_allocation_neighborhood_from_graph(",
        ),
        (
            inventory.absolute_path(
                "crates/worth-ui-runtime/src/graph/allocation_neighborhood/constraint_authority/admission/constraint_projection.rs",
            ),
            "pub fn admit_allocation_constraint_set(",
        ),
        (
            inventory.absolute_path(
                "crates/worth-ui-runtime/src/obligations/selection/selected_obligation_set.rs",
            ),
            "pub fn admit_allocation_neighborhood(",
        ),
    ];
    checks
        .into_iter()
        .filter_map(|(path, forbidden_signature)| {
            let text = inventory.text(&path);
            text.contains(forbidden_signature).then(|| {
                format!(
                    "{} still exposes `{forbidden_signature}` publicly; raw planning admission must stay sealed behind the runtime planning owner lane",
                    path.display()
                )
            })
        })
        .collect()
}

fn host_adapter_planning_semantics_violations(inventory: &WorkspaceSourceInventory) -> Vec<String> {
    let rust_files = [
        "crates/worth-ui-host-native/src",
        "crates/worth-ui-host-headless/src",
    ]
    .into_iter()
    .flat_map(|root| inventory.rust_files_under(root));
    let forbidden_tokens = [
        "WorthUiAllocationPlanning",
        "UiAllocationConstraintSet",
        "UiAllocationNeighborhood",
        "UiAllocationPlanningCostReceipt",
        "UiAllocationSolveTrace",
    ];
    rust_files
        .into_iter()
        .flat_map(|source| {
            let path = source.absolute_path();
            let text = source.text();
            forbidden_tokens.into_iter().filter_map(move |token| {
                if text.contains(token) {
                    Some(format!(
                        "{} reaches planning semantics through `{token}`; host adapters own native mechanics only",
                        path.display()
                    ))
                } else {
                    None
                }
            })
        })
        .collect()
}

fn non_owner_runtime_planning_semantics_violations(
    inventory: &WorkspaceSourceInventory,
) -> Vec<String> {
    let rust_files = inventory.rust_files_under("crates/worth-ui-runtime/src/runtime");
    rust_files
        .into_iter()
        .filter_map(|source| {
            let path = source.absolute_path();
            let relative = relative_runtime_path(inventory, path);
            if is_allowed_planning_owner(&relative) || is_test_file(&relative) {
                return None;
            }
            let text = source.text();
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

fn evidence_local_planning_semantics_violations(
    inventory: &WorkspaceSourceInventory,
) -> Vec<String> {
    let rust_files = inventory.rust_files_under("crates/worth-ui-runtime/src/evidence");
    rust_files
        .into_iter()
        .filter_map(|source| {
            let path = source.absolute_path();
            let relative = path
                .strip_prefix(inventory.absolute_path("crates/worth-ui-runtime/src"))
                .expect("evidence file should sit under worth-ui-runtime/src")
                .to_path_buf();
            if is_test_file(&relative) {
                return None;
            }
            let text = source.text();
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
        || normalized.starts_with("runtime/planning/allocation_planning/")
}

fn is_test_file(relative: &Path) -> bool {
    relative
        .file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| {
            name == "tests.rs" || name.ends_with("_tests.rs") || name.ends_with("_test_support.rs")
        })
}

fn relative_runtime_path(inventory: &WorkspaceSourceInventory, path: &Path) -> PathBuf {
    path.strip_prefix(inventory.absolute_path("crates/worth-ui-runtime/src"))
        .expect("runtime file should sit under worth-ui-runtime/src")
        .to_path_buf()
}

#[cfg(test)]
mod tests {
    use super::{
        audit_allocation_planning_anti_bypass_boundaries, is_allowed_planning_owner,
        WorkspaceSourceInventory,
    };
    use std::path::Path;

    #[test]
    fn planning_owner_allowlist_is_narrow() {
        assert!(is_allowed_planning_owner(Path::new("runtime/host.rs")));
        assert!(is_allowed_planning_owner(Path::new(
            "runtime/planning/allocation_planning/planner.rs"
        )));
        assert!(!is_allowed_planning_owner(Path::new(
            "runtime/planning/plan_topology/assembler.rs"
        )));
    }

    #[test]
    fn workspace_currently_passes_planning_anti_bypass_audit() {
        let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let inventory = WorkspaceSourceInventory::capture(workspace_root);
        let violations = audit_allocation_planning_anti_bypass_boundaries(&inventory);
        assert!(
            violations.is_empty(),
            "allocation planning anti-bypass audit failed: {violations:#?}"
        );
    }
}
