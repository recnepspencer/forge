use forge_store_budgets::PreExecutionBudgetEnvelope;
use forge_store_layout_indexes::{
    layout_btree_recovery, BTreeReplayDenied, BTreeReplayLocation, BTreeReplayPhysicalSource,
    BTreeReplayRequest, BaselineBTreeExecutionDenial,
};
use forge_store_physical_format::{
    PhysicalGeneration, PhysicalGenerationAuthority, PhysicalPageId, PhysicalRecordSlot,
    PhysicalReferenceAuthority, PhysicalSegmentId, PhysicalStoreIdentity,
};
use forge_store_security::{
    admitted_store_managed_root_security_scope_for_layout_partition_test,
    admitted_tenant_page_security_scope_for_layout_partition_test,
};
use forge_store_test_support::{
    admitted_layout_bootstrap_catalog, deterministic_btree_replay_world,
};

use forge_store_test_support::harness::recovery::redo_replay as redo_fixture;

#[test]
fn ordinary_recovery_facade_reopens_and_validates_btree_state() {
    let catalog = admitted_layout_bootstrap_catalog();
    let security = admitted_tenant_page_security_scope_for_layout_partition_test();
    let world = deterministic_btree_replay_world();

    let recovered = layout_btree_recovery()
        .replay(BTreeReplayRequest::new(
            &catalog,
            security.witnesses(),
            location(),
            PreExecutionBudgetEnvelope::maintenance_default(),
            physical_source(&world, world.root_reference()),
        ))
        .unwrap();

    assert!(recovered.replay_generation_monotonic());
    assert!(recovered.manifest_advanced());
    assert!(recovered.rebuild_source_authoritative());
    assert_eq!(recovered.rebuild_authority_records(), 4);
    assert_eq!(recovered.rebuild_output_records(), 4);
    let counters = recovered.exact_counters();
    assert_eq!(counters.wal_replays(), 1);
    assert_eq!(counters.maintenance_reads(), 3);
    assert_eq!(counters.page_touches(), 3);
    assert_eq!(counters.index_probes(), 6);
    assert_eq!(counters.key_comparisons(), 6);
    assert_eq!(counters.manifest_reads(), 1);
    assert_eq!(counters.bytes_read(), 12_288);
    assert_eq!(counters.read_amplification(), 3);
    assert!(recovered
        .recovery_source_digest()
        .starts_with("CheckpointPlusWalTail:"));
}

#[test]
fn copied_or_stale_root_reference_is_rejected_by_recovery_source_admission() {
    let catalog = admitted_layout_bootstrap_catalog();
    let security = admitted_tenant_page_security_scope_for_layout_partition_test();
    let world = deterministic_btree_replay_world();
    let stale_cell = PhysicalGenerationAuthority::for_canonical_physical_format()
        .slot_cell(
            PhysicalSegmentId::from_raw(7).unwrap(),
            PhysicalPageId::from_raw(9).unwrap(),
            PhysicalRecordSlot::from_raw(1).unwrap(),
        )
        .with_slot_generation(PhysicalGeneration::from_raw(999).unwrap());
    let stale = PhysicalReferenceAuthority::for_canonical_physical_format()
        .admit_page_slot(stale_cell)
        .reference();

    let outcome = layout_btree_recovery().replay(BTreeReplayRequest::new(
        &catalog,
        security.witnesses(),
        location(),
        PreExecutionBudgetEnvelope::maintenance_default(),
        physical_source(&world, stale),
    ));

    assert!(matches!(outcome, Err(BTreeReplayDenied::Execution(_))));
}

#[test]
fn store_internal_security_scope_cannot_select_tenant_btree_replay() {
    let catalog = admitted_layout_bootstrap_catalog();
    let security = admitted_store_managed_root_security_scope_for_layout_partition_test();
    let world = deterministic_btree_replay_world();

    let outcome = layout_btree_recovery().replay(BTreeReplayRequest::new(
        &catalog,
        security.witnesses(),
        location(),
        PreExecutionBudgetEnvelope::maintenance_default(),
        physical_source(&world, world.root_reference()),
    ));

    assert_eq!(outcome, Err(BTreeReplayDenied::SecurityScope));
}

#[test]
fn replay_artifact_from_another_store_instance_is_rejected() {
    let catalog = admitted_layout_bootstrap_catalog();
    let security = admitted_tenant_page_security_scope_for_layout_partition_test();
    let world = deterministic_btree_replay_world();
    let key = forge_foundational::aspects()
        .vocabulary()
        .key("store.physical.foreign_instance")
        .unwrap();
    let foreign = PhysicalStoreIdentity::from_aspect_identity(
        forge_store_aspect_native::StoreAspectIdentity::from_aspect_key(key),
    );

    let outcome = layout_btree_recovery().replay(BTreeReplayRequest::new(
        &catalog,
        security.witnesses(),
        location(),
        PreExecutionBudgetEnvelope::maintenance_default(),
        BTreeReplayPhysicalSource::new(
            world.readiness().clone(),
            world.root_reference(),
            world.replay_artifact().clone(),
            foreign,
            redo_fixture::checkpoint_plus_tail_source_for_root(20, 30, world.root_reference()),
        ),
    ));

    assert!(matches!(outcome, Err(BTreeReplayDenied::Execution(_))));
}

#[test]
fn physical_root_without_admitted_checkpoint_or_wal_source_is_rejected() {
    let catalog = admitted_layout_bootstrap_catalog();
    let security = admitted_tenant_page_security_scope_for_layout_partition_test();
    let world = deterministic_btree_replay_world();
    let no_source =
        forge_store_recovery_physics::RecoverySourcePrecedenceGraph::new("btree-no-source")
            .admit_sources();

    let outcome = layout_btree_recovery().replay(BTreeReplayRequest::new(
        &catalog,
        security.witnesses(),
        location(),
        PreExecutionBudgetEnvelope::maintenance_default(),
        BTreeReplayPhysicalSource::new(
            world.readiness().clone(),
            world.root_reference(),
            world.replay_artifact().clone(),
            world.replay_artifact().store_identity().clone(),
            no_source,
        ),
    ));

    assert!(matches!(outcome, Err(BTreeReplayDenied::Execution(_))));
}

#[test]
fn checkpoint_for_copied_root_generation_cannot_authorize_current_root() {
    let catalog = admitted_layout_bootstrap_catalog();
    let security = admitted_tenant_page_security_scope_for_layout_partition_test();
    let world = deterministic_btree_replay_world();
    let copied_generation = PhysicalGenerationAuthority::for_canonical_physical_format()
        .slot_cell(
            PhysicalSegmentId::from_raw(7).unwrap(),
            PhysicalPageId::from_raw(9).unwrap(),
            PhysicalRecordSlot::from_raw(1).unwrap(),
        )
        .with_slot_generation(PhysicalGeneration::from_raw(999).unwrap());
    let copied_root = PhysicalReferenceAuthority::for_canonical_physical_format()
        .admit_page_slot(copied_generation)
        .reference();

    let outcome = layout_btree_recovery().replay(BTreeReplayRequest::new(
        &catalog,
        security.witnesses(),
        location(),
        PreExecutionBudgetEnvelope::maintenance_default(),
        BTreeReplayPhysicalSource::new(
            world.readiness().clone(),
            world.root_reference(),
            world.replay_artifact().clone(),
            world.replay_artifact().store_identity().clone(),
            redo_fixture::checkpoint_plus_tail_source_for_root(20, 30, copied_root),
        ),
    ));

    assert!(matches!(
        outcome,
        Err(BTreeReplayDenied::Execution(
            BaselineBTreeExecutionDenial::Recovery(
                forge_store_recovery_physics::BTreeReplaySourceDenial::CheckpointRootMismatch { .. }
            )
        ))
    ));
}

#[test]
fn wal_only_source_cannot_borrow_an_unmaterialized_physical_root() {
    let catalog = admitted_layout_bootstrap_catalog();
    let security = admitted_tenant_page_security_scope_for_layout_partition_test();
    let world = deterministic_btree_replay_world();

    let outcome = layout_btree_recovery().replay(BTreeReplayRequest::new(
        &catalog,
        security.witnesses(),
        location(),
        PreExecutionBudgetEnvelope::maintenance_default(),
        BTreeReplayPhysicalSource::new(
            world.readiness().clone(),
            world.root_reference(),
            world.replay_artifact().clone(),
            world.replay_artifact().store_identity().clone(),
            redo_fixture::wal_only_source(20, 30),
        ),
    ));

    assert!(matches!(
        outcome,
        Err(BTreeReplayDenied::Execution(
            BaselineBTreeExecutionDenial::Recovery(
                forge_store_recovery_physics::BTreeReplaySourceDenial::WalOnlyRootNotMaterialized
            )
        ))
    ));
}

fn location() -> BTreeReplayLocation {
    BTreeReplayLocation::new(
        PhysicalSegmentId::from_raw(7).unwrap(),
        PhysicalPageId::from_raw(9).unwrap(),
    )
}

fn physical_source(
    world: &forge_store_test_support::DeterministicBTreeReplayWorld,
    root: forge_store_physical_format::PhysicalReference,
) -> BTreeReplayPhysicalSource {
    BTreeReplayPhysicalSource::new(
        world.readiness().clone(),
        root,
        world.replay_artifact().clone(),
        world.replay_artifact().store_identity().clone(),
        redo_fixture::checkpoint_plus_tail_source_for_root(20, 30, root),
    )
}
