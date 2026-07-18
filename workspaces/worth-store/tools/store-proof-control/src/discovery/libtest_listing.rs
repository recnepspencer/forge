use std::collections::{BTreeMap, BTreeSet};

use super::executable_listing::CurrentExecutableListing;
use super::{CaseKind, TestSurfaceInventory};

pub(super) fn validate_libtest_listing(
    inventory: &TestSurfaceInventory,
    listing: &CurrentExecutableListing,
    violations: &mut Vec<String>,
) {
    let known_targets: BTreeSet<_> = inventory
        .targets
        .iter()
        .map(|target| target.identity.as_str())
        .collect();
    let mut observed = BTreeMap::new();
    for target in &listing.libtest_targets {
        if !known_targets.contains(target.target_identity.as_str()) {
            violations.push(format!(
                "libtest listing names unknown target: {}",
                target.target_identity
            ));
        }
        if observed
            .insert(
                target.target_identity.as_str(),
                case_name_counts(target.listed_cases.iter().map(String::as_str)),
            )
            .is_some()
        {
            violations.push(format!(
                "libtest target is listed more than once: {}",
                target.target_identity
            ));
        }
    }
    let mut expected = BTreeMap::<&str, BTreeMap<&str, usize>>::new();
    for case in inventory
        .cases
        .iter()
        .filter(|case| case.kind == CaseKind::RustTest)
    {
        let Some(target) = case.target_identity.as_deref() else {
            continue;
        };
        *expected
            .entry(target)
            .or_default()
            .entry(case.identity.case_name.as_str())
            .or_default() += 1;
    }
    for (target, expected_cases) in &expected {
        let Some(observed_cases) = observed.get(target) else {
            violations.push(format!(
                "registered Rust test target is absent from libtest listing: {target}"
            ));
            continue;
        };
        for case_name in expected_cases
            .keys()
            .chain(observed_cases.keys())
            .collect::<BTreeSet<_>>()
        {
            let expected_count = expected_cases.get(case_name).copied().unwrap_or_default();
            let observed_count = observed_cases.get(case_name).copied().unwrap_or_default();
            if expected_count != observed_count {
                violations.push(format!(
                    "libtest/source multiplicity differs for {target}::{case_name}: expected {expected_count}, observed {observed_count}"
                ));
            }
        }
    }
    for (target, observed_cases) in observed {
        if !observed_cases.is_empty() && !expected.contains_key(target) {
            violations.push(format!(
                "libtest target has executable cases but no source-discovered tests: {target}"
            ));
        }
    }
}

fn case_name_counts<'a>(cases: impl Iterator<Item = &'a str>) -> BTreeMap<&'a str, usize> {
    let mut counts = BTreeMap::new();
    for listed in cases {
        *counts
            .entry(listed.rsplit("::").next().unwrap_or(listed))
            .or_default() += 1;
    }
    counts
}
