#[path = "s4_wal_durability_ack/ack_assertions.rs"]
mod ack_assertions;
#[path = "s4_wal_durability_ack/adversarial_ack_denial_paths.rs"]
mod adversarial_ack_denial_paths;
#[path = "s4_wal_durability_ack/append_path_basis.rs"]
mod append_path_basis;
#[path = "s4_wal_durability_ack/certified_barrier_observations.rs"]
mod certified_barrier_observations;
#[path = "s4_wal_durability_ack/crash_assertions.rs"]
mod crash_assertions;
#[path = "s4_wal_durability_ack/durable_profile_paths.rs"]
mod durable_profile_paths;

use ack_assertions::{assert_ack_basis, assert_denial, assert_missing_barrier, assert_short_write};
use adversarial_ack_denial_paths::{
    adversarial_lost_flush_profile_receipt, adversarial_reordered_missing_fence_receipt,
    delayed_flush_receipt, directory_sync_failed_receipt, lost_flush_receipt,
    mismatched_barrier_scope_denial, missing_posix_directory_receipt, mmap_receipt,
    posix_non_required_barrier_denial, short_write_receipt,
};
use crash_assertions::assert_unacknowledged_replayable_posture;
use durable_profile_paths::{
    adversarial_reordered_completed_receipt, completed_posix_receipt,
    completed_posix_receipt_from_directory_then_file_path, completed_simulated_receipt,
    completed_windows_receipt,
};
use worth_store_physical_backend::{
    AdversarialLostFlushProfile, AdversarialReorderedFlushProfile, PosixFileFsyncDirFsyncProfile,
    SimulatedStrictDurableProfile, WalDurabilityBarrier, WindowsFlushFileBuffersProfile,
};
use worth_store_recovery_physics::{
    AcknowledgmentPrecondition, DurableAckReceipt, IllegalAcknowledgmentDenialKind,
    WalDurabilityCrashPosture, WalDurabilityCrashRecord, WalSegmentId,
};

#[test]
fn equivalent_durable_append_paths_under_same_profile_produce_same_ack_basis() {
    let first = DurableAckReceipt::acknowledge(
        AcknowledgmentPrecondition::from_append_receipt(completed_posix_receipt()).unwrap(),
    );
    let second = DurableAckReceipt::acknowledge(
        AcknowledgmentPrecondition::from_append_receipt(
            completed_posix_receipt_from_directory_then_file_path(),
        )
        .unwrap(),
    );

    assert_eq!(first.ack_basis(), second.ack_basis());
    assert_ack_basis::<PosixFileFsyncDirFsyncProfile>(
        &first,
        WalDurabilityBarrier::WalDirectoryFsync,
    );
}

#[test]
fn lost_flush_short_write_delayed_flush_and_directory_sync_failure_block_ack() {
    assert_denial(
        AcknowledgmentPrecondition::from_append_receipt(lost_flush_receipt()),
        IllegalAcknowledgmentDenialKind::LostFlush,
    );
    assert_short_write(AcknowledgmentPrecondition::from_append_receipt(
        short_write_receipt(),
    ));
    assert_denial(
        AcknowledgmentPrecondition::from_append_receipt(delayed_flush_receipt()),
        IllegalAcknowledgmentDenialKind::DelayedFlush,
    );
    assert_denial(
        AcknowledgmentPrecondition::from_append_receipt(directory_sync_failed_receipt()),
        IllegalAcknowledgmentDenialKind::DirectorySyncFailure,
    );
}

#[test]
fn unsupported_mmap_profile_and_missing_required_barriers_deny_ack() {
    assert_denial(
        AcknowledgmentPrecondition::from_append_receipt(mmap_receipt()),
        IllegalAcknowledgmentDenialKind::UnsupportedDurabilityCapability,
    );
    assert_missing_barrier(
        AcknowledgmentPrecondition::from_append_receipt(missing_posix_directory_receipt()),
        WalDurabilityBarrier::WalDirectoryFsync,
    );
}

#[test]
fn all_named_backend_profiles_either_certify_exact_barriers_or_deny_explicitly() {
    let simulated = DurableAckReceipt::acknowledge(
        AcknowledgmentPrecondition::from_append_receipt(completed_simulated_receipt()).unwrap(),
    );
    assert_ack_basis::<SimulatedStrictDurableProfile>(
        &simulated,
        WalDurabilityBarrier::SimulatedDurableCommit,
    );

    let posix = DurableAckReceipt::acknowledge(
        AcknowledgmentPrecondition::from_append_receipt(completed_posix_receipt()).unwrap(),
    );
    assert_ack_basis::<PosixFileFsyncDirFsyncProfile>(&posix, WalDurabilityBarrier::WalFileFsync);

    let windows = DurableAckReceipt::acknowledge(
        AcknowledgmentPrecondition::from_append_receipt(completed_windows_receipt()).unwrap(),
    );
    assert_ack_basis::<WindowsFlushFileBuffersProfile>(
        &windows,
        WalDurabilityBarrier::WindowsFlushFileBuffers,
    );

    assert_denial(
        AcknowledgmentPrecondition::from_append_receipt(mmap_receipt()),
        IllegalAcknowledgmentDenialKind::UnsupportedDurabilityCapability,
    );
    assert_denial(
        AcknowledgmentPrecondition::from_append_receipt(adversarial_lost_flush_profile_receipt()),
        IllegalAcknowledgmentDenialKind::LostFlush,
    );

    let reordered = DurableAckReceipt::acknowledge(
        AcknowledgmentPrecondition::from_append_receipt(adversarial_reordered_completed_receipt())
            .unwrap(),
    );
    assert_ack_basis::<AdversarialReorderedFlushProfile>(
        &reordered,
        WalDurabilityBarrier::OrderedPersistenceFence,
    );
    assert_denial(
        AcknowledgmentPrecondition::from_append_receipt(
            adversarial_reordered_missing_fence_receipt(),
        ),
        IllegalAcknowledgmentDenialKind::RequiredBarrierMissing,
    );
}

#[test]
fn profile_authority_denies_barriers_outside_its_exact_required_set() {
    let denial = posix_non_required_barrier_denial();

    assert_eq!(
        denial.kind(),
        worth_store_physical_backend::BackendDurabilityBarrierDenialKind::BarrierNotRequiredByProfile
    );
    assert_eq!(
        denial.profile_id(),
        worth_store_physical_backend::BackendDurabilityProfileId::PosixFileFsyncDirFsync
    );
    assert_eq!(
        denial.barrier(),
        WalDurabilityBarrier::WindowsFlushFileBuffers
    );
}

#[test]
fn profile_typed_receipts_cannot_cross_ack_authority_boundaries_at_runtime() {
    let lost = AcknowledgmentPrecondition::<AdversarialLostFlushProfile>::from_append_receipt(
        adversarial_lost_flush_profile_receipt(),
    );
    assert_denial(lost, IllegalAcknowledgmentDenialKind::LostFlush);

    let posix = DurableAckReceipt::acknowledge(
        AcknowledgmentPrecondition::from_append_receipt(completed_posix_receipt()).unwrap(),
    );
    assert_eq!(
        posix.profile_id(),
        worth_store_physical_backend::BackendDurabilityProfileId::PosixFileFsyncDirFsync
    );
}

#[test]
fn crash_after_wal_durability_before_ack_is_unacknowledged_replayable_posture() {
    let precondition =
        AcknowledgmentPrecondition::from_append_receipt(completed_posix_receipt()).unwrap();
    let materialized =
        WalDurabilityCrashRecord::from_unacknowledged_durable_precondition(precondition);
    let reopened = materialized.reopen_for_recovery();
    let posture =
        WalDurabilityCrashPosture::<PosixFileFsyncDirFsyncProfile>::from_reopened_durability_record(
            reopened,
        );

    assert_unacknowledged_replayable_posture(&posture);
}

#[test]
fn reordered_profile_requires_ordering_fence_before_ack() {
    assert_missing_barrier(
        AcknowledgmentPrecondition::from_append_receipt(
            adversarial_reordered_missing_fence_receipt(),
        ),
        WalDurabilityBarrier::OrderedPersistenceFence,
    );

    let acknowledged = DurableAckReceipt::acknowledge(
        AcknowledgmentPrecondition::from_append_receipt(adversarial_reordered_completed_receipt())
            .unwrap(),
    );
    assert_ack_basis::<AdversarialReorderedFlushProfile>(
        &acknowledged,
        WalDurabilityBarrier::OrderedPersistenceFence,
    );
}

#[test]
fn direct_lost_flush_event_blocks_ack_even_under_certified_posix_profile() {
    assert_denial(
        AcknowledgmentPrecondition::from_append_receipt(lost_flush_receipt()),
        IllegalAcknowledgmentDenialKind::LostFlush,
    );
}

#[test]
fn barrier_receipts_cannot_complete_a_different_append_scope() {
    let denial = mismatched_barrier_scope_denial();

    assert_eq!(
        denial.kind(),
        IllegalAcknowledgmentDenialKind::BarrierReceiptScopeMismatch
    );
    assert_eq!(
        denial.profile_id(),
        Some(worth_store_physical_backend::BackendDurabilityProfileId::PosixFileFsyncDirFsync)
    );
    assert_eq!(denial.segment_id(), Some(WalSegmentId::new(42).unwrap()));
    assert_eq!(denial.barrier(), Some(WalDurabilityBarrier::WalFileFsync));
}
