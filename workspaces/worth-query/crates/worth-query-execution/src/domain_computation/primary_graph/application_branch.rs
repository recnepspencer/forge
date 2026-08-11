use worth_relational::facade::history::BranchId;
use worth_runtime_bridge::facade::TruthBranchIdentity;

/// Sole ordinary-branch name for the single-branch application limit.
/// All `BranchId("main")` / `"main"` branch construction must route through this
/// module (PB4).
pub(in crate::domain_computation) const PRIMARY_APPLICATION_BRANCH: &str = "main";

pub(in crate::domain_computation) fn primary_relational_branch_id() -> BranchId {
    BranchId(PRIMARY_APPLICATION_BRANCH.to_owned())
}

pub(in crate::domain_computation) fn primary_truth_branch_identity() -> TruthBranchIdentity {
    TruthBranchIdentity::from_relational_branch_id(PRIMARY_APPLICATION_BRANCH)
}

#[cfg(test)]
mod residue {
    use std::path::PathBuf;

    #[test]
    fn no_branch_id_main_literal_outside_this_module() {
        let src_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src");
        let owner = src_root
            .join("domain_computation")
            .join("primary_graph")
            .join("application_branch.rs");
        let mut offenders = Vec::new();
        for path in walk_rs(&src_root) {
            if path == owner {
                continue;
            }
            let Ok(text) = std::fs::read_to_string(&path) else {
                continue;
            };
            if text.contains("BranchId(\"main\")") {
                offenders.push(path.display().to_string());
            }
        }
        assert!(
            offenders.is_empty(),
            "BranchId(\"main\") must live only in application_branch.rs; found: {offenders:?}"
        );
    }

    fn walk_rs(root: &std::path::Path) -> Vec<PathBuf> {
        let mut out = Vec::new();
        let mut stack = vec![root.to_path_buf()];
        while let Some(dir) = stack.pop() {
            let Ok(entries) = std::fs::read_dir(&dir) else {
                continue;
            };
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    stack.push(path);
                } else if path.extension().and_then(|e| e.to_str()) == Some("rs") {
                    out.push(path);
                }
            }
        }
        out
    }
}
