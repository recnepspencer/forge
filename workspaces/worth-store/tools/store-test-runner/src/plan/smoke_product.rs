use std::{
    collections::{BTreeMap, BTreeSet},
    path::Path,
};

use crate::{catalog::TestCatalog, product::smoke_cases};

use super::TestExecutionUnit;

pub(super) fn smoke(
    catalog: &TestCatalog,
    workspace_root: &Path,
) -> Result<Vec<TestExecutionUnit>, String> {
    let mut packages = BTreeMap::<&str, Vec<_>>::new();
    for case in smoke_cases() {
        packages.entry(case.package).or_default().push(case);
    }
    packages
        .into_iter()
        .map(|(package, cases)| smoke_package(package, &cases, catalog, workspace_root))
        .collect()
}

fn smoke_package(
    package: &str,
    cases: &[&crate::product::SmokeCase],
    catalog: &TestCatalog,
    workspace_root: &Path,
) -> Result<TestExecutionUnit, String> {
    let mut targets = Vec::new();
    let mut selectors = Vec::new();
    let mut features = BTreeSet::new();
    let mut seen = BTreeSet::new();
    for case in cases {
        let identity = (case.target, case.filter);
        if !seen.insert(identity) {
            return Err(format!(
                "duplicate smoke case {package}::{}::{}",
                case.target, case.filter
            ));
        }
        let target = catalog
            .integration_target(case.package, case.target)
            .ok_or_else(|| format!("missing smoke target {}::{}", case.package, case.target))?;
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
    let arguments = smoke_arguments(package, targets, features, selectors);
    Ok(TestExecutionUnit::cargo(
        format!("smoke::{package}"),
        "smoke registration".into(),
        workspace_root,
        arguments,
        Some(cases.len()),
    ))
}

fn smoke_arguments(
    package: &str,
    targets: Vec<String>,
    features: BTreeSet<String>,
    selectors: Vec<String>,
) -> Vec<String> {
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
    arguments
}
