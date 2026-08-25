use super::world::supply_chain::certified_supply_chain_world;
use super::world::supply_chain::SupplyChainScale;
use worth_relational::facade::history::BranchId;

#[test]
fn supply_chain_fork_acquires_source_and_target_head_obligations() {
    let (mut world, _) = certified_supply_chain_world(SupplyChainScale::court());
    let source = BranchId("main".to_owned());
    let target = BranchId("storm".to_owned());
    let (_, basis) = world
        .runtime
        .observe_fork_source(&source)
        .expect("main has a retained source artifact");
    world
        .runtime
        .fork_branch(target.clone(), basis)
        .expect("fork creates a retained target reference");

    assert_eq!(
        world
            .runtime
            .branch_reference_state(&source)
            .expect("source remains registered")
            .head_retention_obligations(),
        1
    );
    assert_eq!(
        world
            .runtime
            .branch_reference_state(&target)
            .expect("target is registered")
            .head_retention_obligations(),
        1
    );
}
