use std::collections::BTreeSet;

use crate::selection::{
    select, ProofProcessModel, StoreProofMode, StoreProofRequest, StructuralPreflightReference,
};

use super::{current_inventory, workspace_root};

#[test]
fn ui_partition_uses_only_standardized_harnesses_and_rustdoc_units() {
    let root = workspace_root();
    let inventory = current_inventory(&root);
    let plan = select(
        &root,
        &inventory,
        request("ui-dependency"),
        StructuralPreflightReference::synthetic_for_selection(&root),
    )
    .unwrap();
    assert_eq!(plan.selection.included_products, ["store-ui"]);
    assert!(!plan.units.is_empty());
    assert!(plan.units.iter().all(|unit| matches!(
        unit.process_model,
        ProofProcessModel::StandardizedUiHarness | ProofProcessModel::RustdocTestProcess
    )));
}

#[test]
fn shard_plans_are_disjoint_complete_and_share_one_topology_identity() {
    let root = workspace_root();
    let inventory = current_inventory(&root);
    let unsharded = select(
        &root,
        &inventory,
        request("scenario-certification"),
        StructuralPreflightReference::synthetic_for_selection(&root),
    )
    .unwrap();
    let shard = |index| {
        select(
            &root,
            &inventory,
            request("scenario-certification").with_shard(Some(index), Some(2)),
            StructuralPreflightReference::synthetic_for_selection(&root),
        )
        .unwrap()
    };
    let left = shard(0);
    let right = shard(1);
    let left_ids: BTreeSet<_> = left.units.iter().map(|unit| unit.identity()).collect();
    let right_ids: BTreeSet<_> = right.units.iter().map(|unit| unit.identity()).collect();
    let all_ids: BTreeSet<_> = unsharded.units.iter().map(|unit| unit.identity()).collect();
    assert!(left_ids.is_disjoint(&right_ids));
    assert_eq!(
        left_ids.union(&right_ids).cloned().collect::<BTreeSet<_>>(),
        all_ids
    );
    assert_eq!(
        left.ci_shard_plan.as_ref().unwrap().plan_identity,
        right.ci_shard_plan.as_ref().unwrap().plan_identity
    );
}

#[test]
fn structural_preflight_partition_is_an_explicit_zero_behavior_unit_product() {
    let root = workspace_root();
    let inventory = current_inventory(&root);
    let plan = select(
        &root,
        &inventory,
        request("structural-preflight"),
        StructuralPreflightReference::synthetic_for_selection(&root),
    )
    .unwrap();
    assert!(plan.units.is_empty());
    assert!(plan.selection.included_products.is_empty());
    assert_eq!(plan.request.partition(), Some("structural-preflight"));
}

#[test]
fn formal_tool_partition_admits_only_its_declared_linux_host() {
    let result = request("formal-external").validate_host();
    assert_eq!(result.is_ok(), cfg!(target_os = "linux"));
    if let Err(denial) = result {
        assert!(denial.to_string().contains("requires linux"));
    }
}

fn request(partition: &str) -> StoreProofRequest {
    StoreProofRequest::new(
        StoreProofMode::Ci,
        None,
        Some(partition.to_owned()),
        None,
        None,
        true,
    )
}
