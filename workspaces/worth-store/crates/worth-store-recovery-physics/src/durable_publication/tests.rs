use worth_store_physical_backend::{
    BackendCapabilityAdmissionRequest, BackendCapabilityEvidenceBasis, BackendCapabilityKind,
    BackendCapabilitySupportPosture, BackendCapabilitySupportSet, BackendMediaAssumptionSet,
    BackendRebindTriggers, BackendTargetProfile, PhysicalBackendCapabilityAdmissionAuthority,
    StoreDurabilityAdmission, StoreDurabilityFileSyncKind, StoreDurabilityRequirement,
    StoreDurabilityState, WalDurabilityBarrier, WalDurabilityBarrierSet,
};
use worth_store_wal::{CheckpointDurablePublicationScope, StoreCheckpointRecordIdentity};

use worth_store_wal::WalFrameDurablePublicationScope;

use crate::{
    CheckpointCrashDurabilityPosture, DurabilityRecoveryReplaySource,
    DurabilityRecoverySourcePrecedence, DurabilityReplayKind, DurableCheckpointPublication,
    DurableManifestPublication, DurableWalPublication, StoreDurablePublicationDenialKind,
};

use super::test_support::{admitted, reach_boundary};

#[test]
fn wal_publication_requires_ordering_barrier_durable_receipt() {
    let requirement = StoreDurabilityRequirement::wal_ordering_barrier(
        WalDurabilityBarrierSet::of(WalDurabilityBarrier::WalFileFsync),
    );
    let accepted = admitted(requirement)
        .submit_write(
            WalFrameDurablePublicationScope::new(1, 1, 10, 12, "sha256:wal", 128).unwrap(),
        )
        .backend_accepted();
    let receipt = reach_boundary(
        accepted,
        StoreDurabilityFileSyncKind::Fdatasync,
        false,
        false,
        true,
    )
    .unwrap()
    .ordering_barrier_durable()
    .unwrap();

    let publication = DurableWalPublication::publish(receipt);

    assert_eq!(
        publication.required_state(),
        StoreDurabilityState::OrderingBarrierDurable
    );
    assert_eq!(publication.replay_identity().digest(), "sha256:wal");
    assert_eq!(publication.counters().ordering_barriers_completed(), 1);
}

#[test]
fn checkpoint_publication_requires_directory_sync_and_rename_ordering() {
    let requirement = StoreDurabilityRequirement::checkpoint_publication(
        WalDurabilityBarrierSet::of(WalDurabilityBarrier::WalFileFsync)
            .insert(WalDurabilityBarrier::WalDirectoryFsync),
    );
    let accepted = admitted(requirement)
        .submit_write(
            CheckpointDurablePublicationScope::new(
                StoreCheckpointRecordIdentity::new(4),
                "sha256:checkpoint",
                5,
                55,
            )
            .unwrap(),
        )
        .backend_accepted();
    let receipt = reach_boundary(
        accepted,
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
    .unwrap();

    let publication = DurableCheckpointPublication::publish(receipt).unwrap();

    assert_eq!(
        publication.scope().checkpoint(),
        StoreCheckpointRecordIdentity::new(4)
    );
    assert_eq!(publication.replay_identity().first_lsn(), 5);
    assert_eq!(publication.counters().directory_syncs_completed(), 1);
    assert_eq!(publication.counters().renames_completed(), 1);
}

#[test]
fn manifest_and_checkpoint_publication_kinds_do_not_alias() {
    let requirement = StoreDurabilityRequirement::checkpoint_publication(
        WalDurabilityBarrierSet::of(WalDurabilityBarrier::WalFileFsync)
            .insert(WalDurabilityBarrier::WalDirectoryFsync),
    );
    let accepted = admitted(requirement)
        .submit_write(
            CheckpointDurablePublicationScope::new(
                StoreCheckpointRecordIdentity::new(8),
                "sha256:manifest",
                1,
                2,
            )
            .unwrap(),
        )
        .backend_accepted();
    let receipt = reach_boundary(
        accepted,
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
    .unwrap();

    let denial = DurableManifestPublication::publish(receipt).unwrap_err();

    assert_eq!(
        denial.kind(),
        StoreDurablePublicationDenialKind::WrongPublicationKind
    );
}

#[test]
fn manifest_publication_requires_fsync_directory_sync_and_rename_ordering() {
    let requirement = StoreDurabilityRequirement::manifest_publication(
        WalDurabilityBarrierSet::of(WalDurabilityBarrier::WalFileFsync)
            .insert(WalDurabilityBarrier::WalDirectoryFsync),
    );
    let accepted = admitted(requirement)
        .submit_write(
            CheckpointDurablePublicationScope::new(
                StoreCheckpointRecordIdentity::new(12),
                "sha256:manifest-durable",
                7,
                77,
            )
            .unwrap(),
        )
        .backend_accepted();
    let receipt = reach_boundary(
        accepted,
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
    .unwrap();

    let publication = DurableManifestPublication::publish(receipt).unwrap();

    assert_eq!(
        publication.replay_identity().kind(),
        DurabilityReplayKind::Manifest
    );
    assert_eq!(
        publication.replay_identity().digest(),
        "sha256:manifest-durable"
    );
    assert_eq!(publication.replay_identity().first_lsn(), 7);
    assert_eq!(publication.replay_identity().last_lsn(), 77);
    assert_eq!(publication.counters().fsyncs_completed(), 1);
    assert_eq!(publication.counters().directory_syncs_completed(), 1);
    assert_eq!(publication.counters().renames_completed(), 1);
    assert_eq!(publication.counters().ordering_barriers_completed(), 1);
}

#[test]
fn unsupported_directory_sync_blocks_checkpoint_before_execution() {
    let support = BackendCapabilitySupportSet::buffered_durable_only().with_posture(
        BackendCapabilityKind::DirectorySync,
        BackendCapabilitySupportPosture::Unsupported,
    );
    let witness = PhysicalBackendCapabilityAdmissionAuthority::store_owned()
        .admit_backend_capability(BackendCapabilityAdmissionRequest::new(
            BackendTargetProfile::PosixFileFsyncDirSync,
            BackendCapabilityEvidenceBasis::certified_backend_profile(),
            support,
            BackendMediaAssumptionSet::platform_file_defaults(),
            BackendRebindTriggers::kernel_filesystem_mount_firmware_and_backend(),
        ))
        .unwrap();

    let denial = StoreDurabilityAdmission::admit(
        StoreDurabilityRequirement::checkpoint_publication(WalDurabilityBarrierSet::of(
            WalDurabilityBarrier::WalFileFsync,
        )),
        &witness,
    )
    .unwrap_err();

    assert_eq!(denial.state(), StoreDurabilityState::DurabilityUnsupported);
}

#[test]
fn crash_source_precedence_uses_wal_when_checkpoint_lacks_namespace_and_rename() {
    let wal_requirement = StoreDurabilityRequirement::wal_ordering_barrier(
        WalDurabilityBarrierSet::of(WalDurabilityBarrier::WalFileFsync),
    );
    let wal_accepted = admitted(wal_requirement)
        .submit_write(
            WalFrameDurablePublicationScope::new(2, 9, 40, 80, "sha256:wal-crash", 256).unwrap(),
        )
        .backend_accepted();
    let wal = reach_boundary(
        wal_accepted,
        StoreDurabilityFileSyncKind::Fdatasync,
        false,
        false,
        true,
    )
    .unwrap()
    .ordering_barrier_durable()
    .unwrap();
    let wal_publication = DurableWalPublication::publish(wal);

    let checkpoint_requirement = StoreDurabilityRequirement::checkpoint_publication(
        WalDurabilityBarrierSet::of(WalDurabilityBarrier::WalFileFsync)
            .insert(WalDurabilityBarrier::WalDirectoryFsync),
    );
    let checkpoint_accepted = admitted(checkpoint_requirement)
        .submit_write(
            CheckpointDurablePublicationScope::new(
                StoreCheckpointRecordIdentity::new(10),
                "sha256:checkpoint-incomplete",
                40,
                80,
            )
            .unwrap(),
        )
        .backend_accepted();
    let incomplete_checkpoint = reach_boundary(
        checkpoint_accepted,
        StoreDurabilityFileSyncKind::Fsync,
        false,
        false,
        false,
    )
    .unwrap();

    let decision = DurabilityRecoverySourcePrecedence::after_incomplete_checkpoint_namespace(
        &wal_publication,
        &incomplete_checkpoint,
    );

    assert_eq!(
        decision.selected_source(),
        DurabilityRecoveryReplaySource::WalFrame
    );
    assert_eq!(
        decision.checkpoint_posture(),
        CheckpointCrashDurabilityPosture::BoundaryReachedWithoutNamespaceOrRename
    );
    assert_eq!(decision.selected_identity().digest(), "sha256:wal-crash");
}

#[test]
fn crash_source_precedence_uses_fully_durable_checkpoint_when_covered() {
    let wal_requirement = StoreDurabilityRequirement::wal_ordering_barrier(
        WalDurabilityBarrierSet::of(WalDurabilityBarrier::WalFileFsync),
    );
    let wal_accepted = admitted(wal_requirement)
        .submit_write(
            WalFrameDurablePublicationScope::new(3, 1, 10, 50, "sha256:wal-covered", 256).unwrap(),
        )
        .backend_accepted();
    let wal = reach_boundary(
        wal_accepted,
        StoreDurabilityFileSyncKind::Fdatasync,
        false,
        false,
        true,
    )
    .unwrap()
    .ordering_barrier_durable()
    .unwrap();
    let wal_publication = DurableWalPublication::publish(wal);

    let checkpoint_requirement = StoreDurabilityRequirement::checkpoint_publication(
        WalDurabilityBarrierSet::of(WalDurabilityBarrier::WalFileFsync)
            .insert(WalDurabilityBarrier::WalDirectoryFsync),
    );
    let checkpoint_accepted = admitted(checkpoint_requirement)
        .submit_write(
            CheckpointDurablePublicationScope::new(
                StoreCheckpointRecordIdentity::new(11),
                "sha256:checkpoint-durable",
                1,
                50,
            )
            .unwrap(),
        )
        .backend_accepted();
    let checkpoint = reach_boundary(
        checkpoint_accepted,
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
    .unwrap();
    let checkpoint_publication = DurableCheckpointPublication::publish(checkpoint).unwrap();

    let decision = DurabilityRecoverySourcePrecedence::after_fully_durable_checkpoint(
        &wal_publication,
        &checkpoint_publication,
    );

    assert_eq!(
        decision.selected_source(),
        DurabilityRecoveryReplaySource::Checkpoint
    );
    assert_eq!(
        decision.checkpoint_posture(),
        CheckpointCrashDurabilityPosture::FullyDurable
    );
    assert_eq!(
        decision.selected_identity().digest(),
        "sha256:checkpoint-durable"
    );
}
