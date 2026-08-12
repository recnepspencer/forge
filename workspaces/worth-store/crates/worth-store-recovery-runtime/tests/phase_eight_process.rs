#[allow(dead_code)]
mod phase_three_support;

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use worth_proof::TransitionOutcome;
use worth_store::physical_runtime::{
    PhysicalCheckpointDeadline, PhysicalCheckpointIdempotencyKey, PhysicalCheckpointOutcome,
    PhysicalCheckpointRequest,
};
use worth_store_offline_verifier::{
    observe_recovery_artifacts, RecoveryObserverDecodeDenial, RecoveryObserverLimits,
    RecoveryObserverReport, RECOVERY_OBSERVER_REPORT_PROTOCOL,
};
use worth_store_recovery_runtime::{
    RecoveryReportDecodeDenial, RecoveryReportEnvelope, RecoveryReportOutcome,
    RECOVERY_REPORT_PROTOCOL,
};
use worth_store_test_support::harness::physical_residency::{
    canonical_physical_mutation_acknowledgment, PhysicalResidencyStoreWorld,
};

const CHILD_ROOT: &str = "WORTH_C8_PHASE8_CHILD_ROOT";
const OBSERVER_REPORT: &str = "WORTH_C8_PHASE8_OBSERVER_REPORT";
const WRITER_TEST: &str = "phase_eight_writer_process";
const OBSERVER_TEST: &str = "phase_eight_observer_process";

#[test]
fn production_recovery_and_independent_observer_emit_distinct_process_reports() {
    let parent = tempfile::tempdir().expect("Phase 8 process parent");
    let marker = parent.path().join("persisted-root");
    let writer = run_child(
        WRITER_TEST,
        parent.path(),
        [(CHILD_ROOT, marker.as_os_str())],
    );
    assert_child_succeeded("writer", &writer);

    let root = PathBuf::from(std::fs::read_to_string(&marker).expect("persisted root marker"));
    let runtime_report = parent.path().join("runtime-report.bin");
    let recovery = Command::new(env!("CARGO_BIN_EXE_physical_store_recover"))
        .arg(&root)
        .arg("--bounded-profile=c8-phase2-admission-v1")
        .arg(format!("--report={}", runtime_report.display()))
        .env("TMP", parent.path())
        .env("TEMP", parent.path())
        .env("TMPDIR", parent.path())
        .output()
        .expect("launch production recovery command");
    assert_child_succeeded("production recovery", &recovery);

    let observer_report = parent.path().join("observer-report.bin");
    let observer = run_child(
        OBSERVER_TEST,
        parent.path(),
        [
            (CHILD_ROOT, root.as_os_str()),
            (OBSERVER_REPORT, observer_report.as_os_str()),
        ],
    );
    assert_child_succeeded("offline observer", &observer);

    let runtime_bytes = std::fs::read(runtime_report).expect("runtime report bytes");
    let runtime = RecoveryReportEnvelope::decode(&runtime_bytes).expect("runtime report decode");
    assert_eq!(runtime.outcome(), RecoveryReportOutcome::Recovered);
    assert!(runtime.store_identity().is_some());
    assert!(runtime.root_generation().is_some());
    assert!(runtime.counters().recovery_effects() > 0);

    let observer_bytes = std::fs::read(observer_report).expect("observer report bytes");
    let observer = RecoveryObserverReport::decode(&observer_bytes).expect("observer report decode");
    assert!(observer.artifact_count() > 0);
    assert!(observer.bytes_read() > 0);
    assert_ne!(observer.artifact_set_digest(), [0; 32]);

    assert_ne!(RECOVERY_REPORT_PROTOCOL, RECOVERY_OBSERVER_REPORT_PROTOCOL);
    assert!(matches!(
        RecoveryReportEnvelope::decode(&observer_bytes),
        Err(RecoveryReportDecodeDenial::WrongProtocolFamily)
    ));
    assert!(matches!(
        RecoveryObserverReport::decode(&runtime_bytes),
        Err(RecoveryObserverDecodeDenial::WrongProtocolFamily)
    ));
}

#[test]
#[ignore = "launched by the Phase 8 report-boundary parent"]
fn phase_eight_writer_process() {
    let marker = required_path(CHILD_ROOT);
    let world = PhysicalResidencyStoreWorld::initialize_for_recovery("c8-phase8-writer")
        .expect("ordinary persisted Store world");
    let retained_root = world.retained_root();
    canonical_physical_mutation_acknowledgment(&world, [0x81; 32], b"phase-eight-report");
    publish_checkpoint(&world);
    drop(world);
    let root = retained_root.persist();
    std::fs::write(marker, root.to_string_lossy().as_bytes()).expect("persisted root marker");
}

#[test]
#[ignore = "launched by the Phase 8 report-boundary parent"]
fn phase_eight_observer_process() {
    let root = required_path(CHILD_ROOT);
    let output = required_path(OBSERVER_REPORT);
    let limits =
        RecoveryObserverLimits::new(16_384, 512 * 1024 * 1024).expect("bounded observer limits");
    let report = observe_recovery_artifacts(&root, limits).expect("independent artifact walk");
    std::fs::write(output, report.encode()).expect("observer report output");
}

fn publish_checkpoint(world: &PhysicalResidencyStoreWorld) {
    let request = PhysicalCheckpointRequest::fuzzy(
        PhysicalCheckpointIdempotencyKey::new([0x82; 32]),
        PhysicalCheckpointDeadline::after_milliseconds(5_000).expect("checkpoint deadline"),
    );
    let TransitionOutcome::Success(handle) =
        world.serving().checkpoints().start(request).into_raw()
    else {
        panic!("checkpoint admission must succeed")
    };
    assert!(matches!(
        handle.wait(),
        PhysicalCheckpointOutcome::Completed(_)
    ));
}

fn run_child<const N: usize>(
    test: &str,
    temporary_root: &Path,
    environment: [(&str, &std::ffi::OsStr); N],
) -> Output {
    let mut command = Command::new(std::env::current_exe().expect("integration test executable"));
    command
        .args(["--exact", test, "--ignored", "--nocapture"])
        .envs(environment)
        .env("TMP", temporary_root)
        .env("TEMP", temporary_root)
        .env("TMPDIR", temporary_root);
    command.output().expect("launch Phase 8 child process")
}

fn required_path(name: &str) -> PathBuf {
    std::env::var_os(name)
        .map(PathBuf::from)
        .unwrap_or_else(|| panic!("missing {name}"))
}

fn assert_child_succeeded(name: &str, output: &Output) {
    assert!(
        output.status.success(),
        "{name} child failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );
}
