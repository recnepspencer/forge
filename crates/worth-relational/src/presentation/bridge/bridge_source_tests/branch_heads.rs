use std::sync::{Arc, Barrier, Mutex};

use worth_runtime_bridge::facade::{
    CommittedPatchSource, RelationalCommittedPatchRequest, TruthBranchHeadSource,
    TruthBranchIdentity, TruthCommitIdentity,
};

use crate::history::data::BranchId;
use crate::tests::support::create_entity_outcome;

use super::super::RuntimeBridgeRelationalSource;
use super::support::runtime_with_test_schema;

#[test]
fn branch_head_loading_requires_an_explicit_owner_admitted_binding() {
    let runtime = Arc::new(Mutex::new(runtime_with_test_schema()));
    let source =
        RuntimeBridgeRelationalSource::for_shared_graph_role(Arc::clone(&runtime), "model")
            .unwrap();
    let branch = TruthBranchIdentity::from_relational_branch_id("main");
    let before = runtime.lock().unwrap().branch_basis_cost_counters();

    let error = source
        .load_branch_head_patch(&branch)
        .expect_err("a branch label alone cannot select an owner head");

    assert!(error
        .to_string()
        .contains("no explicitly admitted head basis"));
    assert_eq!(runtime.lock().unwrap().branch_basis_cost_counters(), before);
}

#[test]
fn sibling_heads_sharing_a_commit_keep_exact_branch_snapshot_bindings() {
    let runtime = Arc::new(Mutex::new(runtime_with_test_schema()));
    let (storm_identity, maintenance_identity) = {
        let mut runtime = runtime.lock().unwrap();
        create_entity_outcome(&mut runtime, "shared-sibling-root");
        runtime
            .history_authority()
            .fork_branch_from(BranchId("storm".to_owned()), &BranchId("main".to_owned()))
            .unwrap();
        runtime
            .history_authority()
            .fork_branch_from(
                BranchId("maintenance".to_owned()),
                &BranchId("main".to_owned()),
            )
            .unwrap();
        (
            runtime
                .branch_identity(&BranchId("storm".to_owned()))
                .unwrap(),
            runtime
                .branch_identity(&BranchId("maintenance".to_owned()))
                .unwrap(),
        )
    };
    let source =
        RuntimeBridgeRelationalSource::for_shared_graph_role(Arc::clone(&runtime), "model")
            .unwrap();
    let (_, storm_basis) = source.observe_branch_basis(&storm_identity).unwrap();
    let (_, maintenance_basis) = source.observe_branch_basis(&maintenance_identity).unwrap();
    assert_eq!(
        storm_basis.observation().selected_root().commit_id(),
        maintenance_basis.observation().selected_root().commit_id(),
    );
    let before_bind = runtime.lock().unwrap().branch_basis_cost_counters();
    let storm_lease = source
        .bind_branch_head_basis_for_bridge(&storm_basis)
        .unwrap();
    let maintenance_lease = source
        .bind_branch_head_basis_for_bridge(&maintenance_basis)
        .unwrap();
    let storm_snapshot = storm_lease.snapshot_identity().clone();
    let maintenance_snapshot = maintenance_lease.snapshot_identity().clone();
    assert_ne!(storm_snapshot, maintenance_snapshot);

    let storm_branch = TruthBranchIdentity::from_relational_branch_id("storm");
    let maintenance_branch = TruthBranchIdentity::from_relational_branch_id("maintenance");
    let storm_envelope = source.load_branch_head_patch(&storm_branch).unwrap();
    let maintenance_envelope = source.load_branch_head_patch(&maintenance_branch).unwrap();
    assert_eq!(storm_envelope.snapshot_identity(), &storm_snapshot);
    assert_eq!(
        maintenance_envelope.snapshot_identity(),
        &maintenance_snapshot
    );
    let commit_id = storm_basis
        .observation()
        .selected_root()
        .commit_id()
        .unwrap();
    let collision = source
        .load_committed_patch(RelationalCommittedPatchRequest::new(
            TruthCommitIdentity::from_relational_commit_id(commit_id.0),
        ))
        .expect_err("a commit shared by two observations is ambiguous");
    assert!(collision
        .to_string()
        .contains("multiple admitted Bridge observations"));

    let storm_receipt = storm_lease.release();
    assert!(storm_receipt.unbound());
    assert_eq!(storm_receipt.branch_identity(), &storm_branch);
    assert!(source.load_branch_head_patch(&storm_branch).is_err());
    assert!(source.load_branch_head_patch(&maintenance_branch).is_ok());
    let sole_remaining = source
        .load_committed_patch(RelationalCommittedPatchRequest::new(
            TruthCommitIdentity::from_relational_commit_id(commit_id.0),
        ))
        .unwrap();
    assert_eq!(sole_remaining.snapshot_identity(), &maintenance_snapshot);
    let after_explicit = runtime.lock().unwrap().branch_basis_cost_counters();
    assert_eq!(
        after_explicit.external_retention_acquires,
        before_bind.external_retention_acquires + 2
    );
    assert_eq!(
        after_explicit.external_retention_releases,
        before_bind.external_retention_releases + 1
    );
    assert_eq!(
        after_explicit.external_retention_drop_releases,
        before_bind.external_retention_drop_releases
    );

    drop(maintenance_lease);
    assert!(source.load_branch_head_patch(&maintenance_branch).is_err());
    let after_drop = runtime.lock().unwrap().branch_basis_cost_counters();
    assert_eq!(
        after_drop.external_retention_releases,
        before_bind.external_retention_releases + 2
    );
    assert_eq!(
        after_drop.external_retention_drop_releases,
        before_bind.external_retention_drop_releases + 1
    );
}

#[test]
fn same_branch_rebinding_survives_delayed_old_release() {
    let runtime = Arc::new(Mutex::new(runtime_with_test_schema()));
    create_entity_outcome(&mut runtime.lock().unwrap(), "first-head");
    let source =
        RuntimeBridgeRelationalSource::for_shared_graph_role(Arc::clone(&runtime), "model")
            .unwrap();
    let identity = runtime.lock().unwrap().main_branch_identity();
    let (_, old_basis) = source.observe_branch_basis(&identity).unwrap();
    let old_lease = source
        .bind_branch_head_basis_for_bridge(&old_basis)
        .unwrap();
    let old_snapshot = old_lease.snapshot_identity().clone();

    create_entity_outcome(&mut runtime.lock().unwrap(), "second-head");
    let (_, new_basis) = source.observe_branch_basis(&identity).unwrap();
    let new_lease = source
        .bind_branch_head_basis_for_bridge(&new_basis)
        .unwrap();
    let new_snapshot = new_lease.snapshot_identity().clone();
    assert_ne!(old_snapshot, new_snapshot);

    let old_release = old_lease.release();
    assert!(!old_release.unbound());
    let current = source
        .load_branch_head_patch(&TruthBranchIdentity::from_relational_branch_id("main"))
        .unwrap();
    assert_eq!(current.snapshot_identity(), &new_snapshot);

    let final_release = new_lease.release();
    assert!(final_release.unbound());
    assert!(source
        .load_branch_head_patch(&TruthBranchIdentity::from_relational_branch_id("main"))
        .is_err());
}

#[test]
fn barrier_ordered_rebind_cannot_be_removed_by_the_old_lease() {
    let runtime = Arc::new(Mutex::new(runtime_with_test_schema()));
    create_entity_outcome(&mut runtime.lock().unwrap(), "race-first-head");
    let source =
        RuntimeBridgeRelationalSource::for_shared_graph_role(Arc::clone(&runtime), "model")
            .unwrap();
    let identity = runtime.lock().unwrap().main_branch_identity();
    let (_, old_basis) = source.observe_branch_basis(&identity).unwrap();
    let old_lease = source
        .bind_branch_head_basis_for_bridge(&old_basis)
        .unwrap();
    create_entity_outcome(&mut runtime.lock().unwrap(), "race-second-head");
    let (_, new_basis) = source.observe_branch_basis(&identity).unwrap();

    let start = Arc::new(Barrier::new(2));
    let rebound = Arc::new(Barrier::new(2));
    let bind_source = source.clone();
    let bind_start = Arc::clone(&start);
    let bind_rebound = Arc::clone(&rebound);
    let binder = std::thread::spawn(move || {
        bind_start.wait();
        let lease = bind_source
            .bind_branch_head_basis_for_bridge(&new_basis)
            .unwrap();
        bind_rebound.wait();
        lease
    });
    let release_start = Arc::clone(&start);
    let release_rebound = Arc::clone(&rebound);
    let releaser = std::thread::spawn(move || {
        release_start.wait();
        release_rebound.wait();
        old_lease.release()
    });

    let new_lease = binder.join().unwrap();
    let old_release = releaser.join().unwrap();
    assert!(!old_release.unbound());
    let current = source
        .load_branch_head_patch(&TruthBranchIdentity::from_relational_branch_id("main"))
        .unwrap();
    assert_eq!(current.snapshot_identity(), new_lease.snapshot_identity());
    assert!(new_lease.release().unbound());
}
