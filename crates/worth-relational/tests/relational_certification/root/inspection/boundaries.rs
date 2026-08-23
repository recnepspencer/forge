use super::root_sharing_observation::inspect_main_regions;
use super::world::supply_chain::{
    assert_oracle_matches, canonical_empty_supply_chain_runtime, certified_supply_chain_world,
    SupplyChainScale,
};
use worth_relational::facade::inspection::RelationalBranchSharingInspectionDenial;
use worth_relational::facade::inspection::RelationalMvccCostScope;

#[test]
fn phase5_inspection_rejects_foreign_and_rootless_identities() {
    let (world, expected) = certified_supply_chain_world(SupplyChainScale::court());
    assert_oracle_matches(&world, &expected);
    let foreign_runtime = world.runtime.fork();
    assert!(matches!(
        foreign_runtime.inspect_branch_sharing(&[world.runtime.main_branch_identity()]),
        Err(RelationalBranchSharingInspectionDenial::ForeignRuntime)
    ));
    let empty_runtime = canonical_empty_supply_chain_runtime(SupplyChainScale::court());
    assert!(matches!(
        empty_runtime.inspect_branch_sharing(&[empty_runtime.main_branch_identity()]),
        Err(RelationalBranchSharingInspectionDenial::RootUnavailable)
    ));
}

#[test]
fn phase5_inspection_rejects_duplicate_branch_scope() {
    let (world, expected) = certified_supply_chain_world(SupplyChainScale::court());
    assert_oracle_matches(&world, &expected);
    let identity = world.runtime.main_branch_identity();
    assert!(matches!(
        world
            .runtime
            .inspect_branch_sharing(&[identity.clone(), identity]),
        Err(RelationalBranchSharingInspectionDenial::DuplicateBranch)
    ));
}

#[test]
fn phase5_region_locators_are_runtime_affine_even_when_storage_is_shared() {
    let (world, expected) = certified_supply_chain_world(SupplyChainScale::court());
    assert_oracle_matches(&world, &expected);
    let source = inspect_main_regions(&world.runtime);
    let foreign = inspect_main_regions(&world.runtime.fork());
    assert_ne!(source, foreign);
    let physical =
        |locators: &[worth_relational::facade::inspection::RelationalStorageRegionLocator]| {
            locators
                .iter()
                .map(|locator| {
                    (
                        locator.root_id(),
                        locator.region_id(),
                        locator.partition_id(),
                    )
                })
                .collect::<Vec<_>>()
        };
    assert_eq!(physical(&source), physical(&foreign));
}

#[test]
fn phase5_public_inspection_artifacts_are_read_only_complete_root_evidence() {
    let (world, expected) = certified_supply_chain_world(SupplyChainScale::court());
    assert_oracle_matches(&world, &expected);
    let identity = world.runtime.main_branch_identity();
    let sharing = world
        .runtime
        .inspect_branch_sharing(std::slice::from_ref(&identity))
        .expect("sharing observation is publicly readable");
    let ledger = world
        .runtime
        .inspect_owner_allocation_ledger(std::slice::from_ref(&identity))
        .expect("owner allocation observation is publicly readable");
    let cost_scope = RelationalMvccCostScope::capture(&world.runtime, vec![identity]);
    let cost = world
        .runtime
        .observe_mvcc_cost(&cost_scope)
        .expect("cost observation is publicly readable");

    assert!(!ledger.authoritative_allocations().is_empty());
    assert_eq!(cost.sharing(), &sharing);
    assert_eq!(sharing.visibility_commitments().len(), 1);
    assert_eq!(
        sharing.visibility_commitments()[0].root_id(),
        sharing.root_ids()[0]
    );
    assert_ne!(sharing.visibility_commitments()[0].digest(), [0; 32]);
}
