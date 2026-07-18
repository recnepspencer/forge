use std::collections::BTreeSet;

use super::{ProofExecutionUnit, SelectedCases, StoreProofSelection};
use crate::ValidatedProofInventory;

pub(super) fn proof_selection(
    inventory: &ValidatedProofInventory,
    products: &BTreeSet<String>,
    case_targets: &SelectedCases,
    units: &[ProofExecutionUnit],
) -> StoreProofSelection {
    let included_packages: BTreeSet<_> = units.iter().map(|unit| unit.package.clone()).collect();
    let excluded_packages = inventory
        .inventory()
        .discovered
        .packages
        .iter()
        .filter(|package| !included_packages.contains(&package.name))
        .map(|package| {
            (
                package.name.clone(),
                "no proof selected from this package for the requested product".to_owned(),
            )
        })
        .collect();
    let included_targets: BTreeSet<_> = units
        .iter()
        .map(|unit| format!("{}::{}", unit.package, unit.target_name))
        .collect();
    let excluded_targets = inventory
        .inventory()
        .discovered
        .targets
        .iter()
        .map(|target| format!("{}::{}", target.package, target.name))
        .filter(|target| !included_targets.contains(target))
        .map(|target| {
            (
                target,
                "target owns no proof selected by this product request".to_owned(),
            )
        })
        .collect();
    let included_fixtures: BTreeSet<_> = inventory
        .inventory()
        .proofs
        .iter()
        .filter(|proof| proof.case.kind == crate::discovery::CaseKind::UiFixture)
        .filter(|proof| !proof.products.is_disjoint(products))
        .map(|proof| proof.case.source_path.clone())
        .collect();
    let excluded_fixtures = inventory
        .inventory()
        .proofs
        .iter()
        .filter(|proof| proof.case.kind == crate::discovery::CaseKind::UiFixture)
        .map(|proof| proof.case.source_path.clone())
        .filter(|fixture| !included_fixtures.contains(fixture))
        .map(|fixture| {
            (
                fixture,
                "fixture is outside the selected compiler-boundary slice".to_owned(),
            )
        })
        .collect();
    let included_suites: BTreeSet<_> = units
        .iter()
        .filter(|unit| is_suite_unit(inventory, unit))
        .map(|unit| unit.target_name.clone())
        .collect();
    let excluded_suites = inventory
        .inventory()
        .discovered
        .targets
        .iter()
        .filter(|target| target.source_path.contains("/tests/suites/"))
        .map(|target| target.name.clone())
        .filter(|suite| !included_suites.contains(suite))
        .map(|suite| {
            (
                suite,
                "suite owns no scenario selected by this product request".to_owned(),
            )
        })
        .collect();
    StoreProofSelection {
        included_products: products.iter().cloned().collect(),
        included_packages: included_packages.into_iter().collect(),
        excluded_packages,
        included_targets: included_targets.into_iter().collect(),
        excluded_targets,
        included_case_responsibilities: case_targets
            .iter()
            .map(|(target, cases)| (target.clone(), cases.keys().cloned().collect()))
            .collect(),
        included_fixtures: included_fixtures.into_iter().collect(),
        excluded_fixtures,
        included_suites: included_suites.into_iter().collect(),
        excluded_suites,
        feature_lanes: units
            .iter()
            .map(|unit| unit.feature_lane.clone())
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect(),
        build_profiles: units
            .iter()
            .map(|unit| unit.build_profile)
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect(),
        subprocess_probes: units
            .iter()
            .filter(|unit| unit.process_model != "libtest-process")
            .map(|unit| {
                format!(
                    "{}::{}={}",
                    unit.package, unit.target_name, unit.process_model
                )
            })
            .collect(),
    }
}

fn is_suite_unit(inventory: &ValidatedProofInventory, unit: &ProofExecutionUnit) -> bool {
    inventory
        .inventory()
        .discovered
        .targets
        .iter()
        .any(|target| {
            target.package == unit.package
                && target.name == unit.target_name
                && target.source_path.contains("/tests/suites/")
        })
}
