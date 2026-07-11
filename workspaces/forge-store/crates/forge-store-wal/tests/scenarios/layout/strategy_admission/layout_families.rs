use forge_store_physical_backend::{
    BackendCapabilityAdmissionRequest, BackendCapabilityEvidenceBasis, BackendCapabilitySupportSet,
    BackendMediaAssumptionSet, BackendRebindTriggers, BackendTargetProfile,
    PhysicalBackendCapabilityAdmissionAuthority, StoreDurabilityAdmission,
    StoreDurabilityBoundaryReached, StoreDurabilityDenial, StoreDurabilityFileSyncKind,
    StoreDurabilityPublicationKind, StoreDurabilityRequirement, StoreDurabilityRuntime,
    StoreDurabilityWriteAccepted, WalDurabilityBarrier, WalDurabilityBarrierSet,
};
use forge_store_wal::{
    admit_checkpoint_cutover, admit_checkpoint_publication, admit_durable_append,
    admit_replay_cursor, inspect_replay_tail_record, BlobWalRecordEnvelope, BlobWalRecordIdentity,
    BlobWalRecordKind, CheckpointDurablePublicationScope, DurablePublicationDeclaration,
    LogSequenceNumber, StoreCheckpointRecordIdentity, WalFrameDurablePublicationScope, WalLsnRange,
    WalOperationDenialKind, WalSegmentGeneration, WalSegmentId, WalSegmentScanRecord,
    WalTopologyScan,
};

#[test]
fn wal_append_tail_and_checkpoint_rules_drive_public_layout_access() {
    let append_scope = WalFrameDurablePublicationScope::new(2, 9, 40, 80, "sha256:wal-crash", 256)
        .expect("append scope");
    let append_receipt = wal_receipt(append_scope.clone());
    let append_receipt = admit_durable_append(&append_receipt).expect("append receipt admission");
    let append_report = append_receipt.report();
    assert_eq!(append_report.scope(), &append_scope);
    assert_eq!(append_report.byte_count(), 256);
    assert_eq!(append_report.range_span(), 40);
    assert_eq!(append_report.counters().ordering_barriers_completed(), 1);

    let replay_record = BlobWalRecordEnvelope::new(
        BlobWalRecordIdentity::new(7, BlobWalRecordKind::GenerationPublication).unwrap(),
        DurablePublicationDeclaration::wal_frame(append_scope.clone()),
        "sha256:payload",
    )
    .expect("replayable wal record");
    let replay_cursor = admit_replay_cursor(
        WalTopologyScan::from_segment_scan([WalSegmentScanRecord::current(
            WalSegmentId::new(append_scope.segment_id()).unwrap(),
            WalSegmentGeneration::new(append_scope.generation()).unwrap(),
            WalLsnRange::new(
                LogSequenceNumber::new(append_scope.lsn_start()),
                LogSequenceNumber::new(append_scope.lsn_end()),
            )
            .unwrap(),
        )]),
        WalSegmentGeneration::new(append_scope.generation()).unwrap(),
    )
    .expect("replay cursor admission");
    let cursor_report = replay_cursor.report();
    assert_eq!(cursor_report.first_lsn(), append_scope.lsn_start());
    assert_eq!(cursor_report.end_lsn(), append_scope.lsn_end());
    assert_eq!(cursor_report.segment_count(), 1);
    let tail_report =
        inspect_replay_tail_record(&replay_cursor, &replay_record).expect("tail report");
    assert_eq!(tail_report.identity(), replay_record.identity());
    assert_eq!(tail_report.segment_id(), append_scope.segment_id());

    let checkpoint_scope = CheckpointDurablePublicationScope::new(
        StoreCheckpointRecordIdentity::new(4),
        "sha256:checkpoint",
        5,
        55,
    )
    .expect("checkpoint scope");
    let admitted_checkpoint_receipt = admit_checkpoint_cutover(&checkpoint_receipt(
        checkpoint_scope.clone(),
        StoreDurabilityRequirement::checkpoint_publication(
            WalDurabilityBarrierSet::of(WalDurabilityBarrier::WalFileFsync)
                .insert(WalDurabilityBarrier::WalDirectoryFsync),
        ),
    ))
    .expect("checkpoint receipt");
    let checkpoint_report = admitted_checkpoint_receipt.report();
    assert_eq!(checkpoint_report.scope(), &checkpoint_scope);
    assert_eq!(
        checkpoint_report.publication(),
        StoreDurabilityPublicationKind::Checkpoint
    );
    assert_eq!(checkpoint_report.range_span(), 50);
    assert_eq!(checkpoint_report.counters().directory_syncs_completed(), 1);

    let manifest_receipt = admit_checkpoint_publication(&checkpoint_receipt(
        checkpoint_scope,
        StoreDurabilityRequirement::manifest_publication(
            WalDurabilityBarrierSet::of(WalDurabilityBarrier::WalFileFsync)
                .insert(WalDurabilityBarrier::WalDirectoryFsync),
        ),
    ))
    .expect("manifest receipt");
    let manifest_report = manifest_receipt.report();
    assert_eq!(
        manifest_report.publication(),
        StoreDurabilityPublicationKind::Manifest
    );
    assert_eq!(manifest_report.counters().renames_completed(), 1);
}

#[test]
fn replay_tail_rejects_non_replay_record_kinds() {
    let record = BlobWalRecordEnvelope::new(
        BlobWalRecordIdentity::new(11, BlobWalRecordKind::ChunkAppend).unwrap(),
        DurablePublicationDeclaration::wal_frame(
            WalFrameDurablePublicationScope::new(1, 1, 10, 20, "sha256:chunk", 64).unwrap(),
        ),
        "sha256:payload",
    )
    .expect("non-replayable record");

    let denial = inspect_replay_tail_record(
        &admit_replay_cursor(
            WalTopologyScan::from_segment_scan([WalSegmentScanRecord::current(
                WalSegmentId::new(1).unwrap(),
                WalSegmentGeneration::new(1).unwrap(),
                WalLsnRange::new(LogSequenceNumber::new(10), LogSequenceNumber::new(20)).unwrap(),
            )]),
            WalSegmentGeneration::new(1).unwrap(),
        )
        .expect("cursor"),
        &record,
    )
    .expect_err("chunk append must not stand in for replay tail");

    assert_eq!(denial.kind(), WalOperationDenialKind::NonReplayTailRecord);
}

fn wal_receipt(
    scope: WalFrameDurablePublicationScope,
) -> forge_store_physical_backend::StoreDurabilityOrderingBarrierDurable<
    WalFrameDurablePublicationScope,
> {
    let requirement = StoreDurabilityRequirement::wal_ordering_barrier(
        WalDurabilityBarrierSet::of(WalDurabilityBarrier::WalFileFsync),
    );
    reach_boundary(
        admitted(requirement).submit_write(scope).backend_accepted(),
        StoreDurabilityFileSyncKind::Fdatasync,
        false,
        false,
        true,
    )
    .unwrap()
    .ordering_barrier_durable()
    .unwrap()
}

fn checkpoint_receipt(
    scope: CheckpointDurablePublicationScope,
    requirement: StoreDurabilityRequirement,
) -> forge_store_physical_backend::StoreDurabilityOrderingBarrierDurable<
    CheckpointDurablePublicationScope,
> {
    reach_boundary(
        admitted(requirement).submit_write(scope).backend_accepted(),
        StoreDurabilityFileSyncKind::Fsync,
        true,
        true,
        true,
    )
    .unwrap()
    .parent_namespace_durable()
    .unwrap()
    .rename_durable()
    .unwrap()
    .ordering_barrier_durable()
    .unwrap()
}

fn admitted(requirement: StoreDurabilityRequirement) -> StoreDurabilityAdmission {
    let witness = PhysicalBackendCapabilityAdmissionAuthority::store_owned()
        .admit_backend_capability(BackendCapabilityAdmissionRequest::new(
            BackendTargetProfile::PosixFileFsyncDirSync,
            BackendCapabilityEvidenceBasis::certified_backend_profile(),
            BackendCapabilitySupportSet::buffered_durable_only(),
            BackendMediaAssumptionSet::platform_file_defaults(),
            BackendRebindTriggers::kernel_filesystem_mount_firmware_and_backend(),
        ))
        .unwrap();
    StoreDurabilityAdmission::admit(requirement, &witness).unwrap()
}

fn reach_boundary<S>(
    accepted: StoreDurabilityWriteAccepted<S>,
    sync: StoreDurabilityFileSyncKind,
    directory_sync_completed: bool,
    rename_completed: bool,
    ordering_barrier_completed: bool,
) -> Result<StoreDurabilityBoundaryReached<S>, StoreDurabilityDenial>
where
    S: Clone + Eq + core::fmt::Debug,
{
    assert_eq!(accepted.requirement().required_file_sync(), sync);
    assert_eq!(
        accepted.requirement().requires_directory_sync(),
        directory_sync_completed
    );
    assert_eq!(
        accepted.requirement().requires_rename_durable(),
        rename_completed
    );
    assert_eq!(
        accepted.requirement().requires_ordering_barrier(),
        ordering_barrier_completed
    );
    let proof = StoreDurabilityRuntime::new()
        .persist_and_execute(
            &std::env::temp_dir(),
            b"wal-layout-durable-write",
            &accepted,
        )
        .unwrap();
    accepted.reach_durability_boundary(proof)
}
