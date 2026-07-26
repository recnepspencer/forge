use std::path::Path;

use worth_store::physical_runtime::certification::MediaFaultDirective;
use worth_store::physical_runtime::{
    FilesystemMediaAdmission, PhysicalRecordId, PhysicalRecordInitialization, PhysicalRecordOpen,
    PhysicalWorkFilesystemProfileEvidence, RecordAppendBatch,
};
use worth_store_physical_backend::{FilesystemAccessPosture, MediaOperationRole};

use super::arguments::C6PressureInvocation;

mod configuration;
mod dirty_pressure;
mod protocol;
mod read_pressure;

use configuration::C6PressureConfiguration;

pub(super) fn validate_configuration(path: &Path) -> Result<(), String> {
    C6PressureConfiguration::read(path).map(|_| ())
}

pub(super) fn run(invocation: C6PressureInvocation) -> Result<(), String> {
    let configuration = C6PressureConfiguration::read(&invocation.configuration)?;
    let oracle = configuration.read_oracle(&invocation.oracle)?;
    seed_empty_store(&invocation.root)?;
    let records = materialize_oversized_world(&invocation.root, &oracle, configuration)?;
    run_inheritance_siege(&invocation.root, &oracle, configuration, &records)
}

fn seed_empty_store(root: &Path) -> Result<(), String> {
    let (format, placement, access) = super::configuration::record_configuration();
    let media = super::admission::admit_media(root, None)?;
    let serving = super::admission::require_serving(
        media.initialize_record_store(PhysicalRecordInitialization::new(format, placement, access)),
        "C.6 empty Store initialization",
    )?;
    let shutdown = serving.close();
    if shutdown.residency().requires_inspection() {
        return Err("C.6 empty Store closed with residency inspection".to_owned());
    }
    Ok(())
}

fn materialize_oversized_world(
    root: &Path,
    oracle: &[u8],
    configuration: C6PressureConfiguration,
) -> Result<Box<[PhysicalRecordId]>, String> {
    let (format, placement, access) = super::configuration::record_configuration();
    let media = super::admission::admit_media(root, None)?;
    let serving = super::admission::require_serving(
        media.open_record_store(
            PhysicalRecordOpen::new(format, access).with_residency_policy(
                configuration
                    .policy(format)
                    .into_result()
                    .map_err(|denial| format!("C.6 residency policy was invalid: {denial:?}"))?,
            ),
        ),
        "C.6 oversized-world open",
    )?;
    let records = oracle.chunks_exact(configuration.record_bytes());
    if !records.remainder().is_empty() || records.len() != configuration.record_count() {
        return Err("C.6 oracle does not form the configured record set".to_owned());
    }
    let batch = RecordAppendBatch::try_from_iter(records)
        .map_err(|denial| format!("C.6 oversized batch denied: {denial:?}"))?;
    let published = serving
        .record_submission()
        .append_batch(batch, placement)
        .map_err(|failure| format!("C.6 oversized publication failed: {failure:?}"))?;
    let record_ids = published.record_ids().to_vec().into_boxed_slice();
    if record_ids.len() != configuration.record_count() {
        return Err("C.6 publication omitted configured records".to_owned());
    }
    let counters = serving.residency_observation().counters();
    if counters.peak_resident_bytes() > configuration.resident_bytes() || counters.evictions() == 0
    {
        return Err("C.6 world materialization escaped its residency budget".to_owned());
    }
    let shutdown = serving.close();
    if shutdown.residency().requires_inspection() {
        return Err("C.6 world materialization closed with residency inspection".to_owned());
    }
    Ok(record_ids)
}

fn run_inheritance_siege(
    root: &Path,
    oracle: &[u8],
    configuration: C6PressureConfiguration,
    records: &[PhysicalRecordId],
) -> Result<(), String> {
    let (schedule, gate) = dirty_pause_schedule()?;
    let (profile, request) = super::exact_write::bind()?;
    let (format, _, access) = super::configuration::record_configuration();
    let media = super::admission::admit_media(root, Some(schedule))?;
    let serving = super::admission::require_serving(
        media.open_record_store(
            PhysicalRecordOpen::new(format, access)
                .with_residency_policy(
                    configuration
                        .policy(format)
                        .into_result()
                        .map_err(|denial| format!("C.6 siege policy was invalid: {denial:?}"))?,
                )
                .with_physical_work_profile(profile),
        ),
        "C.6 inheritance-siege open",
    )?;
    let identity = serving.c6_physical_work_handoff().identity();
    let pins = read_pressure::prove_pins(&serving, configuration)?;
    let reads = read_pressure::prove_reads(&serving, records, oracle, configuration)?;
    let cancellation =
        read_pressure::prove_cancellation(&serving, records[records.len() - 1], configuration)?;
    let dirty = dirty_pressure::prove(&serving, request, gate)?;
    let world = protocol::C6WorldEvidence {
        identity,
        records: configuration.record_count(),
        payload_bytes: oracle.len() as u64,
        directory_bytes: directory_bytes(root)?,
    };
    if world.directory_bytes < configuration.resident_bytes().saturating_mul(8) {
        return Err("C.6 Store was not materially larger than residency".to_owned());
    }
    let media = serving
        .observer()
        .media_snapshot()
        .map_err(|error| format!("C.6 filesystem observation failed: {error:?}"))?;
    let filesystem = PhysicalWorkFilesystemProfileEvidence::from_backend(media.backend_profile())
        .map_err(|denial| format!("C.6 filesystem evidence denied: {denial:?}"))?;
    let close = serving.close();
    protocol::emit(protocol::C6PressureEvidence {
        configuration,
        world,
        reads,
        pins,
        cancellation,
        dirty,
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
                1,
                MediaFaultDirective::PauseBefore(gate.clone()),
            )
            .for_identified_operation_ordinal()])
        .map_err(|denial| format!("C.6 dirty pause schedule denied: {denial:?}"))?;
    Ok((schedule, gate))
}

fn directory_bytes(root: &Path) -> Result<u64, String> {
    let mut total = 0_u64;
    for entry in std::fs::read_dir(root)
        .map_err(|error| format!("cannot inspect C.6 Store directory: {error}"))?
    {
        let path = entry
            .map_err(|error| format!("cannot inspect C.6 Store entry: {error}"))?
            .path();
        total = total.saturating_add(if path.is_dir() {
            directory_bytes(&path)?
        } else {
            path.metadata()
                .map_err(|error| format!("cannot inspect C.6 artifact: {error}"))?
                .len()
        });
    }
    Ok(total)
}
