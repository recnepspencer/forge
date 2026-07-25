use std::path::Path;

use crate::catalog::TestCatalog;

use super::TestExecutionUnit;

pub(super) fn owner(
    package: &str,
    catalog: &TestCatalog,
    workspace_root: &Path,
) -> Result<Vec<TestExecutionUnit>, String> {
    if !catalog.contains_package(package) {
        return Err(format!("unknown Worth Store workspace package `{package}`"));
    }
    Ok(vec![TestExecutionUnit::cargo(
        format!("owner::{package}"),
        "owner product".into(),
        workspace_root,
        vec![
            "nextest".into(),
            "run".into(),
            "-p".into(),
            package.into(),
            "--lib".into(),
            "--bins".into(),
            "--examples".into(),
            "--benches".into(),
            "--no-fail-fast".into(),
        ],
        None,
    )])
}

pub(super) fn owner_ci(workspace_root: &Path) -> Vec<TestExecutionUnit> {
    vec![
        TestExecutionUnit::cargo(
            "owner-unit::workspace-targets".into(),
            "Cargo workspace unit targets".into(),
            workspace_root,
            [
                "nextest",
                "run",
                "--workspace",
                "--lib",
                "--bins",
                "--examples",
                "--benches",
                "--no-fail-fast",
            ]
            .into_iter()
            .map(str::to_owned)
            .collect(),
            None,
        ),
        TestExecutionUnit::cargo(
            "owner-unit::workspace-doctests".into(),
            "Cargo workspace doctests".into(),
            workspace_root,
            ["test", "-q", "--workspace", "--doc"]
                .into_iter()
                .map(str::to_owned)
                .collect(),
            None,
        ),
    ]
}
