use super::super::execution::StoreDurabilityExecutionObservation;
use crate::{
    BackendCapabilityEvidenceBasis, BackendCapabilitySupportSet, BackendMediaAssumptionSet,
    StoreDurabilityAdmission, StoreDurabilityDenialKind, StoreDurabilityFileSyncKind,
    StoreDurabilityRequirement, StoreDurabilityState, WalDurabilityBarrier,
    WalDurabilityBarrierSet,
};

use super::super::test_support::{execution_proof, scripted_execution_proof, witness};

#[test]
fn certified_profile_progresses_through_backend_neutral_states() {
    let witness = witness(
        BackendCapabilityEvidenceBasis::certified_backend_profile(),
        BackendCapabilitySupportSet::buffered_durable_only(),
        BackendMediaAssumptionSet::platform_file_defaults(),
    );
    let requirement = StoreDurabilityRequirement::checkpoint_publication(
        WalDurabilityBarrierSet::of(WalDurabilityBarrier::WalFileFsync)
            .insert(WalDurabilityBarrier::WalDirectoryFsync),
    );

    let admission = StoreDurabilityAdmission::admit(requirement, &witness).unwrap();
    let submitted = admission.submit_write("checkpoint-manifest");
    assert_eq!(submitted.state(), StoreDurabilityState::WriteSubmitted);

    let accepted = submitted.backend_accepted();
    assert_eq!(
        accepted.state(),
        StoreDurabilityState::WriteAcceptedByBackend
    );
    let proof = execution_proof(
        &accepted,
        StoreDurabilityFileSyncKind::Fsync,
        true,
        true,
        true,
    );
    let boundary = accepted.reach_durability_boundary(proof).unwrap();
    assert_eq!(
        boundary.state(),
        StoreDurabilityState::WriteReachedDurabilityBoundary
    );
    let namespace = boundary.parent_namespace_durable().unwrap();
    assert_eq!(
        namespace.state(),
        StoreDurabilityState::ParentNamespaceDurable
    );
    let renamed = namespace.rename_durable().unwrap();
    assert_eq!(renamed.state(), StoreDurabilityState::RenameDurable);
    let ordered = renamed.ordering_barrier_durable().unwrap();
    assert_eq!(
        ordered.state(),
        StoreDurabilityState::OrderingBarrierDurable
    );
    assert_eq!(ordered.counters().writes_submitted(), 1);
    assert_eq!(ordered.counters().writes_accepted(), 1);
    assert_eq!(ordered.counters().flushes_completed(), 1);
    assert_eq!(ordered.counters().fsyncs_completed(), 1);
    assert_eq!(ordered.counters().fdatasyncs_completed(), 0);
    assert_eq!(ordered.counters().directory_syncs_completed(), 1);
    assert_eq!(ordered.counters().renames_completed(), 1);
    assert_eq!(ordered.counters().ordering_barriers_completed(), 1);
}

#[test]
fn missing_required_barrier_denies_before_durable_progression() {
    let witness = witness(
        BackendCapabilityEvidenceBasis::certified_backend_profile(),
        BackendCapabilitySupportSet::buffered_durable_only(),
        BackendMediaAssumptionSet::platform_file_defaults(),
    );
    let requirement = StoreDurabilityRequirement::wal_ordering_barrier(
        WalDurabilityBarrierSet::of(WalDurabilityBarrier::WalFileFsync)
            .insert(WalDurabilityBarrier::WalDirectoryFsync),
    );
    let admission = StoreDurabilityAdmission::admit(requirement, &witness).unwrap();
    let accepted = admission.submit_write("wal-frame").backend_accepted();
    let proof = scripted_execution_proof(
        &accepted,
        StoreDurabilityExecutionObservation::new(
            WalDurabilityBarrierSet::of(WalDurabilityBarrier::WalFileFsync),
            StoreDurabilityFileSyncKind::Fdatasync,
        )
        .with_ordering_barrier_completed(),
    );

    let denial = accepted.reach_durability_boundary(proof).unwrap_err();

    assert_eq!(
        denial.kind(),
        StoreDurabilityDenialKind::MissingRequiredBarrier
    );
    assert_eq!(
        denial.missing_barrier(),
        Some(WalDurabilityBarrier::WalDirectoryFsync)
    );
}

#[test]
fn wal_progression_records_fdatasync_without_collapsing_into_fsync() {
    let witness = witness(
        BackendCapabilityEvidenceBasis::certified_backend_profile(),
        BackendCapabilitySupportSet::buffered_durable_only(),
        BackendMediaAssumptionSet::platform_file_defaults(),
    );
    let requirement = StoreDurabilityRequirement::wal_ordering_barrier(
        WalDurabilityBarrierSet::of(WalDurabilityBarrier::WalFileFsync),
    );
    let accepted = StoreDurabilityAdmission::admit(requirement, &witness)
        .unwrap()
        .submit_write("wal-frame")
        .backend_accepted();
    let proof = execution_proof(
        &accepted,
        StoreDurabilityFileSyncKind::Fdatasync,
        false,
        false,
        true,
    );
    let ordered = accepted
        .reach_durability_boundary(proof)
        .unwrap()
        .ordering_barrier_durable()
        .unwrap();

    assert_eq!(ordered.counters().flushes_completed(), 1);
    assert_eq!(ordered.counters().fdatasyncs_completed(), 1);
    assert_eq!(ordered.counters().fsyncs_completed(), 0);
}

#[test]
fn fdatasync_cannot_satisfy_checkpoint_fsync_requirement() {
    let witness = witness(
        BackendCapabilityEvidenceBasis::certified_backend_profile(),
        BackendCapabilitySupportSet::buffered_durable_only(),
        BackendMediaAssumptionSet::platform_file_defaults(),
    );
    let requirement = StoreDurabilityRequirement::checkpoint_publication(
        WalDurabilityBarrierSet::of(WalDurabilityBarrier::WalFileFsync)
            .insert(WalDurabilityBarrier::WalDirectoryFsync),
    );
    let accepted = StoreDurabilityAdmission::admit(requirement, &witness)
        .unwrap()
        .submit_write("checkpoint")
        .backend_accepted();
    let proof = execution_proof(
        &accepted,
        StoreDurabilityFileSyncKind::Fdatasync,
        true,
        true,
        true,
    );

    let denial = accepted.reach_durability_boundary(proof).unwrap_err();

    assert_eq!(
        denial.kind(),
        StoreDurabilityDenialKind::MissingMediaAssumption
    );
}

#[test]
fn checkpoint_ordering_denies_when_execution_did_not_complete_directory_sync_or_rename() {
    let witness = witness(
        BackendCapabilityEvidenceBasis::certified_backend_profile(),
        BackendCapabilitySupportSet::buffered_durable_only(),
        BackendMediaAssumptionSet::platform_file_defaults(),
    );
    let requirement = StoreDurabilityRequirement::checkpoint_publication(
        WalDurabilityBarrierSet::of(WalDurabilityBarrier::WalFileFsync)
            .insert(WalDurabilityBarrier::WalDirectoryFsync),
    );
    let accepted = StoreDurabilityAdmission::admit(requirement, &witness)
        .unwrap()
        .submit_write("checkpoint")
        .backend_accepted();
    let proof = execution_proof(
        &accepted,
        StoreDurabilityFileSyncKind::Fsync,
        false,
        false,
        true,
    );
    let boundary = accepted.reach_durability_boundary(proof).unwrap();

    let denial = boundary.parent_namespace_durable().unwrap_err();

    assert_eq!(denial.kind(), StoreDurabilityDenialKind::FailedSync);
    assert_eq!(denial.state(), StoreDurabilityState::Denied);
}

#[test]
fn parent_namespace_claim_requires_directory_sync_even_for_wal_requirements() {
    let witness = witness(
        BackendCapabilityEvidenceBasis::certified_backend_profile(),
        BackendCapabilitySupportSet::buffered_durable_only(),
        BackendMediaAssumptionSet::platform_file_defaults(),
    );
    let requirement = StoreDurabilityRequirement::wal_ordering_barrier(
        WalDurabilityBarrierSet::of(WalDurabilityBarrier::WalFileFsync),
    );
    let accepted = StoreDurabilityAdmission::admit(requirement, &witness)
        .unwrap()
        .submit_write("wal-frame")
        .backend_accepted();
    let proof = execution_proof(
        &accepted,
        StoreDurabilityFileSyncKind::Fdatasync,
        false,
        false,
        true,
    );
    let boundary = accepted.reach_durability_boundary(proof).unwrap();

    let denial = boundary.parent_namespace_durable().unwrap_err();

    assert_eq!(denial.kind(), StoreDurabilityDenialKind::FailedSync);
}

#[test]
fn rename_claim_requires_rename_completion_even_when_requirement_does_not_need_rename() {
    let witness = witness(
        BackendCapabilityEvidenceBasis::certified_backend_profile(),
        BackendCapabilitySupportSet::buffered_durable_only(),
        BackendMediaAssumptionSet::platform_file_defaults(),
    );
    let requirement = StoreDurabilityRequirement::wal_ordering_barrier(
        WalDurabilityBarrierSet::of(WalDurabilityBarrier::WalFileFsync),
    );
    let accepted = StoreDurabilityAdmission::admit(requirement, &witness)
        .unwrap()
        .submit_write("wal-frame")
        .backend_accepted();
    let proof = execution_proof(
        &accepted,
        StoreDurabilityFileSyncKind::Fdatasync,
        true,
        false,
        true,
    );
    let namespace = accepted
        .reach_durability_boundary(proof)
        .unwrap()
        .parent_namespace_durable()
        .unwrap();

    let denial = namespace.rename_durable().unwrap_err();

    assert_eq!(denial.kind(), StoreDurabilityDenialKind::FailedSync);
}
