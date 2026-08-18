use super::phase4_fork_evidence::{
    assert_denial_left_no_reference_residue, capture_reference_evidence,
    certified_supply_chain_world,
};
use super::world::supply_chain::SupplyChainScale;
use worth_relational::facade::history::BranchId;

#[test]
fn inventoried_historical_reads_do_not_move_branch_cells() {
    let (mut world, _) = certified_supply_chain_world(SupplyChainScale::court());
    let before = capture_reference_evidence(
        &mut world.runtime,
        &BranchId("main".to_owned()),
        &BranchId("compat".to_owned()),
        world.commit.commit_id,
    );
    let generation_before = before
        .source_state
        .as_ref()
        .expect("main cell")
        .observation()
        .generation()
        .get();
    let truth_before = before
        .source_state
        .as_ref()
        .expect("main cell")
        .truth_version()
        .as_u64();

    let identity = world.runtime.main_branch_identity();
    assert!(world
        .runtime
        .history()
        .historical_branch_head(identity.branch_id())
        .is_some());
    let _ = world.runtime.snapshots().historical_snapshot();
    assert!(world
        .runtime
        .snapshots()
        .historical_snapshot_for_identity(&identity)
        .is_some());
    assert!(world
        .runtime
        .snapshots()
        .historical_snapshot_for_branch(&BranchId("main".to_owned()))
        .is_some());
    let replay_version = world
        .runtime
        .history()
        .historical_branch_head(&BranchId("main".to_owned()))
        .expect("main has a published catalog receipt")
        .version_id;
    assert!(world
        .runtime
        .history_authority()
        .retain_version_for_replay(replay_version));

    let after = capture_reference_evidence(
        &mut world.runtime,
        &BranchId("main".to_owned()),
        &BranchId("compat".to_owned()),
        world.commit.commit_id,
    );
    assert_denial_left_no_reference_residue(&before, &after);
    assert_eq!(
        after
            .source_state
            .as_ref()
            .expect("main cell after compatibility reads")
            .observation()
            .generation()
            .get(),
        generation_before
    );
    assert_eq!(
        after
            .source_state
            .as_ref()
            .expect("main cell after compatibility reads")
            .truth_version()
            .as_u64(),
        truth_before
    );
}

#[test]
fn publication_cannot_mint_a_missing_branch_cell() {
    let (mut world, _) = certified_supply_chain_world(SupplyChainScale::court());
    assert!(matches!(
        world.runtime.branch_identity(&BranchId("ghost".to_owned())),
        Err(_)
    ));
    let identity = world.runtime.main_branch_identity();
    let options = world
        .runtime
        .transaction_options_for(&identity)
        .expect("main remains owner-admissible");
    let before = capture_reference_evidence(
        &mut world.runtime,
        &BranchId("main".to_owned()),
        &BranchId("ghost".to_owned()),
        world.commit.commit_id,
    );
    let mut transaction = world.runtime.begin_transaction(options);
    transaction.push_batch(
        worth_relational::facade::transactions::WorkerIntentBatch::new("cannot-create-ghost"),
    );
    transaction
        .commit()
        .expect("main publication still advances the admitted main cell");
    let after = capture_reference_evidence(
        &mut world.runtime,
        &BranchId("main".to_owned()),
        &BranchId("ghost".to_owned()),
        world.commit.commit_id,
    );
    assert!(after.target_state.is_none());
    assert!(world
        .runtime
        .branch_identity(&BranchId("ghost".to_owned()))
        .is_err());
    assert_eq!(after.catalog_count, before.catalog_count + 1);
}
