use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) struct TestExecutionUnit {
    identity: String,
    origin: String,
    directory: PathBuf,
    program: String,
    arguments: Vec<String>,
    expected_test_count: Option<usize>,
}

impl TestExecutionUnit {
    pub(super) fn cargo(
        identity: String,
        origin: String,
        workspace_root: &Path,
        arguments: Vec<String>,
        expected_test_count: Option<usize>,
    ) -> Self {
        Self {
            identity,
            origin,
            directory: workspace_root.to_path_buf(),
            program: "cargo".into(),
            arguments,
            expected_test_count,
        }
    }

    pub(super) fn command(
        identity: &str,
        repository_root: PathBuf,
        program: &str,
        arguments: &[&str],
    ) -> Self {
        Self {
            identity: identity.into(),
            origin: "structural partition".into(),
            directory: repository_root,
            program: program.into(),
            arguments: arguments.iter().map(|value| (*value).into()).collect(),
            expected_test_count: None,
        }
    }

    pub(crate) fn identity(&self) -> &str {
        &self.identity
    }

    pub(super) fn origin(&self) -> &str {
        &self.origin
    }

    pub(crate) fn directory(&self) -> &Path {
        &self.directory
    }

    pub(crate) fn program(&self) -> &str {
        &self.program
    }

    pub(crate) fn arguments(&self) -> &[String] {
        &self.arguments
    }

    pub(crate) const fn expected_test_count(&self) -> Option<usize> {
        self.expected_test_count
    }

    pub(crate) fn display_command(&self) -> String {
        std::iter::once(self.program.as_str())
            .chain(self.arguments.iter().map(String::as_str))
            .collect::<Vec<_>>()
            .join(" ")
    }
}

pub(super) fn apply_ci_profiles(units: &mut [TestExecutionUnit]) {
    for unit in units {
        if unit.program != "cargo" {
            continue;
        }
        if unit
            .arguments
            .starts_with(&["nextest".into(), "run".into()])
        {
            unit.arguments.splice(
                2..2,
                [
                    "--profile".into(),
                    "ci".into(),
                    "--cargo-profile".into(),
                    "ci-test".into(),
                ],
            );
        } else if matches!(
            unit.arguments.first().map(String::as_str),
            Some("test" | "build")
        ) {
            unit.arguments
                .splice(1..1, ["--profile".into(), "ci-test".into()]);
        }
    }
}
