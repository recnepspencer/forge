use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::catalog::TestCatalog;
use crate::classification::CiTestLane;
use crate::product::{smoke_cases, TestProduct};

mod offline_observer_build;
mod structural_product;

use offline_observer_build::offline_observer_build;
use structural_product::structural;

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
    expected_test_count: Option<usize>,
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
            TestProduct::Ui => integration_lane(CiTestLane::Ui, None, catalog, workspace_root)?,
            TestProduct::Mutants => {
                return Err("mutation campaigns execute outside the ordinary test plan".into())
            }
            TestProduct::Ci {
                lane: selected,
                shard,
            } => match selected {
                CiTestLane::OwnerUnit if shard.is_none() => owner_ci(workspace_root),
                CiTestLane::Structural if shard.is_none() => structural(workspace_root)?,
                CiTestLane::OwnerUnit | CiTestLane::Structural => {
                    return Err(format!("the {selected} partition is not shardable"))
                }
                selected_lane => integration_lane(*selected_lane, *shard, catalog, workspace_root)?,
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
        } else if matches!(
            unit.arguments.first().map(String::as_str),
            Some("test" | "build")
        ) {
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
            expected_test_count: None,
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
            "--lib".into(),
            "--bins".into(),
            "--examples".into(),
            "--benches".into(),
            "--no-fail-fast".into(),
        ],
        None,
    )])
}

fn smoke(catalog: &TestCatalog, workspace_root: &Path) -> Result<Vec<TestExecutionUnit>, String> {
    let mut packages = BTreeMap::<&str, Vec<_>>::new();
    for case in smoke_cases() {
        packages.entry(case.package).or_default().push(case);
    }
    packages
        .into_iter()
        .map(|(package, cases)| {
            let mut targets = Vec::new();
            let mut selectors = Vec::new();
            let mut features = BTreeSet::new();
            let mut seen = BTreeSet::new();
            for case in &cases {
                let identity = (case.target, case.filter);
                if !seen.insert(identity) {
                    return Err(format!(
                        "duplicate smoke case {package}::{}::{}",
                        case.target, case.filter
                    ));
                }
                let target = catalog
                    .integration_target(case.package, case.target)
                    .ok_or_else(|| {
                        format!("missing smoke target {}::{}", case.package, case.target)
                    })?;
                if !targets.iter().any(|name| name == &target.name) {
                    targets.push(target.name.clone());
                }
                features.extend(
                    target
                        .required_features
                        .iter()
                        .map(|feature| format!("{package}/{feature}")),
                );
                selectors.push(format!(
                    "(binary_id(={}::{}) & test(={}))",
                    case.package, case.target, case.filter
                ));
            }
            let mut arguments = vec![
                "nextest".into(),
                "run".into(),
                "-p".into(),
                package.into(),
                "--no-fail-fast".into(),
            ];
            for target in targets {
                arguments.extend(["--test".into(), target]);
            }
            for feature in features {
                arguments.extend(["--features".into(), feature]);
            }
            arguments.extend(["--filterset".into(), selectors.join(" + ")]);
            Ok(TestExecutionUnit::cargo(
                format!("smoke::{package}"),
                "smoke registration".into(),
                workspace_root,
                arguments,
                Some(cases.len()),
            ))
        })
        .collect()
}

fn integration_lane(
    selected: CiTestLane,
    shard: Option<(usize, usize)>,
    catalog: &TestCatalog,
    workspace_root: &Path,
) -> Result<Vec<TestExecutionUnit>, String> {
    let targets = catalog
        .targets()
        .iter()
        .filter(|target| target.lane == selected)
        .collect::<Vec<_>>();
    if targets.is_empty() {
        return Ok(Vec::new());
    }

    let target_names = targets
        .iter()
        .map(|target| target.name.as_str())
        .collect::<BTreeSet<_>>();
    for name in &target_names {
        if let Some(conflict) = catalog
            .targets()
            .iter()
            .find(|target| target.name == *name && target.lane != selected)
        {
            return Err(format!(
                "Cargo target name `{name}` crosses the {selected} and {} lanes at {}::{}; exact workspace target selection would compile the wrong lane",
                conflict.lane, conflict.package, conflict.name
            ));
        }
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
    for target in target_names {
        arguments.extend(["--test".into(), target.into()]);
    }
    for feature in features {
        arguments.extend(["--features".into(), feature]);
    }
    if let Some((index, count)) = shard {
        arguments.extend([
            "--partition".into(),
            format!("hash:{}/{}", index + 1, count),
        ]);
    }

    let tests = TestExecutionUnit::cargo(
        format!("{selected}::nextest"),
        format!("Cargo targets classified as {selected}"),
        workspace_root,
        arguments,
        None,
    );
    if targets
        .iter()
        .any(|target| target.package == "worth-store" && target.name == "physical_record_journeys")
    {
        Ok(vec![offline_observer_build(workspace_root), tests])
    } else {
        Ok(vec![tests])
    }
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

#[cfg(test)]
mod tests;
