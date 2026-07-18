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
        let units = match product {
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

    pub(crate) fn may_run_concurrently(&self) -> bool {
        matches!(
            self.product,
            TestProduct::Ui
                | TestProduct::Ci {
                    lane: CiTestLane::Ui | CiTestLane::Scenario,
                    ..
                }
        )
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
        vec!["test".into(), "-q".into(), "-p".into(), package.into()],
        false,
    )])
}

fn smoke(catalog: &TestCatalog, workspace_root: &Path) -> Result<Vec<TestExecutionUnit>, String> {
    smoke_cases()
        .iter()
        .map(|case| {
            catalog
                .integration_target(case.package, case.target)
                .ok_or_else(|| format!("missing smoke target {}::{}", case.package, case.target))?;
            Ok(TestExecutionUnit::cargo(
                format!("smoke::{}::{}::{}", case.package, case.target, case.filter),
                "smoke registration".into(),
                workspace_root,
                vec![
                    "test".into(),
                    "-q".into(),
                    "-p".into(),
                    case.package.into(),
                    "--test".into(),
                    case.target.into(),
                    case.filter.into(),
                ],
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
    select_shard(&targets, shard)
        .into_iter()
        .map(|target| {
            let arguments = vec![
                "test".into(),
                "-q".into(),
                "-p".into(),
                target.package.clone(),
                "--test".into(),
                target.name.clone(),
            ];
            TestExecutionUnit::cargo(
                format!("{selected}::{}::{}", target.package, target.name),
                format!("{}::{} at {}", target.package, target.name, target.source),
                workspace_root,
                arguments,
                false,
            )
        })
        .collect()
}

fn select_shard<T>(items: &[T], shard: Option<(usize, usize)>) -> Vec<&T> {
    match shard {
        None => items.iter().collect(),
        Some((index, count)) => items
            .iter()
            .enumerate()
            .filter_map(|(position, item)| (position % count == index).then_some(item))
            .collect(),
    }
}

fn owner_ci(workspace_root: &Path) -> Vec<TestExecutionUnit> {
    vec![
        TestExecutionUnit::cargo(
            "owner-unit::workspace-targets".into(),
            "Cargo workspace unit targets".into(),
            workspace_root,
            [
                "test",
                "-q",
                "--workspace",
                "--lib",
                "--bins",
                "--examples",
                "--benches",
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
mod tests {
    use std::collections::BTreeSet;
    use std::path::Path;

    use super::{select_shard, TestExecutionUnit, TestPlan};
    use crate::catalog::TestCatalog;
    use crate::classification::CiTestLane;
    use crate::product::TestProduct;

    #[test]
    fn duplicate_identity_names_both_origins() {
        let units = vec![unit("same", "first"), unit("same", "second")];
        let error = TestPlan::new(TestProduct::Smoke, units).unwrap_err();
        assert!(error.contains("first"));
        assert!(error.contains("second"));
    }

    #[test]
    fn stable_shards_are_disjoint_and_converge() {
        let whole = (0..11).collect::<Vec<_>>();
        let mut observed = Vec::new();
        for index in 0..3 {
            observed.extend(select_shard(&whole, Some((index, 3))).into_iter().copied());
        }
        observed.sort();
        assert_eq!(observed, whole);
    }

    #[test]
    fn unknown_owner_is_denied_before_execution() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(Path::parent)
            .unwrap();
        let catalog = TestCatalog::load(root).unwrap();
        let product = TestProduct::Owner {
            package: "worth-store-does-not-exist".into(),
        };
        let error = TestPlan::build(&product, &catalog, root).unwrap_err();
        assert!(error.contains("worth-store-does-not-exist"));
    }

    #[test]
    fn empty_product_is_never_green() {
        let error = TestPlan::new(TestProduct::Smoke, Vec::new()).unwrap_err();
        assert!(error.contains("selected zero units"));
    }

    #[test]
    fn integration_partitions_cover_the_current_catalog_exactly() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(Path::parent)
            .unwrap();
        let catalog = TestCatalog::load(root).unwrap();
        for lane in [CiTestLane::Scenario, CiTestLane::Ui, CiTestLane::Formal] {
            let product = TestProduct::Ci { lane, shard: None };
            let plan = TestPlan::build(&product, &catalog, root).unwrap();
            let actual = plan
                .units()
                .iter()
                .map(|unit| unit.identity().to_owned())
                .collect::<BTreeSet<_>>();
            let expected = catalog
                .targets()
                .iter()
                .filter(|target| target.lane == lane)
                .map(|target| format!("{lane}::{}::{}", target.package, target.name))
                .collect::<BTreeSet<_>>();
            assert_eq!(actual, expected, "{lane} partition drifted");
        }
    }

    fn unit(identity: &str, origin: &str) -> TestExecutionUnit {
        TestExecutionUnit::cargo(
            identity.into(),
            origin.into(),
            Path::new("."),
            vec!["test".into()],
            false,
        )
    }
}
