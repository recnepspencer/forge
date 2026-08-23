use std::path::Path;

use worth_store::physical_runtime::certification::MediaFaultDirective;
use worth_store::physical_runtime::{
    FilesystemMediaAdmission, PhysicalRecordOpen, PhysicalWorkCapacity,
    PhysicalWorkFilesystemProfileEvidence,
};
use worth_store_physical_backend::{FilesystemAccessPosture, MediaOperationRole};

use super::{
    configuration::BoundedResidencyConfiguration,
    protocol::{self, BoundedResidencyWorldEvidence, PhysicalWorkCourtroomWorldIdentity},
};

mod record_inventory;

pub(super) fn run(
    invocation: super::super::arguments::BoundedResidencyServingInvocation,
) -> Result<(), String> {
    let configuration = BoundedResidencyConfiguration::read(&invocation.configuration)?;
    let schedule_plan = invocation.schedule;
    let process_allocation = super::super::process_allocation::ProcessAllocationEpoch::begin()?;
    let (schedule, gate) = dirty_pause_schedule()?;
    let capacity = PhysicalWorkCapacity::default()
        .with_terminal_evidence_capacity(configuration.causal_evidence_capacity()?)
        .ok_or_else(|| "bounded-residency causal evidence capacity was zero".to_owned())?;
    let profile = super::super::exact_write::profile()?.with_capacity(capacity);
    let (format, _, access) = super::super::configuration::record_configuration();
    let policy = configuration
        .serving_policy(format)
        .into_result()
        .map_err(|denial| format!("bounded-residency serving policy denied: {denial:?}"))?;
    let media = super::super::admission::admit_media(&invocation.root, Some(schedule))?;
    let durability = super::super::admission::admit_durability_with_checkpoint_memory(
        &media,
        configuration.checkpoint_memory_limit(),
    )?;
    let serving = super::super::admission::require_serving(
        media.open_record_store(
            PhysicalRecordOpen::new(format, access, durability)
                .with_residency_policy(policy)
                .with_physical_work_profile(profile),
        ),
        "bounded-residency serving open",
    )?;
    let mutation_gate = serving.pause_physical_mutation_at(
        worth_store::physical_runtime::production::PhysicalMutationCheckpoint::
            AfterWritebackAdmissionBeforeEffect,
    );
    let identity = PhysicalWorkCourtroomWorldIdentity::new(
        serving.store_identity(),
        serving.runtime_identity(),
        serving.residency_observation().store_generation(),
    );
    let work_reconciliation_window =
        super::work_reconciliation::PhysicalWorkReconciliationWindow::begin(&serving)?;
    let dirty = super::writeback_pressure::prove(&serving, configuration, gate, mutation_gate)?;
    let records = record_inventory::discover(&serving, configuration)?;
    let speculation =
        super::speculative_pressure::prove(&serving, &records, configuration, schedule_plan)?;
    let pins = super::read_pressure::prove_pins(&serving, &records, configuration)?;
    let duplicate_ordinal = configuration.first_extent_ordinal();
    let duplicate = super::read_pressure::prove_duplicate_fault(
        &serving,
        records[duplicate_ordinal],
        duplicate_ordinal,
        configuration,
        schedule_plan,
    )?;
    let executed_schedule = super::schedule::BoundedResidencyExecutedSchedule::from_proofs(
        speculation.schedule,
        duplicate.schedule,
    );
    let reads = super::read_pressure::prove_reads(&serving, &records, configuration)?;
    let pending_cancellation = super::cancellation::exercise(&serving)?;
    let generation_fencing = super::generation_fencing::prove(&serving)?;
    let allocation = super::allocation_pressure::prove(&serving, configuration)?;
    super::checkpoint::complete_reliability_seed(&serving)?;
    let work_reconciliation_basis = work_reconciliation_window.finish(&serving)?;
    let work_observer = serving.physical_work_observer();
    let media = serving
        .observer()
        .media_snapshot()
        .map_err(|error| format!("bounded-residency filesystem observation failed: {error:?}"))?;
    let filesystem = PhysicalWorkFilesystemProfileEvidence::from_backend(media.backend_profile())
        .map_err(|denial| {
        format!("bounded-residency filesystem evidence denied: {denial:?}")
    })?;
    let close = serving.close();
    let process_allocation = process_allocation.finish();
    let cancellation = pending_cancellation.finalize(close.work().drain())?;
    let work_reconciliation =
        super::work_reconciliation::reconcile(work_reconciliation_basis, &work_observer, &close)?;
    let world = BoundedResidencyWorldEvidence {
        identity,
        records: configuration.record_count(),
        payload_bytes: configuration.payload_bytes()?,
        directory_bytes: directory_bytes(&invocation.root)?,
    };
    if world.payload_bytes < configuration.resident_bytes().saturating_mul(32)
        || world.payload_bytes < configuration.total_bytes().saturating_mul(16)
    {
        return Err("bounded-residency serving world failed its hostile ratios".to_owned());
    }
    protocol::emit(protocol::BoundedResidencyEvidence {
        configuration,
        schedule: executed_schedule,
        world,
        process_allocation,
        reads,
        pins,
        duplicate: duplicate.evidence,
        cancellation,
        generation_fencing,
        dirty,
        speculation: speculation.evidence,
        work_reconciliation,
        allocation,
        filesystem,
        close,
    })
}

fn dirty_pause_schedule() -> Result<
    (
        worth_store_physical_backend::MediaFaultSchedule,
        worth_store_physical_backend::MediaPauseGate,
    ),
    String,
> {
    let admission =
        FilesystemMediaAdmission::production(FilesystemAccessPosture::CoordinatedServiceAccount);
    let authority = admission.fault_schedule_authority();
    let gate = authority.pause_gate();
    let schedule = authority
        .schedule(vec![authority
            .rule(
                MediaOperationRole::PositionedWrite,
                super::writeback_pressure::CANDIDATE_WRITEBACK_POSITIONED_WRITE_ORDINAL,
                // Hold the primary writeback claim after its backend effect
                // completes. A pre-effect pause would also hold the backend
                // interposer while the competing mutation is trying to reach
                // its independent residency denial.
                MediaFaultDirective::PauseAfter(gate.clone()),
            )
            .for_identified_operation_ordinal()])
        .map_err(|denial| format!("bounded-residency dirty pause schedule denied: {denial:?}"))?;
    Ok((schedule, gate))
}

fn directory_bytes(root: &Path) -> Result<u64, String> {
    let mut total = 0_u64;
    for entry in std::fs::read_dir(root)
        .map_err(|error| format!("cannot inspect bounded-residency Store directory: {error}"))?
    {
        let path = entry
            .map_err(|error| format!("cannot inspect bounded-residency Store entry: {error}"))?
            .path();
        total = total.saturating_add(if path.is_dir() {
            directory_bytes(&path)?
        } else {
            path.metadata()
                .map_err(|error| format!("cannot inspect bounded-residency artifact: {error}"))?
                .len()
        });
    }
    Ok(total)
}
