use super::workspace_source_inventory::WorkspaceSourceInventory;

const FORBIDDEN_PRODUCT_TOKENS: &[(&str, &str)] = &[
    ("runtime_exports::*", "uncurated runtime topology"),
    ("WorthUiCandidateExecutionPlan", "raw plan construction"),
    ("WorthUiExecutionPlanInput", "raw plan construction"),
    (
        "WorthUiExecutionPlanLoweringAuthority",
        "internal lowering authority",
    ),
    ("WorthUiExecutionPlanDigest", "digest-based plan comparison"),
    ("WorthUiPlanNodeDigest", "digest-based plan comparison"),
    (
        "WorthUiExecutionPlanFrameExecutor",
        "executor plan injection",
    ),
    ("WorthUiLaneFrameExecutor", "executor plan injection"),
    (
        "worth_ui_runtime::runtime",
        "runtime-internal lowerer import",
    ),
];

pub fn audit_product_lifecycle_facade(inventory: &WorkspaceSourceInventory) -> Vec<String> {
    let mut violations: Vec<_> = inventory
        .rust_files_under("crates/worth-ui/src/facade")
        .flat_map(|source| facade_source_violations(source.text(), source.relative_path()))
        .collect();
    violations.sort();
    violations.dedup();
    violations
}

fn facade_source_violations(source: &str, path: &std::path::Path) -> Vec<String> {
    FORBIDDEN_PRODUCT_TOKENS
        .iter()
        .filter(|(token, _)| source.contains(token))
        .map(|(token, reason)| {
            format!(
                "{} exposes `{token}` through the product facade; {reason} must remain runtime-owned",
                path.display()
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::facade_source_violations;
    use std::path::Path;

    #[test]
    fn audit_rejects_each_predecessor_authority_family() {
        let source = r#"
            pub use worth_ui_runtime::runtime::WorthUiExecutionPlanLoweringAuthority;
            pub use worth_ui_runtime::facade::{
                WorthUiCandidateExecutionPlan,
                WorthUiExecutionPlanDigest,
                WorthUiLaneFrameExecutor,
            };
        "#;
        let violations = facade_source_violations(source, Path::new("facade/runtime.rs"));
        assert!(violations
            .iter()
            .any(|row| row.contains("raw plan construction")));
        assert!(violations
            .iter()
            .any(|row| row.contains("digest-based plan comparison")));
        assert!(violations
            .iter()
            .any(|row| row.contains("executor plan injection")));
        assert!(violations
            .iter()
            .any(|row| row.contains("runtime-internal lowerer import")));
    }
}
