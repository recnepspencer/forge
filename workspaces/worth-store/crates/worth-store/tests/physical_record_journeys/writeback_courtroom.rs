use std::io::Write;
use std::path::Path;

use sha2::{Digest, Sha256};
use worth_store::physical_runtime::{PhysicalWorkEffectFate, ServingPhysicalRuntime};
use worth_store_contracts::QueueProducerResourceShape;
use worth_store_io_scheduler::{
    admit_secure_io_scope_for_scheduler, admit_security_scope_for_scheduler, SecureIoOperation,
    SecureIoPostureRequirement, SecureIoPreservationRequest,
};
use worth_store_physical_format::{RecordArtifactFile, RecordFrameCoordinate};

#[test]
fn physical_writeback_survives_process_exit_and_fresh_store_admission() {
    let parent = tempfile::tempdir().unwrap();
    let root = parent.path().join("store");
    let writer = super::run_child("c6_writeback_writer", &root, None);
    let digest = writer
        .lines()
        .find_map(|line| line.strip_prefix("C6_WRITEBACK "))
        .expect("writer must publish the predeclared digest");
    let observer = super::run_child("c6_writeback_observer", &root, Some(digest));
    assert!(observer.lines().any(|line| line == "C6_WRITEBACK_OBSERVED"));
    let reopener = super::run_child("c6_writeback_reopener", &root, None);
    assert!(reopener.lines().any(|line| line == "C6_WRITEBACK_REOPENED"));
}

pub(super) fn writer(root: &Path) {
    let (profile, _, request) = super::physical_work::work_fixture();
    let serving =
        super::physical_work::serving_from_initialization_with_work_profile(root, profile);
    let target = root.join("families/records/bootstrap.catalog");
    let bytes = std::fs::read(target).unwrap()[8..16].to_vec();
    let coordinate =
        RecordFrameCoordinate::new(RecordArtifactFile::BootstrapCatalog, 8, 8).unwrap();
    serving
        .certification_admit_dirty_frame(coordinate, bytes.clone())
        .unwrap();
    let admitted = admitted_writeback(&serving, request);
    let command = serving
        .physical_residency_writeback_command(admitted)
        .unwrap();
    let outcome = serving.execute_physical_work(command).unwrap();
    assert_eq!(
        outcome.settled().evidence().fate(),
        PhysicalWorkEffectFate::WriteCompleted
    );
    assert_eq!(serving.residency_counters().dirty_frames(), 0);
    println!("C6_WRITEBACK {}", hex(&Sha256::digest(bytes)));
    std::io::stdout().flush().unwrap();
    std::process::exit(0);
}

pub(super) fn observer(root: &Path, expected_digest: &str) {
    let bytes = std::fs::read(root.join("families/records/bootstrap.catalog")).unwrap();
    assert_eq!(hex(&Sha256::digest(&bytes[8..16])), expected_digest);
    println!("C6_WRITEBACK_OBSERVED");
    std::io::stdout().flush().unwrap();
}

pub(super) fn reopener(root: &Path) {
    let serving = super::serving_from_open(root);
    assert!(!serving.observed_non_authoritative_residue());
    println!("C6_WRITEBACK_REOPENED");
    std::io::stdout().flush().unwrap();
    serving.close();
}

fn admitted_writeback(
    serving: &ServingPhysicalRuntime,
    request: worth_store::physical_runtime::PhysicalMutationWorkRequest,
) -> worth_store::physical_runtime::ResourceAdmittedPhysicalWork {
    let ready = super::physical_work::ready_work(serving, request);
    let reservation = super::physical_work::reserved_page_write(serving);
    let shape = QueueProducerResourceShape::new()
        .with_queue_slots(1)
        .with_bandwidth_tokens(8)
        .with_write_back_windows(1)
        .with_worker_permits(1);
    let demand = serving
        .prepare_physical_residency_writeback(ready, reservation, 7, shape, None)
        .unwrap();
    let work = demand.queue_work();
    let backend = serving
        .admit_physical_scheduler_capability(work.backend_requirement())
        .unwrap();
    let security_scope =
        worth_store_security::admitted_store_internal_security_scope_for_io_qos_test();
    let scope = admit_security_scope_for_scheduler(&security_scope).unwrap();
    let secure_io = admit_secure_io_scope_for_scheduler(
        SecureIoPreservationRequest::new(SecureIoOperation::WriteBack, &scope, &backend)
            .require_posture(SecureIoPostureRequirement::ScopePreserving),
    )
    .unwrap();
    let demand = demand.with_secure_io(secure_io);
    serving
        .admit_physical_scheduler_demand(
            demand,
            &backend,
            super::physical_work::policy_receipt(work.requested_budget()),
        )
        .unwrap()
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}
