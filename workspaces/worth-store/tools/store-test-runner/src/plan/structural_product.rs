use std::path::Path;

use super::TestExecutionUnit;

pub(super) fn structural(workspace_root: &Path) -> Result<Vec<TestExecutionUnit>, String> {
    let repository_root = workspace_root
        .parent()
        .and_then(Path::parent)
        .ok_or_else(|| "Worth Store workspace is not nested under workspaces/".to_owned())?
        .to_path_buf();
    Ok(vec![
        TestExecutionUnit::command(
            "structural::boundary-check",
            repository_root.clone(),
            "cargo",
            &[
                "run",
                "--manifest-path",
                "tools/boundary-check/Cargo.toml",
                "--",
                "--root",
                ".",
            ],
        ),
        TestExecutionUnit::command(
            "structural::agent-context",
            repository_root.clone(),
            "cargo",
            &[
                "run",
                "--manifest-path",
                "tools/agent-context/Cargo.toml",
                "--",
                "check",
            ],
        ),
        TestExecutionUnit::command(
            "structural::line-caps",
            repository_root,
            "bash",
            &["scripts/ci/check_workspace_rust_line_caps.sh"],
        ),
    ])
}
