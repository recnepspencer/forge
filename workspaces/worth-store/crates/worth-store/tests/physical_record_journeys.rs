use std::path::Path;

#[path = "physical_record_journeys/allocation_probe.rs"]
mod allocation_probe;
#[path = "physical_record_journeys/baseline_admission.rs"]
mod baseline_admission;
#[path = "physical_record_journeys/batch_admission.rs"]
mod batch_admission;
#[path = "physical_record_journeys/bootstrap_faults.rs"]
mod bootstrap_faults;
#[path = "physical_record_journeys/child_process.rs"]
mod child_process;
#[path = "physical_record_journeys/close_phase_crash.rs"]
mod close_phase_crash;
#[path = "physical_record_journeys/configuration_boundaries.rs"]
mod configuration_boundaries;
#[path = "physical_record_journeys/courtroom_child.rs"]
mod courtroom_child;
#[path = "c5/courtroom_evidence_support.rs"]
mod courtroom_evidence_support;
#[path = "c5/courtroom_oracle.rs"]
mod courtroom_oracle;
#[path = "c5/extent_child.rs"]
mod extent_child;
#[path = "physical_record_journeys/extent_streaming.rs"]
mod extent_streaming;
#[path = "physical_record_journeys/format_readmission.rs"]
mod format_readmission;
#[path = "physical_record_journeys/foundational_evidence.rs"]
mod foundational_evidence;
#[path = "physical_record_journeys/generation_policy_truth.rs"]
mod generation_policy_truth;
#[path = "physical_record_journeys/identity_process.rs"]
mod identity_process;
#[path = "physical_record_journeys/locator_free_space.rs"]
mod locator_free_space;
#[path = "physical_record_journeys/manifest_fixture.rs"]
mod manifest_fixture;
#[path = "physical_record_journeys/manifest_scale.rs"]
mod manifest_scale;
#[path = "c5/observer.rs"]
mod observer;
#[path = "physical_record_journeys/ordinary_writeback_failures.rs"]
mod ordinary_writeback_failures;
#[path = "physical_record_journeys/page_packing_oracle.rs"]
mod page_packing_oracle;
#[path = "physical_record_journeys/physical_work/mod.rs"]
mod physical_work;
#[path = "c5/courtrooms.rs"]
mod production_courtrooms;
#[path = "physical_record_journeys/publication_failure_topology.rs"]
mod publication_failure_topology;
#[path = "physical_record_journeys/publication_faults.rs"]
mod publication_faults;
#[path = "physical_record_journeys/publication_mutants.rs"]
mod publication_mutants;
#[path = "physical_record_journeys/publication_recovery_faults.rs"]
mod publication_recovery_faults;
#[path = "physical_record_journeys/publication_reopener.rs"]
mod publication_reopener;
#[path = "physical_record_journeys/read_boundaries.rs"]
mod read_boundaries;
#[path = "physical_record_journeys/record_chunk_views.rs"]
mod record_chunk_views;
#[path = "physical_record_journeys/residency_pressure_processes.rs"]
mod residency_pressure_processes;
#[path = "physical_record_journeys/residency_writeback_fresh_reopen.rs"]
mod residency_writeback_fresh_reopen;
#[path = "physical_record_journeys/residue_safety.rs"]
mod residue_safety;
#[path = "physical_record_journeys/reusable_segment_residue.rs"]
mod reusable_segment_residue;
#[path = "c5/scale_invalid_worlds.rs"]
mod scale_invalid_worlds;
#[path = "c5/scale_policy_evolution.rs"]
mod scale_policy_evolution;
#[path = "c5/scale.rs"]
mod scale_support;
#[path = "physical_record_journeys/scan_journeys.rs"]
mod scan_journeys;
#[path = "physical_record_journeys/scenario_artifact_evidence.rs"]
mod scenario_artifact_evidence;
#[path = "physical_record_journeys/scenario_configuration.rs"]
mod scenario_configuration;
#[path = "physical_record_journeys/scenario_evidence.rs"]
mod scenario_evidence;
#[path = "physical_record_journeys/scenario_process_evidence.rs"]
mod scenario_process_evidence;
#[path = "physical_record_journeys/segment_journeys.rs"]
mod segment_journeys;
#[path = "physical_record_journeys/segment_truth.rs"]
mod segment_truth;
#[path = "physical_record_journeys/serving_lifecycle.rs"]
mod serving_lifecycle;
#[path = "physical_record_journeys/stream_fixture.rs"]
mod stream_fixture;
#[path = "physical_record_journeys/successor_scope_admission.rs"]
mod successor_scope_admission;

use child_process::run_child;
use worth_proof::TransitionOutcome;
use worth_store::physical_runtime::{
    AdmittedPhysicalRecordFormat, AdmittedRecordAccessPolicy, AdmittedRecordPlacementPolicy,
    FilesystemMediaAdmission, ManifestEntryCapacity, MediaOwnedPhysicalRuntime,
    PhysicalRecordAccessPolicy, PhysicalRecordFormatDeclaration, PhysicalRecordInitialization,
    PhysicalRecordOpen, PhysicalRecordPlacementPolicy, PhysicalRuntimeAdmission, PhysicalStore,
    RecordBootstrapDenial, RecordReadObservation, RecordReadSession, RecordServingAdmissionOutcome,
    RecordStoreInitializationDenial, RecordStoreOpenDenial, ServingPhysicalRuntime,
};
use worth_store_physical_backend::FilesystemAccessPosture;

fn configuration() -> (
    AdmittedPhysicalRecordFormat,
    AdmittedRecordPlacementPolicy,
    AdmittedRecordAccessPolicy,
) {
    let format = AdmittedPhysicalRecordFormat::admit(
        PhysicalRecordFormatDeclaration::builder().admit().unwrap(),
    );
    let placement = PhysicalRecordPlacementPolicy::builder()
        .manifest_capacity(ManifestEntryCapacity::new(64).unwrap())
        .admit(format)
        .unwrap();
    let access = PhysicalRecordAccessPolicy::builder().admit(format).unwrap();
    (format, placement, access)
}

fn media(root: &Path) -> MediaOwnedPhysicalRuntime {
    let runtime = PhysicalStore::admit(PhysicalRuntimeAdmission::new(root).unwrap()).unwrap();
    match runtime
        .try_admit_filesystem_media(FilesystemMediaAdmission::production(
            FilesystemAccessPosture::CoordinatedServiceAccount,
        ))
        .into_raw()
    {
        TransitionOutcome::Success(media) => media,
        _ => panic!("real filesystem media admission must succeed"),
    }
}

fn serving_from_initialization(root: &Path) -> ServingPhysicalRuntime {
    let (format, placement, access) = configuration();
    success(
        media(root)
            .initialize_record_store(PhysicalRecordInitialization::new(format, placement, access)),
    )
}

fn serving_from_open(root: &Path) -> ServingPhysicalRuntime {
    let (format, _, access) = configuration();
    success(media(root).open_record_store(PhysicalRecordOpen::new(format, access)))
}

trait BootstrapDenialReason {
    fn reason(&self) -> RecordBootstrapDenial;
}

impl BootstrapDenialReason for RecordStoreInitializationDenial {
    fn reason(&self) -> RecordBootstrapDenial {
        self.reason()
    }
}

impl BootstrapDenialReason for RecordStoreOpenDenial {
    fn reason(&self) -> RecordBootstrapDenial {
        self.reason()
    }
}

fn success<Denial>(outcome: RecordServingAdmissionOutcome<Denial>) -> ServingPhysicalRuntime
where
    Denial: BootstrapDenialReason,
{
    match outcome.into_raw() {
        TransitionOutcome::Success(serving) => serving,
        TransitionOutcome::Denied(denial) => {
            panic!(
                "record-serving progression must succeed; denied by {:?}",
                denial.reason()
            )
        }
        _ => panic!("record-serving progression must succeed; non-denial outcome"),
    }
}

fn read_record(
    mut session: RecordReadSession,
    expected_bytes: usize,
) -> (Vec<u8>, RecordReadObservation) {
    let mut bytes = vec![0_u8; expected_bytes];
    let mut completed = 0;
    while completed < bytes.len() {
        let count = session.read_next(&mut bytes[completed..]).unwrap();
        assert!(count > 0, "the record ended before its declared length");
        completed += count;
    }
    assert_eq!(session.read_next(&mut [0_u8; 1]).unwrap(), 0);
    (bytes, session.observation())
}
