mod build_profile;
mod execution_contract;
mod execution_plan;
mod feature_lane;
mod owner_execution;
mod plan_inventory;
mod process_model;
mod product_selection;
mod proof_mode;
mod repository_identity;
mod scenario_execution;
mod ui_execution;

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

pub use build_profile::StoreBuildProfileIdentity;
pub use execution_contract::{
    ProofExecutionCommand, ProofExecutionIsolation, ProofExecutionResources, ProofFailurePolicy,
    ProofRetryPolicy,
};
pub use execution_plan::{
    ProofExecutionUnit, SelectedProofExecutionPlan, StoreProofSelection,
    StructuralPreflightReference,
};
pub use feature_lane::StoreFeatureLane;
pub use process_model::ProofProcessModel;
pub use proof_mode::{StoreProofMode, StoreProofRequest};
pub use proof_unavailable::ProofProductUnavailable;
pub use repository_identity::RepositoryIdentity;

use crate::discovery::TestTargetIdentity;
use crate::ValidatedProofInventory;
use feature_lane::feature_lane_for_target;
use owner_execution::validate_owner_execution_locality;
use plan_inventory::proof_selection;
mod proof_unavailable;
use product_selection::{
    excluded_products, feature_compatibility_units, validate_selected_product_reachability,
};
use scenario_execution::{
    declared_suite_filters, suite_case_responsibilities, DeclaredScenarioExecution,
};
use ui_execution::{selected_ui_runner_cases, ui_doctest_execution_units};

pub(super) type SelectedCases = BTreeMap<String, BTreeMap<String, BTreeSet<String>>>;

pub fn select(
    workspace_root: &Path,
    inventory: &ValidatedProofInventory,
    request: StoreProofRequest,
    structural_preflight: StructuralPreflightReference,
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
    let ui_product_selected = selected_products.contains("store-ui");
    let selected_case_targets = if request.mode() == StoreProofMode::Ui {
        selected_ui_runner_cases(
            inventory,
            &selected_products,
            &request,
            &suite_responsibilities,
        )?
    } else {
        let mut selected = selected_case_targets(
            inventory,
            &selected_products,
            &request,
            &suite_responsibilities,
        );
        if ui_product_selected {
            merge_selected_cases(
                &mut selected,
                selected_ui_runner_cases(
                    inventory,
                    &selected_products,
                    &request,
                    &suite_responsibilities,
                )?,
            );
        }
        selected
    };
    let mut units = execution_units(inventory, &request, &selected_case_targets, &suite_filters)?;
    if selected_products.contains("store-ci:feature-compatibility") {
        units.extend(feature_compatibility_units(inventory));
    }
    if request.semantic_ci_partition() == Some(crate::ci::CiProofPartitionKind::FormalExternal) {
        units.push(ProofExecutionUnit::formal_tool(workspace_root));
    }
    if ui_product_selected {
        units.extend(ui_doctest_execution_units(inventory, &request));
    }
    validate_owner_execution_locality(&request, &units)?;
    units.sort();
    units.dedup();
    for unit in &mut units {
        unit.bind_workspace(workspace_root, &request);
    }
    let ci_shard_plan = request
        .shard()
        .map(|(selected_shard, shard_count)| {
            let partition = request.partition().unwrap_or("unpartitioned");
            crate::ci::CiShardPlan::lower(partition, &units, shard_count, selected_shard)
                .map_err(ProofProductUnavailable::RepositoryObservation)
        })
        .transpose()?;
    if let Some(shard_plan) = &ci_shard_plan {
        units.retain(|unit| shard_plan.includes(unit));
    }
    let structural_only = request.semantic_ci_partition()
        == Some(crate::ci::CiProofPartitionKind::StructuralPreflight);
    if units.is_empty() && !structural_only {
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
        ci_shard_plan,
        excluded_products(&selected_products),
        repository_identity::observe_repository_identity(workspace_root)?,
        structural_preflight,
    )
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
            if aggregates_entire_target(request)
                && !selected_cases_include_compiler_boundary(inventory, target, selected)
            {
                let process_model = suite_filters
                    .get(&target.identity)
                    .is_some_and(|scenarios| {
                        scenarios
                            .values()
                            .any(|scenario| !scenario.process_model.is_plain_libtest())
                    })
                    .then_some(ProofProcessModel::LibtestWithDeclaredSubprocesses);
                let unit = ProofExecutionUnit::from_target(target, request, None)
                    .with_feature_lane(feature_lane_for_target(inventory, target));
                return Ok(vec![
                    process_model.map_or(unit.clone(), |model| unit.with_process_model(model))
                ]);
            }
            let selected_cases = selected
                .get(&target.identity)
                .expect("selected target carries selected cases");
            let units = execution_filters(
                inventory,
                target,
                selected_cases,
                suite_filters.get(&target.identity),
            )?
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

fn merge_selected_cases(destination: &mut SelectedCases, source: SelectedCases) {
    for (target, responsibilities) in source {
        for (responsibility, cases) in responsibilities {
            destination
                .entry(target.clone())
                .or_default()
                .entry(responsibility)
                .or_default()
                .extend(cases);
        }
    }
}

fn selected_cases_include_compiler_boundary(
    inventory: &ValidatedProofInventory,
    target: &TestTargetIdentity,
    selected: &SelectedCases,
) -> bool {
    let selected_names: BTreeSet<_> = selected
        .get(&target.identity)
        .into_iter()
        .flat_map(|responsibilities| responsibilities.values())
        .flatten()
        .collect();
    inventory.inventory().proofs.iter().any(|proof| {
        proof.case.target_identity.as_deref() == Some(target.identity.as_str())
            && selected_names.contains(&proof.case.identity.case_name)
            && proof.case.compiler_boundary_harness.is_some()
    })
}

fn aggregates_entire_target(request: &StoreProofRequest) -> bool {
    !matches!(request.mode(), StoreProofMode::Smoke | StoreProofMode::Ui)
        && request.scenario_identity().is_none()
}

fn executable_case_filter(filter: String) -> Option<String> {
    (!filter.is_empty()).then_some(filter)
}

fn execution_filters(
    inventory: &ValidatedProofInventory,
    target: &TestTargetIdentity,
    selected: &BTreeMap<String, BTreeSet<String>>,
    declared_suite_filters: Option<&BTreeMap<String, DeclaredScenarioExecution>>,
) -> Result<Vec<DeclaredScenarioExecution>, ProofProductUnavailable> {
    if target.kinds.iter().any(|kind| kind == "doc") {
        return Ok(vec![DeclaredScenarioExecution {
            filter: String::new(),
            process_model: ProofProcessModel::RustdocTestProcess,
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
    Ok(selected
        .values()
        .flat_map(|case_names| {
            case_names
                .iter()
                .map(|case_name| DeclaredScenarioExecution {
                    filter: case_name.clone(),
                    process_model: process_model_for_case(inventory, target, case_name),
                })
        })
        .collect())
}

fn process_model_for_case(
    inventory: &ValidatedProofInventory,
    target: &TestTargetIdentity,
    case_name: &str,
) -> ProofProcessModel {
    let case = inventory.inventory().proofs.iter().find(|proof| {
        proof.case.target_identity.as_deref() == Some(target.identity.as_str())
            && proof.case.identity.case_name == case_name
    });
    if case.is_some_and(|proof| proof.case.compiler_boundary_harness.is_some()) {
        ProofProcessModel::StandardizedUiHarness
    } else if case.is_some_and(|proof| proof.case.launches_nested_cargo) {
        ProofProcessModel::NestedCargoProcess
    } else {
        ProofProcessModel::LibtestProcess
    }
}

#[cfg(test)]
mod tests {
    use super::{
        aggregates_entire_target, executable_case_filter, ProofExecutionUnit, ProofProcessModel,
    };
    use crate::discovery::TestTargetIdentity;
    use crate::selection::{StoreProofMode, StoreProofRequest};

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
        let execution = vec![super::DeclaredScenarioExecution {
            filter: String::new(),
            process_model: ProofProcessModel::RustdocTestProcess,
        }];
        assert_eq!(execution.len(), 1);
        assert_eq!(
            execution[0].process_model,
            ProofProcessModel::RustdocTestProcess
        );
        assert!(executable_case_filter(execution[0].filter.clone()).is_none());

        let request = StoreProofRequest::new(StoreProofMode::Ui, None, None, None, None, true);
        assert!(!aggregates_entire_target(&request));
        let native = ProofExecutionUnit::from_target(&target, &request, None);
        assert_eq!(native.process_model, ProofProcessModel::RustdocTestProcess);
        assert!(!native.process_model.requires_ui_proof_evidence());

        let standardized = native.with_process_model(ProofProcessModel::StandardizedUiHarness);
        assert!(standardized.process_model.requires_ui_proof_evidence());
        assert!(standardized
            .expected_evidence
            .iter()
            .any(|item| item == "ui_proof_run_evidence"));
    }
}
