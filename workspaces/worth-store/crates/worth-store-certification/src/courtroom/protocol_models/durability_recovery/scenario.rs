use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use worth_store_formal_models::{
    map_certified_wal_durability_mechanism, map_checkpoint_cutover, map_checkpoint_selection,
    map_directory_sync_failure, map_recovery_completion, map_redo_execution,
    map_redo_generation_denial, map_reopened_recovery_artifact, DurabilityRecoveryAction,
};
#[cfg(not(windows))]
use worth_store_physical_backend::PosixFileFsyncDirFsyncProfile;
#[cfg(windows)]
use worth_store_physical_backend::WindowsFlushFileBuffersProfile;
use worth_store_physical_backend::{
    AdmittedBackendCapabilityWitness, BackendCapabilityAdmissionRequest,
    BackendCapabilityEvidenceBasis, BackendCapabilitySupportSet, BackendDurabilityProfile,
    BackendMediaAssumptionSet, BackendRebindTriggers, BackendTargetProfile,
    PhysicalBackendCapabilityAdmissionAuthority, ProductionStorageBoundarySeam,
    ScriptedStorageBoundaryControl, StorageBoundaryFault, StoreDurabilityAdmission,
    StoreDurabilityExecutionBoundary, StoreDurabilityRequirement, StoreDurabilityRuntime,
    WalDurabilityBarrier,
};
use worth_store_physical_format::PhysicalPageId;
use worth_store_recovery_physics::{
    certify_wal_durability_mechanism, AcknowledgmentPrecondition, AdmittedRedoFrame,
    CertifiedWalDurabilityMechanismObservation, CheckpointArtifactDurabilityCommitment,
    CheckpointBaseAdmission, CheckpointCutoverReceipt, CheckpointDurabilityEvidenceSet,
    CheckpointPublicationPlan, DurableAckReceipt, LogSequenceNumber, RecoveryRedoPlan,
    RedoRecordGrammar, RedoRecordIdempotenceBasis, RedoRecordIntegrityBinding,
    RedoRecordOperationForm, RedoRecordTargetGeneration, WalAppendPlan,
    WalDurabilityObservationSequence, WalLsnRange, WalSegmentGeneration, WalSegmentId,
};
use worth_store_test_support::harness::recovery::{
    checkpoint_basis, checkpoint_durability, closeout, redo_replay, source_precedence,
};

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
    let wal_directory = worth_store_test_support::TemporaryDirectory::create("protocol-wal")
        .expect("protocol WAL directory");
    let wal = map_certified_wal_durability_mechanism(&execute_certified_wal(wal_directory.path()));
    let checkpoint = execute_checkpoint();
    let directory_failure = execute_directory_sync_crash(61);
    let redo = execute_redo_traces();
    let completion = map_recovery_completion(&closeout::recovery_completion());
    let reopen = map_reopened_recovery_artifact(
        &worth_store_test_support::harness::recovery::reopened_recovery_artifact_fixture(
            "protocol-durability-reopen",
        ),
    );

    let stable_setup = wal
        .iter()
        .copied()
        .chain(checkpoint.iter().copied())
        .collect::<Vec<_>>();
    let applied_recovery = stable_setup
        .iter()
        .copied()
        .chain(redo[0].iter().copied())
        .chain(completion)
        .collect();
    let failed_directory_sync = wal
        .iter()
        .copied()
        .chain(checkpoint[..2].iter().copied())
        .chain(directory_failure)
        .chain([reopen])
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
        applied_recovery,
        failed_directory_sync,
        skipped_recovery,
        rejected_generation,
    ]
}

pub(in crate::courtroom::protocol_models) fn execute_certified_wal(
    root: &Path,
) -> CertifiedWalDurabilityMechanismObservation<HostDurabilityProfile> {
    let payload = b"checked-durability-frontier";
    let plan = wal_plan::<HostDurabilityProfile>(11, 40, 41, payload);
    let wal_root = unique_root(root, "wal");
    let planner = worth_store_wal::WalAppendPlanner::open(&wal_root, 11, 3)
        .expect("open ordinary WAL append planner");
    certify_wal_durability_mechanism(
        &planner,
        payload,
        plan,
        &admitted_backend(HostDurabilityProfile::TARGET),
    )
    .expect("real physical execution reaches legal acknowledgment")
}

pub(in crate::courtroom::protocol_models) fn replay_acknowledgment_ordering_guard(
    seed: u64,
) -> Vec<DurabilityRecoveryAction> {
    let actions = execute_directory_sync_crash(seed);
    assert!(!actions.contains(&DurabilityRecoveryAction::WalAcknowledgmentLegal));
    actions
}

fn execute_checkpoint() -> Vec<DurabilityRecoveryAction> {
    let validation = checkpoint_durability::validate(checkpoint_basis::manifest(10, 20, 19));
    let backend = admitted_backend(HostDurabilityProfile::TARGET);
    let directory = worth_store_test_support::TemporaryDirectory::create("protocol-checkpoint")
        .expect("protocol checkpoint directory");
    let root = unique_root(directory.path(), "checkpoint");
    let manifest = checkpoint_ack(
        &validation,
        CheckpointArtifactDurabilityCommitment::manifest(&validation),
        51,
        &root,
        &backend,
    );
    let root_ack = checkpoint_ack(
        &validation,
        CheckpointArtifactDurabilityCommitment::root(&validation),
        52,
        &root,
        &backend,
    );
    let frontier = checkpoint_ack(
        &validation,
        CheckpointArtifactDurabilityCommitment::page_lsn_frontier(&validation),
        53,
        &root,
        &backend,
    );
    let locator = checkpoint_ack(
        &validation,
        CheckpointArtifactDurabilityCommitment::locator(&validation),
        54,
        &root,
        &backend,
    );
    let durability = CheckpointDurabilityEvidenceSet::admit(
        &validation,
        &manifest,
        &root_ack,
        &frontier,
        &locator,
    )
    .unwrap();
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

fn checkpoint_ack(
    validation: &worth_store_recovery_physics::CheckpointValidation,
    commitment: CheckpointArtifactDurabilityCommitment,
    segment: u64,
    root: &Path,
    backend: &AdmittedBackendCapabilityWitness,
) -> DurableAckReceipt<HostDurabilityProfile> {
    let range = validation.manifest().covered_lsn_range().range();
    let payload = commitment.digest().as_bytes();
    let plan = WalAppendPlan::<HostDurabilityProfile>::new(
        WalSegmentId::new(segment).unwrap(),
        WalSegmentGeneration::new(1).unwrap(),
        range,
        commitment.digest(),
        payload.len() as u64,
    )
    .unwrap();
    let progress = plan.record_written_bytes(payload.len() as u64);
    let scope = progress.durability_scope();
    let requirement = StoreDurabilityRequirement::checkpoint_publication(
        HostDurabilityProfile::REQUIRED_BARRIERS,
    );
    let admission = StoreDurabilityAdmission::admit(requirement, backend).unwrap();
    let accepted = admission.submit_write(scope).backend_accepted();
    let execution = StoreDurabilityRuntime::new()
        .persist_and_execute(root, payload, &accepted)
        .unwrap();
    let file = execution
        .certify_completed_barrier::<HostDurabilityProfile>(host_file_barrier())
        .unwrap();
    let directory = execution
        .certify_completed_barrier::<HostDurabilityProfile>(host_directory_barrier())
        .unwrap();
    let append = WalDurabilityObservationSequence::new(progress)
        .completed(file)
        .unwrap()
        .completed(directory)
        .unwrap()
        .finish()
        .unwrap();
    DurableAckReceipt::acknowledge(AcknowledgmentPrecondition::from_append_receipt(append).unwrap())
}

fn execute_directory_sync_crash(seed: u64) -> Vec<DurabilityRecoveryAction> {
    let payload = b"directory-sync-crash";
    let plan = wal_plan::<HostDurabilityProfile>(61, 60, 61, payload);
    let control = ScriptedStorageBoundaryControl::inject(
        ProductionStorageBoundarySeam::DirectorySync,
        StorageBoundaryFault::AbortBeforeDurabilityBarrier,
    );
    let progress = plan.record_written_bytes(payload.len() as u64);
    let requirement = StoreDurabilityRequirement::checkpoint_publication(
        HostDurabilityProfile::REQUIRED_BARRIERS,
    );
    let backend = admitted_backend(HostDurabilityProfile::TARGET);
    let admission = StoreDurabilityAdmission::admit(requirement, &backend).unwrap();
    let accepted = admission
        .submit_write(progress.durability_scope())
        .backend_accepted();
    let directory = worth_store_test_support::TemporaryDirectory::create("protocol-sync-crash")
        .expect("protocol sync crash directory");
    let failure = StoreDurabilityRuntime::new()
        .persist_and_execute_to_with_control(
            &unique_root(directory.path(), &format!("directory-sync-crash-{seed}")),
            payload,
            &accepted,
            StoreDurabilityExecutionBoundary::Complete,
            &control,
        )
        .unwrap_err();
    map_directory_sync_failure(&failure, &control.trace())
        .unwrap()
        .to_vec()
}

#[cfg(not(windows))]
const fn host_file_barrier() -> WalDurabilityBarrier {
    WalDurabilityBarrier::WalFileFsync
}

#[cfg(windows)]
const fn host_file_barrier() -> WalDurabilityBarrier {
    WalDurabilityBarrier::WindowsFlushFileBuffers
}

#[cfg(not(windows))]
const fn host_directory_barrier() -> WalDurabilityBarrier {
    WalDurabilityBarrier::WalDirectoryFsync
}

#[cfg(windows)]
const fn host_directory_barrier() -> WalDurabilityBarrier {
    WalDurabilityBarrier::WindowsDirectorySync
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

fn wal_plan<P: worth_store_physical_backend::BackendDurabilityProfile>(
    segment: u64,
    start: u64,
    end: u64,
    payload: &[u8],
) -> WalAppendPlan<P> {
    WalAppendPlan::new(
        WalSegmentId::new(segment).unwrap(),
        WalSegmentGeneration::new(3).unwrap(),
        WalLsnRange::new(LogSequenceNumber::new(start), LogSequenceNumber::new(end)).unwrap(),
        format!("durability-frontier-frame-{segment}"),
        payload.len() as u64,
    )
    .unwrap()
}

pub(super) fn admitted_backend(profile: BackendTargetProfile) -> AdmittedBackendCapabilityWitness {
    PhysicalBackendCapabilityAdmissionAuthority::store_owned()
        .admit_backend_capability(BackendCapabilityAdmissionRequest::new(
            profile,
            BackendCapabilityEvidenceBasis::certified_backend_profile(),
            BackendCapabilitySupportSet::buffered_durable_only(),
            BackendMediaAssumptionSet::platform_file_defaults(),
            BackendRebindTriggers::kernel_filesystem_mount_firmware_and_backend(),
        ))
        .expect("certified backend fixture is admissible")
}

fn unique_root(parent: &Path, lane: &str) -> PathBuf {
    static EXECUTION: AtomicU64 = AtomicU64::new(0);
    parent.join(format!(
        "worth-store-protocol-{lane}-{}-{}",
        std::process::id(),
        EXECUTION.fetch_add(1, Ordering::Relaxed),
    ))
}
