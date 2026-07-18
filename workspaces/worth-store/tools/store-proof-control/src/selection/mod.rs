mod build_profile;
mod execution_plan;
mod feature_lane;
mod owner_execution;
mod plan_inventory;
mod proof_mode;
mod repository_identity;
mod scenario_execution;

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

pub use build_profile::StoreBuildProfileIdentity;
pub use execution_plan::{ProofExecutionUnit, SelectedProofExecutionPlan, StoreProofSelection};
pub use feature_lane::StoreFeatureLane;
pub use proof_mode::{ProofProductUnavailable, StoreProofMode, StoreProofRequest};
pub use repository_identity::RepositoryIdentity;

use crate::discovery::TestTargetIdentity;
use crate::ValidatedProofInventory;
use feature_lane::feature_lane_for_target;
use owner_execution::validate_owner_execution_locality;
use plan_inventory::proof_selection;
use scenario_execution::{
    declared_suite_filters, suite_case_responsibilities, DeclaredScenarioExecution,
};

pub(super) type SelectedCases = BTreeMap<String, BTreeMap<String, BTreeSet<String>>>;

pub fn select(
    workspace_root: &Path,
    inventory: &ValidatedProofInventory,
    request: StoreProofRequest,
) -> Result<SelectedProofExecutionPlan, ProofProductUnavailable> {
    let selected_products = request.selected_product_names(inventory)?;
    request.validate_host()?;
    validate_selected_product_reachability(inventory, &selected_products)?;
    let suite_inventory = crate::classification::build_consolidated_suite_inventory(
        workspace_root,
        inventory.inventory(),
    )
    .map_err(|violations| ProofProductUnavailable::ScenarioTopology(violations.join("\n  - ")))?;
    let suite_filters = declared_suite_filters(&suite_inventory);
    let suite_responsibilities = suite_case_responsibilities(&suite_inventory);
    let selected_case_targets = if request.mode() == StoreProofMode::Ui {
        SelectedCases::new()
    } else {
        selected_case_targets(
            inventory,
            &selected_products,
            &request,
            &suite_responsibilities,
        )
    };
    let mut units = execution_units(inventory, &request, &selected_case_targets, &suite_filters)?;
    if selected_products.contains("store-ci:feature-compatibility") {
        units.extend(feature_compatibility_units(inventory));
    }
    if request.mode() == StoreProofMode::Ui {
        units.extend(ui_execution_units(inventory, &request));
    }
    validate_owner_execution_locality(&request, &units)?;
    units.sort();
    units.dedup();
    if units.is_empty() {
        return Err(ProofProductUnavailable::NoReachableProof {
            product: request.display_name(),
        });
    }
    SelectedProofExecutionPlan::lower(
        workspace_root,
        request,
        proof_selection(
            inventory,
            &selected_products,
            &selected_case_targets,
            &units,
        ),
        units,
        excluded_products(&selected_products),
        repository_identity::observe_repository_identity(workspace_root)?,
    )
}

fn validate_selected_product_reachability(
    inventory: &ValidatedProofInventory,
    products: &BTreeSet<String>,
) -> Result<(), ProofProductUnavailable> {
    for product in products {
        if product == "store-ci:feature-compatibility" {
            continue;
        }
        if !inventory
            .inventory()
            .proofs
            .iter()
            .any(|proof| proof.products.contains(product))
        {
            return Err(ProofProductUnavailable::MissingRequiredProofProduct(
                product.clone(),
            ));
        }
    }
    Ok(())
}

fn feature_compatibility_units(inventory: &ValidatedProofInventory) -> Vec<ProofExecutionUnit> {
    inventory
        .inventory()
        .discovered
        .packages
        .iter()
        .filter(|package| package.name.starts_with("worth-store"))
        .flat_map(|package| {
            let production =
                ProofExecutionUnit::feature_compatibility(package.name.clone(), Vec::new());
            let named = package
                .features
                .iter()
                .filter(|feature| *feature != "default")
                .map(|feature| {
                    ProofExecutionUnit::feature_compatibility(
                        package.name.clone(),
                        vec![feature.clone()],
                    )
                });
            std::iter::once(production).chain(named).collect::<Vec<_>>()
        })
        .collect()
}

fn selected_case_targets(
    inventory: &ValidatedProofInventory,
    products: &BTreeSet<String>,
    request: &StoreProofRequest,
    suite_responsibilities: &BTreeMap<String, String>,
) -> SelectedCases {
    let mut targets = SelectedCases::new();
    for proof in &inventory.inventory().proofs {
        if matches!(
            proof.case.kind,
            crate::discovery::CaseKind::UiFixture
                | crate::discovery::CaseKind::DoctestCompileFail
                | crate::discovery::CaseKind::DoctestIgnored
        ) {
            continue;
        }
        if proof.products.is_disjoint(products) {
            continue;
        }
        let responsibility = suite_responsibilities
            .get(&proof.case.identity.stable_id)
            .unwrap_or(&proof.case.identity.responsibility);
        if request.scenario_identity().is_some_and(|requested| {
            requested != format!("{}::{responsibility}", proof.owner.package)
        }) {
            continue;
        }
        if let Some(target) = &proof.case.target_identity {
            targets
                .entry(target.clone())
                .or_default()
                .entry(responsibility.clone())
                .or_default()
                .insert(proof.case.identity.case_name.clone());
        }
    }
    targets
}

fn execution_units(
    inventory: &ValidatedProofInventory,
    request: &StoreProofRequest,
    selected: &SelectedCases,
    suite_filters: &BTreeMap<String, BTreeMap<String, DeclaredScenarioExecution>>,
) -> Result<Vec<ProofExecutionUnit>, ProofProductUnavailable> {
    inventory
        .inventory()
        .discovered
        .targets
        .iter()
        .filter(|target| selected.contains_key(&target.identity))
        .map(|target| {
            if request.mode() != StoreProofMode::Smoke && request.scenario_identity().is_none() {
                let process_model = suite_filters
                    .get(&target.identity)
                    .is_some_and(|scenarios| {
                        scenarios
                            .values()
                            .any(|scenario| scenario.process_model != "libtest-process")
                    })
                    .then_some("libtest-with-declared-subprocesses");
                let unit = ProofExecutionUnit::from_target(target, request, None)
                    .with_feature_lane(feature_lane_for_target(inventory, target));
                return Ok(vec![
                    process_model.map_or(unit.clone(), |model| unit.with_process_model(model))
                ]);
            }
            let selected_cases = selected
                .get(&target.identity)
                .expect("selected target carries selected cases");
            let units =
                execution_filters(target, selected_cases, suite_filters.get(&target.identity))?
                    .into_iter()
                    .map(|execution| {
                        let filter = executable_case_filter(execution.filter);
                        ProofExecutionUnit::from_target(target, request, filter)
                            .with_feature_lane(feature_lane_for_target(inventory, target))
                            .with_process_model(execution.process_model)
                    })
                    .collect::<Vec<_>>();
            Ok(units)
        })
        .collect::<Result<Vec<_>, _>>()
        .map(|units| units.into_iter().flatten().collect())
}

fn executable_case_filter(filter: String) -> Option<String> {
    (!filter.is_empty()).then_some(filter)
}

fn execution_filters(
    target: &TestTargetIdentity,
    selected: &BTreeMap<String, BTreeSet<String>>,
    declared_suite_filters: Option<&BTreeMap<String, DeclaredScenarioExecution>>,
) -> Result<Vec<DeclaredScenarioExecution>, ProofProductUnavailable> {
    if target.kinds.iter().any(|kind| kind == "doc") {
        return Ok(vec![DeclaredScenarioExecution {
            filter: String::new(),
            process_model: "rustdoc-test-process".to_owned(),
        }]);
    }
    if target.source_path.contains("/tests/suites/") {
        let declared = declared_suite_filters.ok_or_else(|| {
            ProofProductUnavailable::ScenarioTopology(format!(
                "suite target has no declarations: {}",
                target.identity
            ))
        })?;
        return selected
            .keys()
            .map(|responsibility| {
                declared.get(responsibility).cloned().ok_or_else(|| {
                    ProofProductUnavailable::ScenarioTopology(format!(
                        "suite {} has no filter for scenario {}",
                        target.identity, responsibility
                    ))
                })
            })
            .collect();
    }
    let process_model = if target.name.contains("compile_fail") {
        "nested-cargo-process"
    } else {
        "libtest-process"
    };
    Ok(selected
        .values()
        .flat_map(|case_names| {
            case_names
                .iter()
                .map(|case_name| DeclaredScenarioExecution {
                    filter: case_name.clone(),
                    process_model: process_model.to_owned(),
                })
        })
        .collect())
}

fn ui_execution_units(
    inventory: &ValidatedProofInventory,
    request: &StoreProofRequest,
) -> Vec<ProofExecutionUnit> {
    inventory
        .inventory()
        .discovered
        .targets
        .iter()
        .filter(|target| {
            inventory.inventory().proofs.iter().any(|proof| {
                proof.case.target_identity.as_deref() == Some(target.identity.as_str())
                    && proof.products.contains("store-ui")
            })
        })
        .map(|target| {
            ProofExecutionUnit::from_target(target, request, None)
                .with_feature_lane(feature_lane_for_target(inventory, target))
        })
        .collect()
}

fn excluded_products(included: &BTreeSet<String>) -> BTreeMap<String, String> {
    let all = [
        "store-owner",
        "store-smoke",
        "store-ui",
        "store-ci",
        "store-soak",
        "store-release",
        "store-hardware",
    ];
    all.into_iter()
        .filter(|product| {
            !included
                .iter()
                .any(|included| included.starts_with(product))
        })
        .map(|product| {
            (
                product.to_owned(),
                "outside the explicitly requested proof product".to_owned(),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{executable_case_filter, execution_filters};
    use crate::discovery::TestTargetIdentity;
    use std::collections::{BTreeMap, BTreeSet};

    #[test]
    fn doctest_selection_cannot_lower_a_synthetic_zero_match_filter() {
        let target = TestTargetIdentity {
            identity: "owner::doc::owner".to_owned(),
            package: "owner".to_owned(),
            name: "owner".to_owned(),
            kinds: vec!["doc".to_owned()],
            source_path: "src/lib.rs".to_owned(),
            required_features: Vec::new(),
        };
        let selected = BTreeMap::from([(
            "owner/docs".to_owned(),
            BTreeSet::from(["doctest_runnable_synthetic".to_owned()]),
        )]);
        let execution = execution_filters(&target, &selected, None).unwrap();
        assert_eq!(execution.len(), 1);
        assert_eq!(execution[0].process_model, "rustdoc-test-process");
        assert!(executable_case_filter(execution[0].filter.clone()).is_none());
    }
}
