use std::io::Write;

use worth_store::physical_runtime::certification::{
    CertificationPhysicalExecutionCheckpoint, MediaFaultDirective,
};
use worth_store::physical_runtime::{
    FilesystemMediaAdmission, PhysicalManifestCapacityTransition,
    PhysicalMutationIdempotencyMaterial, PhysicalRecordInitialization, PhysicalRecordOpen,
    PhysicalWorkFilesystemProfileEvidence, RecordAppendBatch, ServingPhysicalRuntime,
};
use worth_store_physical_backend::{FilesystemAccessPosture, MediaOperationRole};

use super::arguments::{WriteInvocation, WriteScenario};

pub(super) fn run(invocation: WriteInvocation) -> Result<(), String> {
    let configuration =
        super::configuration::CourtroomConfiguration::read(&invocation.configuration)?;
    let payload = std::fs::read(&invocation.oracle)
        .map_err(|error| format!("cannot read courtroom oracle payload: {error}"))?;
    if payload.len() != configuration.payload_bytes() {
        return Err("oracle payload length does not match immutable configuration".to_owned());
    }
    match invocation.scenario {
        WriteScenario::SeedPriorTruth => seed_prior_truth(&invocation.root, &payload),
        WriteScenario::BeforeBackendDispatch => {
            crash_before_backend_dispatch(&invocation, &payload)
        }
        WriteScenario::AfterExactWriteBeforeSchedulerSettlement => {
            crash_after_exact_write(&invocation)
        }
        WriteScenario::DuringShortWrite => crash_during_short_write(&invocation, &payload),
        WriteScenario::DuringRootPublication => {
            crash_after_catalog_replacement(&invocation, &payload)
        }
    }
}

fn seed_prior_truth(root: &std::path::Path, payload: &[u8]) -> Result<(), String> {
    let (format, placement, access) = super::configuration::record_configuration();
    let media = super::admission::admit_media(root, None)?;
    let durability = super::admission::admit_durability(&media)?;
    let serving = super::admission::require_serving(
        media.initialize_record_store(PhysicalRecordInitialization::new(
            format, placement, access, durability,
        )),
        "record-store initialization",
    )?;
    let published = serving.certification_publish_single_durable_mutation(
        placement,
        PhysicalManifestCapacityTransition::PreserveCurrent,
        PhysicalMutationIdempotencyMaterial::new([0xC7; 32]),
        RecordAppendBatch::try_from_iter([payload])
            .map_err(|denial| format!("seed batch denied: {denial:?}"))?,
    );
    let media = serving
        .observer()
        .media_snapshot()
        .map_err(|error| format!("seed filesystem observation failed: {error:?}"))?;
    let filesystem = PhysicalWorkFilesystemProfileEvidence::from_backend(media.backend_profile())
        .map_err(|denial| format!("seed filesystem evidence denied: {denial:?}"))?;
    super::filesystem_profile::emit(&filesystem);
    println!(
        "C5_1_COURTROOM_SEEDED {} {} {}",
        std::process::id(),
        published.current_root().generation(),
        published
            .settled_members()
            .iter()
            .map(|member| member.persisted_records().len())
            .sum::<usize>(),
    );
    std::io::stdout()
        .flush()
        .map_err(|error| format!("seed marker failed: {error}"))?;
    serving.close();
    Ok(())
}

fn crash_before_backend_dispatch(
    invocation: &WriteInvocation,
    payload: &[u8],
) -> Result<(), String> {
    let serving = open(&invocation.root, None)?;
    let gate = serving.certification_pause_physical_execution_at(
        CertificationPhysicalExecutionCheckpoint::BeforeBackendDispatch,
    );
    super::checkpoint::watch_execution(invocation.scenario.label(), gate);
    append(&serving, payload)?;
    Err("write crossed its execution crash checkpoint".to_owned())
}

fn crash_after_exact_write(invocation: &WriteInvocation) -> Result<(), String> {
    let (profile, request) = super::exact_write::bind()?;
    let serving = open_with_profile(&invocation.root, None, Some(profile))?;
    let command = super::exact_write::prepare_command(&serving, request)?;
    let checkpoint =
        CertificationPhysicalExecutionCheckpoint::AfterExactWriteBeforeSchedulerSettlement;
    let gate = serving.certification_pause_physical_execution_at(checkpoint);
    super::checkpoint::watch_execution(invocation.scenario.label(), gate);
    serving
        .execute_physical_work(command)
        .map_err(|denial| format!("courtroom exact-write execution denied: {denial:?}"))?;
    Err("exact write crossed its execution crash checkpoint".to_owned())
}

fn crash_during_short_write(invocation: &WriteInvocation, payload: &[u8]) -> Result<(), String> {
    let admission =
        FilesystemMediaAdmission::production(FilesystemAccessPosture::CoordinatedServiceAccount);
    let authority = admission.fault_schedule_authority();
    let gate = authority.pause_gate();
    let schedule = authority
        .schedule(vec![authority
            .rule(
                MediaOperationRole::PositionedWrite,
                1,
                MediaFaultDirective::AllowPrefixThenPause {
                    bytes: 1,
                    gate: gate.clone(),
                },
            )
            .for_identified_operation_ordinal()])
        .map_err(|denial| format!("media fault schedule denied: {denial:?}"))?;
    let serving = open(&invocation.root, Some(schedule))?;
    super::checkpoint::watch_media(invocation.scenario.label(), gate);
    append(&serving, payload)?;
    Err("write crossed its media crash checkpoint".to_owned())
}

fn crash_after_catalog_replacement(
    invocation: &WriteInvocation,
    payload: &[u8],
) -> Result<(), String> {
    let serving = open(&invocation.root, None)?;
    let gate = serving.certification_pause_physical_execution_at(
        CertificationPhysicalExecutionCheckpoint::AfterCatalogReplacementBeforeSchedulerSettlement,
    );
    super::checkpoint::watch_execution(invocation.scenario.label(), gate);
    append(&serving, payload)?;
    Err("write crossed its catalog-replacement crash checkpoint".to_owned())
}

fn open(
    root: &std::path::Path,
    schedule: Option<worth_store_physical_backend::MediaFaultSchedule>,
) -> Result<ServingPhysicalRuntime, String> {
    open_with_profile(root, schedule, None)
}

fn open_with_profile(
    root: &std::path::Path,
    schedule: Option<worth_store_physical_backend::MediaFaultSchedule>,
    profile: Option<worth_store::physical_runtime::PhysicalWorkProfileDeclaration>,
) -> Result<ServingPhysicalRuntime, String> {
    let (format, _, access) = super::configuration::record_configuration();
    let media = super::admission::admit_media(root, schedule)?;
    let durability = super::admission::admit_durability(&media)?;
    let mut request = PhysicalRecordOpen::new(format, access, durability);
    if let Some(profile) = profile {
        request = request.with_physical_work_profile(profile);
    }
    super::admission::require_serving(media.open_record_store(request), "record-store open")
}

fn append(serving: &ServingPhysicalRuntime, payload: &[u8]) -> Result<(), String> {
    let (_, placement, _) = super::configuration::record_configuration();
    let batch = RecordAppendBatch::try_from_iter([payload])
        .map_err(|denial| format!("crash batch denied: {denial:?}"))?;
    let _ = serving.certification_publish_single_durable_mutation(
        placement,
        PhysicalManifestCapacityTransition::PreserveCurrent,
        PhysicalMutationIdempotencyMaterial::new([0xC8; 32]),
        batch,
    );
    Ok(())
}
