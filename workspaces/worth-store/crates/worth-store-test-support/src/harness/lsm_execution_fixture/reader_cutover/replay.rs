use super::{ReaderCutoverWorld, *};

pub(super) fn adjudicate_replay_sources(
    world: &ReaderCutoverWorld,
) -> worth_store_lsm_authority::AdmittedLsmReplaySource {
    let wal_replay_source = world
        .access
        .admit_replay_source(&world.plan, None, None)
        .expect("recovery owner admits the persisted WAL replay source");
    assert_eq!(
        wal_replay_source.selected_source(),
        LsmReplaySourceKind::WalFrame,
    );
    let replay = admit_replay(world, 43, &wal_replay_source)
        .expect("recovery-owned WAL source admits exact LSM replay");
    assert_eq!(
        admit_replay(world, 99, &wal_replay_source),
        Err(worth_store_layout_indexes::LsmMaintenanceAdmissionDenied::UnexpectedSelectedOperation),
        "a replay source cannot authorize a different canonical WAL operation",
    );
    reject_incomplete_checkpoints(world);
    let manifest = fully_covering_checkpoint(world);
    let checkpoint_replay_source = world
        .access
        .admit_replay_source(&world.plan, Some(&manifest), None)
        .expect("recovery owner admits the fully covering checkpoint source");
    assert_eq!(
        checkpoint_replay_source.selected_source(),
        LsmReplaySourceKind::Checkpoint,
    );
    let checkpoint_replay = admit_replay(world, 43, &checkpoint_replay_source)
        .expect("recovery-owned checkpoint source admits exact LSM replay");
    reject_unbound_and_ambiguous_sources(world, &manifest);
    verify_partial_publication_precedence(world, &manifest);
    execute_replay_lanes(replay, checkpoint_replay);
    wal_replay_source
}

fn admit_replay(
    world: &ReaderCutoverWorld,
    record_identity: u64,
    source: &worth_store_lsm_authority::AdmittedLsmReplaySource,
) -> Result<
    worth_store_layout_indexes::BaselineLsmReplayAdmission,
    worth_store_layout_indexes::LsmMaintenanceAdmissionDenied,
> {
    layout_lsm_maintenance()
        .admit_replay(LsmReplayAdmissionRequest::new(
            &world.catalog,
            world.security.witnesses(),
            WalRecordFamily::DurableMutationIntent,
            StoreWalRecordIdentity::new(record_identity),
            source,
            PreExecutionBudgetEnvelope::maintenance_default(),
        ))
        .into_result()
}

fn reject_incomplete_checkpoints(world: &ReaderCutoverWorld) {
    let partial_scope = world
        .plan
        .manifest_scope(StoreCheckpointRecordIdentity::new(2), 42, 45)
        .expect("hostile partial checkpoint scope");
    let partial = admit_checkpoint_publication(&manifest_receipt(partial_scope))
        .expect("hostile checkpoint remains durably published");
    assert_eq!(
        world
            .access
            .admit_replay_source(&world.plan, Some(&partial), None),
        Err(LsmReplaySourceDenial::CheckpointDoesNotCoverMembership),
    );
    let stale_scope = world
        .plan
        .manifest_scope(StoreCheckpointRecordIdentity::new(4), 40, 42)
        .expect("stale checkpoint scope remains well formed");
    let stale = admit_checkpoint_publication(&manifest_receipt(stale_scope)).unwrap();
    assert_eq!(
        world
            .access
            .admit_replay_source(&world.plan, Some(&stale), None)
            .unwrap()
            .selected_source(),
        LsmReplaySourceKind::WalFrame,
    );
}

fn fully_covering_checkpoint(
    world: &ReaderCutoverWorld,
) -> worth_store_wal::AdmittedCheckpointPublicationReceipt {
    let scope = world
        .plan
        .manifest_scope(StoreCheckpointRecordIdentity::new(1), 41, 45)
        .expect("manifest coverage");
    admit_checkpoint_publication(&manifest_receipt(scope)).expect("executed manifest durability")
}

fn reject_unbound_and_ambiguous_sources(
    world: &ReaderCutoverWorld,
    manifest: &worth_store_wal::AdmittedCheckpointPublicationReceipt,
) {
    let wrong_scope = CheckpointDurablePublicationScope::new(
        StoreCheckpointRecordIdentity::new(3),
        "copied-unrelated-lsm-manifest",
        41,
        45,
    )
    .unwrap();
    let wrong = admit_checkpoint_publication(&manifest_receipt(wrong_scope)).unwrap();
    assert_eq!(
        world
            .access
            .admit_replay_source(&world.plan, Some(&wrong), None),
        Err(LsmReplaySourceDenial::CheckpointDoesNotBindMembership),
    );
    let ambiguous = PartialPublicationClassification::classify(
        PartialPublicationEvidence::insufficient_persisted_evidence(
            "lsm-checkpoint-cutover-ambiguous",
        ),
    );
    assert_eq!(
        world
            .access
            .admit_replay_source(&world.plan, Some(manifest), Some(&ambiguous)),
        Err(LsmReplaySourceDenial::PartialPublicationAmbiguous),
    );
}

fn verify_partial_publication_precedence(
    world: &ReaderCutoverWorld,
    manifest: &worth_store_wal::AdmittedCheckpointPublicationReceipt,
) {
    let before_wal = PartialPublicationClassification::classify(
        PartialPublicationEvidence::from_persisted_crash_edge(
            PartialPublicationCrashEdge::before_wal_append("lsm-replacement"),
        ),
    );
    assert_eq!(
        world
            .access
            .admit_replay_source(&world.plan, Some(manifest), Some(&before_wal))
            .unwrap()
            .selected_source(),
        LsmReplaySourceKind::WalFrame,
    );
    let log_only = PartialPublicationClassification::classify(
        PartialPublicationEvidence::from_log_only("copied-lsm-log-claim"),
    );
    assert_eq!(
        world
            .access
            .admit_replay_source(&world.plan, Some(manifest), Some(&log_only))
            .unwrap()
            .selected_source(),
        LsmReplaySourceKind::WalFrame,
    );
    let torn = PartialPublicationClassification::classify(
        PartialPublicationEvidence::from_torn_publication(TornPublicationDenial::new(
            None,
            "torn LSM replacement publication",
        )),
    );
    assert_eq!(
        world
            .access
            .admit_replay_source(&world.plan, Some(manifest), Some(&torn)),
        Err(LsmReplaySourceDenial::TornPublication),
    );
}

fn execute_replay_lanes(
    wal: worth_store_layout_indexes::BaselineLsmReplayAdmission,
    checkpoint: worth_store_layout_indexes::BaselineLsmReplayAdmission,
) {
    let wal = lsm_replay_runtime()
        .execute(wal)
        .into_result()
        .expect("WAL-selected replay executes the durable tail");
    assert_eq!((wal.replayable_count(), wal.remaining_run_count()), (3, 3));
    assert_eq!(
        (
            wal.counters().wal_replays(),
            wal.counters().maintenance_reads()
        ),
        (3, 0)
    );
    let checkpoint = lsm_replay_runtime()
        .execute(checkpoint)
        .into_result()
        .expect("selected replay executes only from recovery-owned source");
    assert_eq!(
        (
            checkpoint.replayable_count(),
            checkpoint.stale_run_count(),
            checkpoint.cleanup_batch_count(),
            checkpoint.remaining_run_count(),
        ),
        (0, 3, 1, 1),
    );
    assert_eq!(
        (
            checkpoint.counters().wal_replays(),
            checkpoint.counters().maintenance_reads(),
        ),
        (0, 1),
    );
}
