use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::catalog::TestCatalog;
use crate::classification::CiTestLane;
use crate::product::{smoke_cases, TestProduct};

#[derive(Debug, Clone)]
pub(crate) struct TestPlan {
    product: TestProduct,
    units: Vec<TestExecutionUnit>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) struct TestExecutionUnit {
    identity: String,
    origin: String,
    directory: PathBuf,
    program: String,
    arguments: Vec<String>,
    filtered: bool,
}

impl TestPlan {
    pub(crate) fn build(
        product: &TestProduct,
        catalog: &TestCatalog,
        workspace_root: &Path,
    ) -> Result<Self, String> {
        let mut units = match product {
            TestProduct::Owner { package } => owner(package, catalog, workspace_root)?,
            TestProduct::Smoke => smoke(catalog, workspace_root)?,
            TestProduct::Ui => integration_lane(CiTestLane::Ui, None, catalog, workspace_root),
            TestProduct::Ci {
                lane: selected,
                shard,
            } => match selected {
                CiTestLane::OwnerUnit if shard.is_none() => owner_ci(workspace_root),
                CiTestLane::Structural if shard.is_none() => structural(workspace_root)?,
                CiTestLane::OwnerUnit | CiTestLane::Structural => {
                    return Err(format!("the {selected} partition is not shardable"))
                }
                selected_lane => integration_lane(*selected_lane, *shard, catalog, workspace_root),
            },
        };
        if matches!(product, TestProduct::Ci { .. }) {
            apply_ci_profiles(&mut units);
        }
        Self::new(product.clone(), units)
    }

    fn new(product: TestProduct, mut units: Vec<TestExecutionUnit>) -> Result<Self, String> {
        if units.is_empty() {
            return Err(format!(
                "test product `{}` selected zero units",
                product.name()
            ));
        }
        units.sort_by(|left, right| left.identity.cmp(&right.identity));
        let mut origins = BTreeMap::new();
        for unit in &units {
            if let Some(first) = origins.insert(unit.identity.clone(), unit.origin.clone()) {
                return Err(format!(
                    "duplicate execution unit `{}` from `{first}` and `{}`",
                    unit.identity, unit.origin
                ));
            }
        }
        Ok(Self { product, units })
    }

    pub(crate) fn product_name(&self) -> String {
        self.product.name()
    }

    pub(crate) fn units(&self) -> &[TestExecutionUnit] {
        &self.units
    }
}

fn apply_ci_profiles(units: &mut [TestExecutionUnit]) {
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
        } else if unit.arguments.first().is_some_and(|value| value == "test") {
            unit.arguments
                .splice(1..1, ["--profile".into(), "ci-test".into()]);
        }
    }
}

impl TestExecutionUnit {
    fn cargo(
        identity: String,
        origin: String,
        workspace_root: &Path,
        arguments: Vec<String>,
        filtered: bool,
    ) -> Self {
        Self {
            identity,
            origin,
            directory: workspace_root.to_path_buf(),
            program: "cargo".into(),
            arguments,
            filtered,
        }
    }

    fn command(
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
            filtered: false,
        }
    }

    pub(crate) fn identity(&self) -> &str {
        &self.identity
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

    pub(crate) fn is_filtered(&self) -> bool {
        self.filtered
    }

    pub(crate) fn display_command(&self) -> String {
        std::iter::once(self.program.as_str())
            .chain(self.arguments.iter().map(String::as_str))
            .collect::<Vec<_>>()
            .join(" ")
    }
}

fn owner(
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
            "--no-fail-fast".into(),
        ],
        false,
    )])
}

fn smoke(catalog: &TestCatalog, workspace_root: &Path) -> Result<Vec<TestExecutionUnit>, String> {
    smoke_cases()
        .iter()
        .map(|case| {
            let target = catalog
                .integration_target(case.package, case.target)
                .ok_or_else(|| format!("missing smoke target {}::{}", case.package, case.target))?;
            let mut arguments = cargo_test_target_arguments(target);
            arguments.extend([case.filter.into(), "--".into(), "--exact".into()]);
            Ok(TestExecutionUnit::cargo(
                format!("smoke::{}::{}::{}", case.package, case.target, case.filter),
                "smoke registration".into(),
                workspace_root,
                arguments,
                true,
            ))
        })
        .collect()
}

fn integration_lane(
    selected: CiTestLane,
    shard: Option<(usize, usize)>,
    catalog: &TestCatalog,
    workspace_root: &Path,
) -> Vec<TestExecutionUnit> {
    let targets = catalog
        .targets()
        .iter()
        .filter(|target| target.lane == selected)
        .collect::<Vec<_>>();
    if targets.is_empty() {
        return Vec::new();
    }

    let filter = targets
        .iter()
        .map(|target| format!("binary_id(={}::{})", target.package, target.name))
        .collect::<Vec<_>>()
        .join(" + ");
    let mut features = targets
        .iter()
        .flat_map(|target| {
            target
                .required_features
                .iter()
                .map(|feature| format!("{}/{}", target.package, feature))
        })
        .collect::<Vec<_>>();
    features.sort();
    features.dedup();

    let mut arguments = vec![
        "nextest".into(),
        "run".into(),
        "--workspace".into(),
        "--no-fail-fast".into(),
        "--filterset".into(),
        filter,
    ];
    for feature in features {
        arguments.extend(["--features".into(), feature]);
    }
    if let Some((index, count)) = shard {
        arguments.extend([
            "--partition".into(),
            format!("hash:{}/{}", index + 1, count),
        ]);
    }

    vec![TestExecutionUnit::cargo(
        format!("{selected}::nextest"),
        format!("Cargo targets classified as {selected}"),
        workspace_root,
        arguments,
        false,
    )]
}

fn cargo_test_target_arguments(target: &crate::catalog::TestTarget) -> Vec<String> {
    let mut arguments = vec![
        "test".into(),
        "-q".into(),
        "-p".into(),
        target.package.clone(),
        "--test".into(),
        target.name.clone(),
    ];
    if !target.required_features.is_empty() {
        arguments.extend(["--features".into(), target.required_features.join(",")]);
    }
    arguments
}

fn owner_ci(workspace_root: &Path) -> Vec<TestExecutionUnit> {
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
            false,
        ),
        TestExecutionUnit::cargo(
            "owner-unit::workspace-doctests".into(),
            "Cargo workspace doctests".into(),
            workspace_root,
            ["test", "-q", "--workspace", "--doc"]
                .into_iter()
                .map(str::to_owned)
                .collect(),
            false,
        ),
    ]
}

fn structural(workspace_root: &Path) -> Result<Vec<TestExecutionUnit>, String> {
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

#[cfg(test)]
mod tests;
