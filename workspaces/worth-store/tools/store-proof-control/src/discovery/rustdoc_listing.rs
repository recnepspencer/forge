use std::collections::BTreeMap;
use std::path::Path;
use std::process::Command;

use super::executable_listing::{CurrentExecutableListing, ExecutableTargetListing};
use super::{CaseKind, TestCaseSurface, TestSurfaceInventory, TestTargetIdentity};

pub(super) fn observe_rustdoc_listings(
    workspace_root: &Path,
    inventory: &TestSurfaceInventory,
) -> Result<Vec<ExecutableTargetListing>, String> {
    let mut listings = Vec::new();
    for target in inventory
        .targets
        .iter()
        .filter(|target| target.kinds.iter().any(|kind| kind == "doc"))
    {
        listings.push(observe_rustdoc_target(workspace_root, target)?);
    }
    Ok(listings)
}

fn observe_rustdoc_target(
    workspace_root: &Path,
    target: &TestTargetIdentity,
) -> Result<ExecutableTargetListing, String> {
    let mut arguments = vec![
        "test".to_owned(),
        "-p".to_owned(),
        target.package.clone(),
        "--doc".to_owned(),
    ];
    if !target.required_features.is_empty() {
        arguments.extend(["--features".to_owned(), target.required_features.join(",")]);
    }
    arguments.extend([
        "--".to_owned(),
        "--list".to_owned(),
        "--format".to_owned(),
        "terse".to_owned(),
    ]);
    let output = Command::new("cargo")
        .args(arguments)
        .current_dir(workspace_root)
        .output()
        .map_err(|error| {
            format!(
                "could not launch rustdoc listing for {}: {error}",
                target.identity
            )
        })?;
    if !output.status.success() {
        return Err(format!(
            "rustdoc listing failed for {}: {}",
            target.identity,
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    Ok(ExecutableTargetListing {
        target_identity: target.identity.clone(),
        listed_cases: super::executable_listing::parse_harness_listing(&output.stdout),
    })
}

pub(super) fn validate_rustdoc_listing(
    inventory: &TestSurfaceInventory,
    listing: &CurrentExecutableListing,
    violations: &mut Vec<String>,
) {
    let observed: BTreeMap<_, _> = listing
        .rustdoc_targets
        .iter()
        .map(|target| (target.target_identity.as_str(), target))
        .collect();
    for target in inventory
        .targets
        .iter()
        .filter(|target| target.kinds.iter().any(|kind| kind == "doc"))
    {
        let Some(listed) = observed.get(target.identity.as_str()) else {
            violations.push(format!(
                "rustdoc target is absent from executable listing: {}",
                target.identity
            ));
            continue;
        };
        validate_target_cases(inventory, target, listed, violations);
    }
    for unknown in observed.keys().filter(|identity| {
        !inventory
            .targets
            .iter()
            .any(|target| &target.identity == **identity)
    }) {
        violations.push(format!("rustdoc listing names unknown target: {unknown}"));
    }
}

fn validate_target_cases(
    inventory: &TestSurfaceInventory,
    target: &TestTargetIdentity,
    listing: &ExecutableTargetListing,
    violations: &mut Vec<String>,
) {
    let expected = expected_case_locations(inventory, target);
    let observed = observed_case_locations(&listing.listed_cases);
    for malformed in listing
        .listed_cases
        .iter()
        .filter(|listed| rustdoc_case_location(listed).is_none())
    {
        violations.push(format!(
            "rustdoc case has no parseable source location in {}: {malformed}",
            target.identity
        ));
    }
    for (location, count) in &expected {
        if observed.get(location) != Some(count) {
            violations.push(format!(
                "source-discovered doctest is absent or duplicated in {} at {}:{}",
                target.identity, location.0, location.1
            ));
        }
    }
    for (location, count) in &observed {
        if expected.get(location) != Some(count) {
            violations.push(format!(
                "rustdoc case has no exact source discovery identity in {} at {}:{}",
                target.identity, location.0, location.1
            ));
        }
    }
}

fn expected_case_locations(
    inventory: &TestSurfaceInventory,
    target: &TestTargetIdentity,
) -> BTreeMap<(String, usize), usize> {
    let mut locations = BTreeMap::new();
    for case in inventory
        .cases
        .iter()
        .filter(|case| is_doctest(case))
        .filter(|case| case.identity.package == target.package)
        .filter(|case| {
            case.required_features
                .iter()
                .all(|feature| target.required_features.contains(feature))
        })
    {
        let location = (
            relative_source_path(&inventory.workspace_root, &case.source_path),
            case.source_line,
        );
        *locations.entry(location).or_default() += 1;
    }
    locations
}

fn observed_case_locations(listed_cases: &[String]) -> BTreeMap<(String, usize), usize> {
    let mut locations = BTreeMap::new();
    for listed in listed_cases {
        let Some((path, line)) = rustdoc_case_location(listed) else {
            continue;
        };
        *locations.entry((path, line)).or_default() += 1;
    }
    locations
}

fn rustdoc_case_location(listed: &str) -> Option<(String, usize)> {
    let (path, rest) = listed.split_once(" - ")?;
    let line = rest
        .rsplit_once("(line ")?
        .1
        .strip_suffix(')')?
        .parse()
        .ok()?;
    Some((path.replace('\\', "/"), line))
}

fn relative_source_path(workspace_root: &str, source_path: &str) -> String {
    source_path
        .replace('\\', "/")
        .strip_prefix(&format!("{}/", workspace_root.replace('\\', "/")))
        .unwrap_or(source_path)
        .replace('\\', "/")
}

fn is_doctest(case: &TestCaseSurface) -> bool {
    matches!(
        case.kind,
        CaseKind::DoctestRunnable | CaseKind::DoctestCompileFail | CaseKind::DoctestIgnored
    )
}
