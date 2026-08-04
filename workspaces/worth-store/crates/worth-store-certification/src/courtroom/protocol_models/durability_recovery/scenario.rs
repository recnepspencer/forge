use worth_store_formal_models::{
    map_checkpoint_cutover, map_checkpoint_selection, map_failed_wal_fence,
    map_recovery_completion, map_redo_execution, map_redo_generation_denial,
    map_reopened_recovery_artifact, DurabilityRecoveryAction,
};
#[cfg(not(windows))]
use worth_store_physical_backend::PosixFileFsyncDirFsyncProfile;
#[cfg(windows)]
use worth_store_physical_backend::WindowsFlushFileBuffersProfile;
use worth_store_physical_backend::{
    BackendDurabilityProfile, SimulatedStrictDurableProfile, WalDurabilityBarrier,
};
use worth_store_physical_format::PhysicalPageId;
use worth_store_recovery_physics::{
    AdmittedRedoFrame, CheckpointBaseAdmission, CheckpointCutoverReceipt,
    CheckpointPublicationPlan, LogSequenceNumber, RecoveryRedoPlan, RedoRecordGrammar,
    RedoRecordIdempotenceBasis, RedoRecordIntegrityBinding, RedoRecordOperationForm,
    RedoRecordTargetGeneration, WalAppendFailureObservation, WalAppendObservationScope,
    WalAppendReceipt, WalDurabilityObservation, WalLsnRange, WalSegmentGeneration, WalSegmentId,
};
use worth_store_test_support::harness::{
    physical_residency::{canonical_physical_mutation_acknowledgment, PhysicalResidencyStoreWorld},
    recovery::{checkpoint_basis, checkpoint_durability, closeout, redo_replay, source_precedence},
};

use super::map_physical_mutation_acknowledgment;

#[cfg(not(windows))]
type HostDurabilityProfile = PosixFileFsyncDirFsyncProfile;
#[cfg(windows)]
type HostDurabilityProfile = WindowsFlushFileBuffersProfile;

pub(in crate::courtroom::protocol_models) const fn ordinary_durability_profile(
) -> worth_store_physical_backend::BackendDurabilityProfileId {
    HostDurabilityProfile::ID
}

pub(in crate::courtroom::protocol_models) fn execute_ordinary_durability_recovery(
) -> Vec<DurabilityRecoveryAction> {
    execute_ordinary_durability_recovery_traces()
        .into_iter()
        .flatten()
        .collect()
}

pub(in crate::courtroom::protocol_models) fn execute_ordinary_durability_recovery_traces(
) -> Vec<Vec<DurabilityRecoveryAction>> {
    let completed_mutation = execute_canonical_physical_mutation();
    let data_durable_prefix = completed_mutation[..6].to_vec();
    let checkpoint = execute_checkpoint();
    let stable_setup = data_durable_prefix
        .iter()
        .copied()
        .chain(checkpoint.iter().copied())
        .collect::<Vec<_>>();
    let redo = execute_redo_traces();
    let completion = map_recovery_completion(&closeout::recovery_completion());
    let reopen = map_reopened_recovery_artifact(
        &worth_store_test_support::harness::recovery::reopened_recovery_artifact_fixture(
            "protocol-durability-reopen",
        ),
    );

    let applied_recovery = stable_setup
        .iter()
        .copied()
        .chain(redo[0].iter().copied())
        .chain(completion)
        .collect();
    let crash_reopen = stable_setup
        .iter()
        .copied()
        .chain([DurabilityRecoveryAction::Crash, reopen])
        .collect();
    let skipped_recovery = stable_setup
        .iter()
        .copied()
        .chain(redo[1].iter().copied())
        .collect();
    let rejected_generation = stable_setup
        .into_iter()
        .chain(redo[2].iter().copied())
        .collect();
    vec![
        completed_mutation,
        applied_recovery,
        crash_reopen,
        skipped_recovery,
        rejected_generation,
    ]
}

pub(in crate::courtroom::protocol_models) fn replay_acknowledgment_ordering_guard(
    seed: u64,
) -> Vec<DurabilityRecoveryAction> {
    let scope = WalAppendObservationScope::new(
        WalSegmentId::new(seed.max(1)).unwrap(),
        WalSegmentGeneration::new(1).unwrap(),
        WalLsnRange::new(LogSequenceNumber::new(10), LogSequenceNumber::new(11)).unwrap(),
        format!("failed-wal-fence-{seed}"),
        64,
    )
    .unwrap();
    let receipt = WalAppendReceipt::<SimulatedStrictDurableProfile>::from_certification_observation(
        scope,
        64,
        SimulatedStrictDurableProfile::REQUIRED_BARRIERS,
        Some(WalAppendFailureObservation::BarrierFailed(
            WalDurabilityBarrier::SimulatedDurableCommit,
        )),
    );
    let denial = WalDurabilityObservation::from_append_receipt(receipt).unwrap_err();
    let actions = map_failed_wal_fence(&denial).unwrap().to_vec();
    assert!(!actions.contains(&DurabilityRecoveryAction::PhysicalMutationAcknowledged));
    actions
}

fn execute_canonical_physical_mutation() -> Vec<DurabilityRecoveryAction> {
    let world = PhysicalResidencyStoreWorld::initialize("protocol-physical-ack")
        .expect("canonical physical mutation world");
    let acknowledgment = canonical_physical_mutation_acknowledgment(
        &world,
        [0x7a; 32],
        b"protocol-physical-mutation",
    );
    let actions = map_physical_mutation_acknowledgment(&acknowledgment).to_vec();
    let _closed = world.close();
    actions
}

fn execute_checkpoint() -> Vec<DurabilityRecoveryAction> {
    let validation = checkpoint_durability::validate(checkpoint_basis::manifest(10, 20, 19));
    let durability = checkpoint_durability::checkpoint_durability_for_profile::<
        HostDurabilityProfile,
    >(&validation);
    let plan = CheckpointPublicationPlan::<HostDurabilityProfile>::plan_cutover(
        validation.clone(),
        durability,
    )
    .unwrap();
    let receipt = CheckpointCutoverReceipt::publish(plan);
    let selection = CheckpointBaseAdmission::from_validated_checkpoint(
        &validation,
        &receipt,
        source_precedence::trace("protocol-checkpoint", 1),
    )
    .unwrap();
    let mut actions = map_checkpoint_cutover(&receipt).unwrap().to_vec();
    actions.push(map_checkpoint_selection(&selection));
    actions
}

fn execute_redo_traces() -> [Vec<DurabilityRecoveryAction>; 3] {
    let source = redo_replay::checkpoint_plus_tail_source(20, 21);
    let prefix = redo_replay::valid_prefix(&source, 20, 21, [redo_replay::frame(20)]);
    let eligibility = redo_replay::redo_eligibility(19, 20);
    let grammar = redo_replay::grammar_for(&eligibility, 20, redo_replay::page_lsn(20)).unwrap();
    let admitted = AdmittedRedoFrame::admit(grammar, &prefix).unwrap();
    let plan = RecoveryRedoPlan::from_valid_prefix(&source, prefix, vec![admitted]).unwrap();
    let applied = plan
        .execute(&redo_replay::cursor(&eligibility, 19, "checkpoint-page"))
        .unwrap();
    let skipped = plan.execute(&applied.recovered_cursor().unwrap()).unwrap();
    [
        map_redo_execution(&applied),
        map_redo_execution(&skipped),
        vec![
            DurabilityRecoveryAction::RecoveryReplayRequired,
            execute_redo_generation_denial(),
        ],
    ]
}

fn execute_redo_generation_denial() -> DurabilityRecoveryAction {
    let source = redo_replay::checkpoint_plus_tail_source(20, 21);
    let prefix = redo_replay::valid_prefix(&source, 20, 21, [redo_replay::frame(20)]);
    let eligibility = redo_replay::redo_eligibility(19, 20);
    let wrong_target = RedoRecordGrammar::admit(
        Some(PhysicalPageId::from_raw(99).unwrap()),
        Some(RedoRecordTargetGeneration::new(
            eligibility.page_generation(),
        )),
        Some(redo_replay::lsn(20)),
        Some(RedoRecordOperationForm::declared_digest("op-20")),
        Some(RedoRecordIntegrityBinding::declared_digest("integrity-20")),
        Some(RedoRecordIdempotenceBasis::declared_digest("idem-20")),
        Some(redo_replay::page_lsn(20)),
    )
    .unwrap();
    let denial = AdmittedRedoFrame::admit(wrong_target, &prefix).unwrap_err();
    map_redo_generation_denial(&denial).unwrap()
}
