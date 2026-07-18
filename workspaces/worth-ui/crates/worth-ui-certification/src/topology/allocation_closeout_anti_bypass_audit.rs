use std::path::Path;

use super::workspace_source_inventory::WorkspaceSourceInventory;

pub fn audit_allocation_closeout_anti_bypass_boundaries(
    inventory: &WorkspaceSourceInventory,
) -> Vec<String> {
    let mut violations = Vec::new();
    violations.extend(host_allocation_authority_violations(inventory));
    violations.extend(receipt_construction_violations(inventory));
    violations.extend(local_cache_authority_violations(inventory));
    violations.sort();
    violations
}

fn host_allocation_authority_violations(inventory: &WorkspaceSourceInventory) -> Vec<String> {
    inventory
        .rust_files_under("crates/worth-ui-host-egui/src")
        .flat_map(|source| {
            let path = source.absolute_path();
            forbidden_host_authority_patterns(source.text())
                .into_iter()
                .map(move |pattern| {
                    format!(
                        "{} contains `{pattern}`; a host adapter must not construct allocation truth",
                        path.display()
                    )
                })
        })
        .collect()
}

fn forbidden_host_authority_patterns(text: &str) -> Vec<&'static str> {
    [
        "struct UiAllocationReceipt",
        "UiAllocationReceipt::from_candidate(",
        "UiAllocationCandidate",
        "UiCommittedAllocation",
    ]
    .into_iter()
    .filter(|pattern| text.contains(pattern))
    .collect()
}

fn receipt_construction_violations(inventory: &WorkspaceSourceInventory) -> Vec<String> {
    let runtime = inventory.absolute_path("crates/worth-ui-runtime/src");
    inventory
        .rust_files_under("crates/worth-ui-runtime/src")
        .filter_map(|source| {
            let path = source.absolute_path();
            let relative = normalized_relative(&runtime, path);
            if matches!(
                relative.as_str(),
                "runtime/allocation_receipt/committed_truth/committed_receipt.rs"
                    | "runtime/allocation_receipt/transaction/receipt_commit.rs"
            ) {
                return None;
            }
            let text = source.text();
            (text.contains("UiAllocationReceipt::from_candidate(")
                || text.contains("struct UiAllocationReceipt {"))
                .then(|| format!(
                    "{relative} constructs committed allocation truth outside the receipt-commit owner"
                ))
        })
        .collect()
}

fn local_cache_authority_violations(inventory: &WorkspaceSourceInventory) -> Vec<String> {
    let runtime = inventory.absolute_path("crates/worth-ui-runtime/src/runtime");
    inventory
        .rust_files_under("crates/worth-ui-runtime/src/runtime")
        .filter_map(|source| {
            let path = source.absolute_path();
            let relative = normalized_relative(&runtime, path);
            if relative.starts_with("allocation_receipt/ledger_lifecycle/") {
                return None;
            }
            let text = source.text();
            text.contains("committed_by_scope").then(|| {
                format!(
                "runtime/{relative} reaches the receipt ledger cache outside its owner boundary"
            )
            })
        })
        .collect()
}

fn normalized_relative(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .expect("collected file is below root")
        .to_string_lossy()
        .replace('\\', "/")
}

#[cfg(test)]
mod tests {
    #[test]
    fn host_may_consume_a_receipt_without_owning_allocation_authority() {
        let source = "fn paint(receipt: &UiAllocationReceipt) { inspect(receipt); }";
        assert!(super::forbidden_host_authority_patterns(source).is_empty());
    }

    #[test]
    fn host_cannot_construct_or_shadow_allocation_authority() {
        for source in [
            "struct UiAllocationReceipt { generation: u64 }",
            "UiAllocationReceipt::from_candidate(candidate)",
            "fn plan(candidate: UiAllocationCandidate) {}",
            "fn commit(value: UiCommittedAllocation) {}",
        ] {
            assert_eq!(super::forbidden_host_authority_patterns(source).len(), 1);
        }
    }

    #[test]
    fn current_workspace_has_no_allocation_closeout_bypass() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let inventory = super::WorkspaceSourceInventory::capture(root);
        let violations = super::audit_allocation_closeout_anti_bypass_boundaries(&inventory);
        assert!(
            violations.is_empty(),
            "allocation anti-bypass violations: {violations:#?}"
        );
    }
}
