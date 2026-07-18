use std::collections::{BTreeMap, BTreeSet};

use super::feature_lane::feature_lane_for_target;
use super::{ProofExecutionUnit, ProofProductUnavailable, SelectedCases, StoreProofRequest};
use crate::discovery::CaseKind;
use crate::ValidatedProofInventory;

pub(super) fn selected_ui_runner_cases(
    inventory: &ValidatedProofInventory,
    products: &BTreeSet<String>,
    request: &StoreProofRequest,
    suite_responsibilities: &BTreeMap<String, String>,
) -> Result<SelectedCases, ProofProductUnavailable> {
    let fixture_targets: BTreeSet<_> = inventory
        .inventory()
        .proofs
        .iter()
        .filter(|proof| proof.case.kind == CaseKind::UiFixture)
        .filter(|proof| !proof.products.is_disjoint(products))
        .filter_map(|proof| proof.case.target_identity.clone())
        .collect();
    let mut selected = SelectedCases::new();
    for target_identity in fixture_targets {
        let runners: Vec<_> = inventory
            .inventory()
            .proofs
            .iter()
            .filter(|proof| {
                proof.case.kind == CaseKind::RustTest
                    && proof.case.target_identity.as_deref() == Some(target_identity.as_str())
                    && proof.case.compiler_boundary_harness.is_some()
            })
            .collect();
        if runners.is_empty() {
            return Err(ProofProductUnavailable::ScenarioTopology(format!(
                "UI fixture target has no declared standardized harness runner: {target_identity}"
            )));
        }
        for runner in runners {
            let responsibility = suite_responsibilities
                .get(&runner.case.identity.stable_id)
                .unwrap_or(&runner.case.identity.responsibility);
            if request.scenario_identity().is_some_and(|requested| {
                requested != format!("{}::{responsibility}", runner.owner.package)
            }) {
                continue;
            }
            selected
                .entry(target_identity.clone())
                .or_default()
                .entry(responsibility.clone())
                .or_default()
                .insert(runner.case.identity.case_name.clone());
        }
    }
    Ok(selected)
}

pub(super) fn ui_doctest_execution_units(
    inventory: &ValidatedProofInventory,
    request: &StoreProofRequest,
) -> Vec<ProofExecutionUnit> {
    inventory
        .inventory()
        .discovered
        .targets
        .iter()
        .filter(|target| {
            target.kinds.iter().any(|kind| kind == "doc")
                && inventory.inventory().proofs.iter().any(|proof| {
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
