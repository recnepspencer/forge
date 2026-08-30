use std::path::Path;
use std::process::{Command, ExitStatus};

use serde_json::Value;
use sha2::{Digest, Sha256};

use super::process_courtroom_assertions::{
    assert_addressed_root_poison_preserves_current_selector, assert_offline_expectation,
    assert_recovery_expectation, DecoderCounters,
};
use super::process_identity_substitution::assert_recovery_store_substitution_is_denied;
use super::process_manifest::ProcessTreeSnapshot;
use super::process_protocol::{
    executable_sha256, read_wire, write_create_new, ProcessReportPayload, ProcessSubjectReport,
    ProcessSubjectRequest, SUBJECT_REQUEST_ENV,
};
use super::{DeclaredProcessPoison, ExternalReportPaths, ProcessRootCase, RootWireRole};

pub(super) fn run(observer_executable: &Path) {
    assert!(
        observer_executable.is_file(),
        "Cargo-owned observer executable is unavailable"
    );
    let world = tempfile::tempdir().expect("courtroom directory");
    let stores = world.path().join("stores");
    let reports = world.path().join("reports");
    std::fs::create_dir_all(&stores).expect("Store rows directory");
    std::fs::create_dir_all(&reports).expect("external reports directory");
    let parent_executable = std::env::current_exe().expect("courtroom executable");
    let parent_digest = executable_sha256(&parent_executable).expect("courtroom digest");
    let producer_scenario = fresh_identity("producer-scenario", world.path());
    let producer_run = fresh_identity("producer-run", world.path());
    let baseline = stores.join("closed-production-baseline");
    let producer = run_subject(
        &parent_executable,
        &reports,
        "producer",
        ProcessSubjectRequest::producer(
            producer_scenario,
            producer_run,
            baseline.clone(),
            reports.join("producer.report"),
        ),
    );
    let manifest = match producer.report.payload() {
        ProcessReportPayload::Produced(manifest) => manifest.clone(),
        _ => panic!("producer report payload substitution"),
    };
    producer
        .report
        .require(
            RootWireRole::Producer,
            producer_scenario,
            producer_run,
            manifest.store_identity(),
            producer.process_id,
            parent_digest,
        )
        .expect("producer report binding");
    assert!(manifest.file_count() > 0);
    assert!(manifest.byte_count() > 0);
    manifest
        .require_unchanged(&baseline)
        .expect("producer baseline remains exact after producer exit");

    let observer_digest = executable_sha256(observer_executable).expect("observer digest");
    assert_ne!(
        observer_digest, parent_digest,
        "recovery/verifier binary reuse"
    );
    let mut clean_counters = None;
    for case in [
        ProcessRootCase::CleanControl,
        ProcessRootCase::PoisonCurrentSelector,
        ProcessRootCase::PoisonAddressedRoot,
    ] {
        let row = stores.join(case.label());
        manifest.copy_to(&baseline, &row).expect("fresh exact row");
        let scenario = fresh_identity(&format!("{}-scenario", case.label()), world.path());
        let declared_poison = case.role().map(|_| {
            DeclaredProcessPoison::for_case(&manifest, case)
                .expect("independent poison declaration")
        });
        if let Some(declaration) = declared_poison.as_ref() {
            let editor_snapshot =
                ProcessTreeSnapshot::observe(&row).expect("snapshot row before editor process");
            let editor_run = fresh_identity(&format!("{}-editor", case.label()), world.path());
            let editor = run_subject(
                &parent_executable,
                &reports,
                &format!("{}-editor", case.label()),
                ProcessSubjectRequest::editor(
                    scenario,
                    editor_run,
                    row.clone(),
                    reports.join(format!("{}-editor.report", case.label())),
                    manifest.clone(),
                    declaration.clone(),
                ),
            );
            editor
                .report
                .require(
                    RootWireRole::ArtifactEditor,
                    scenario,
                    editor_run,
                    manifest.store_identity(),
                    editor.process_id,
                    parent_digest,
                )
                .expect("editor report binding");
            let audit = match editor.report.payload() {
                ProcessReportPayload::Edited(audit) => audit,
                _ => panic!("editor report payload substitution"),
            };
            let artifact = manifest
                .artifact(declaration.role())
                .expect("declared editor target");
            let (actual_before_sha256, actual_after_sha256) = editor_snapshot
                .require_exact_one_byte_delta(
                    &row,
                    artifact.relative_path(),
                    declaration.offset(),
                    declaration.xor_mask(),
                )
                .expect("editor changes exactly the declared byte and nothing else");
            assert_eq!(audit.declaration_identity(), declaration.identity());
            assert_eq!(audit.changed_offset(), declaration.offset());
            assert_eq!(audit.before_sha256(), actual_before_sha256);
            assert_eq!(audit.after_sha256(), actual_after_sha256);
            if case == ProcessRootCase::PoisonAddressedRoot {
                assert_addressed_root_poison_preserves_current_selector(&row, &manifest);
            }
        }
        let observer_snapshot =
            ProcessTreeSnapshot::observe(&row).expect("snapshot row before offline observation");
        let observer_run = fresh_identity(&format!("{}-observer", case.label()), world.path());
        let offline_report = reports.join(format!("{}-offline.json", case.label()));
        let observed = run_offline_observer(
            observer_executable,
            &row,
            &offline_report,
            observer_run,
            scenario,
        );
        assert_eq!(
            observed.report["protocol"],
            "store.physical.integrity-observation"
        );
        assert_eq!(observed.report["version"], 1);
        assert_eq!(observed.report["role"], "offline-root-observer");
        assert_eq!(
            observed.report["executable"],
            "physical_store_integrity_observer"
        );
        assert_eq!(observed.report["process"], observed.process_id.to_string());
        assert_eq!(observed.report["run"], hex(observer_run));
        assert_eq!(observed.report["scenario"], hex(scenario));
        assert_eq!(observed.report["store"], hex(manifest.store_identity()));
        assert_eq!(observed.executable_sha256, observer_digest);
        assert_eq!(
            observed.report["completeness"], "complete",
            "offline report: {}",
            observed.report
        );
        assert_offline_expectation(&observed.report, &manifest, declared_poison.as_ref());
        observer_snapshot
            .require_unchanged(&row)
            .expect("offline observer leaves isolated Store row byte-exact");
        let counters = DecoderCounters::from_report(&observed.report);
        match case {
            ProcessRootCase::CleanControl => {
                assert!(counters.checksum_calculations > 0);
                assert!(counters.checksum_validated_frames > 0);
                assert!(counters.selector_payload_entries > 0);
                assert!(counters.root_manifest_payload_entries > 0);
                clean_counters = Some(counters);
            }
            ProcessRootCase::PoisonCurrentSelector => {
                let clean = clean_counters.expect("clean row precedes poison rows");
                assert_eq!(counters.checksum_calculations, clean.checksum_calculations);
                assert_eq!(
                    counters.checksum_validated_frames + 1,
                    clean.checksum_validated_frames
                );
                assert_eq!(
                    counters.selector_payload_entries + 1,
                    clean.selector_payload_entries
                );
                assert_eq!(
                    counters.root_manifest_payload_entries,
                    clean.root_manifest_payload_entries
                );
            }
            ProcessRootCase::PoisonAddressedRoot => {
                let clean = clean_counters.expect("clean row precedes poison rows");
                assert_eq!(counters.checksum_calculations, clean.checksum_calculations);
                assert_eq!(
                    counters.checksum_validated_frames + 1,
                    clean.checksum_validated_frames
                );
                assert_eq!(
                    counters.selector_payload_entries,
                    clean.selector_payload_entries
                );
                assert_eq!(
                    counters.root_manifest_payload_entries + 1,
                    clean.root_manifest_payload_entries
                );
            }
        }
        let recovery_run = fresh_identity(&format!("{}-recovery", case.label()), world.path());
        let recovery = run_subject(
            &parent_executable,
            &reports,
            &format!("{}-recovery", case.label()),
            ProcessSubjectRequest::recovery(
                scenario,
                recovery_run,
                row.clone(),
                reports.join(format!("{}-recovery.report", case.label())),
                manifest.store_identity(),
            ),
        );
        recovery
            .report
            .require(
                RootWireRole::Recovery,
                scenario,
                recovery_run,
                manifest.store_identity(),
                recovery.process_id,
                parent_digest,
            )
            .expect("recovery report binding");
        let recovery_observation = match recovery.report.payload() {
            ProcessReportPayload::Recovered(observation) => observation,
            _ => panic!("recovery report payload substitution"),
        };
        assert_recovery_expectation(recovery_observation, &manifest, case);
    }
    assert_recovery_store_substitution_is_denied(
        &parent_executable,
        &stores,
        &reports,
        &baseline,
        &manifest,
        fresh_identity("recovery-store-substitution-scenario", world.path()),
        fresh_identity("recovery-store-substitution-run", world.path()),
    );
    manifest
        .require_unchanged(&baseline)
        .expect("courtroom never edits the production baseline");
}

struct SubjectExecution {
    process_id: u32,
    report: ProcessSubjectReport,
}

fn run_subject(
    executable: &Path,
    reports: &Path,
    label: &str,
    request: ProcessSubjectRequest,
) -> SubjectExecution {
    let request_path = reports.join(format!("{label}.request"));
    let report_path =
        ExternalReportPaths::authorize(request.store_root(), request.report_path().to_path_buf())
            .expect("external process report path");
    let request_path = ExternalReportPaths::authorize(request.store_root(), request_path)
        .expect("external process request path");
    write_create_new(request_path.as_path(), &request).expect("write subject request");
    let mut child = Command::new(executable)
        .args([
            "--exact",
            "c9_integrity_localization::c9_root_process_subject",
            "--nocapture",
        ])
        .env(SUBJECT_REQUEST_ENV, request_path.as_path())
        .spawn()
        .expect("launch process subject");
    let process_id = child.id();
    assert_ne!(
        process_id,
        std::process::id(),
        "subject must be a child process"
    );
    let status = child.wait().expect("wait for process subject");
    assert_success(status, "process subject");
    let report = read_wire(report_path.as_path()).expect("read process report");
    SubjectExecution { process_id, report }
}

struct OfflineExecution {
    process_id: u32,
    executable_sha256: [u8; 32],
    report: Value,
}

fn run_offline_observer(
    executable: &Path,
    store_root: &Path,
    report_path: &Path,
    run: [u8; 32],
    scenario: [u8; 32],
) -> OfflineExecution {
    let report_path = ExternalReportPaths::authorize(store_root, report_path.to_path_buf())
        .expect("external offline report path");
    let mut child = Command::new(executable)
        .args([
            "observe",
            "--store-root",
            store_root.to_str().expect("UTF-8 Store root"),
            "--report",
            report_path.as_path().to_str().expect("UTF-8 report path"),
            "--max-entries",
            "4096",
            "--max-bytes",
            "134217728",
            "--max-open-files",
            "16",
            "--max-depth",
            "16",
            "--max-symlinks",
            "1",
            "--max-elapsed-ms",
            "30000",
            "--max-report-bytes",
            "1048576",
            "--run",
            &hex(run),
            "--scenario",
            &hex(scenario),
        ])
        .spawn()
        .expect("launch independent offline observer");
    let process_id = child.id();
    assert_ne!(
        process_id,
        std::process::id(),
        "offline observer must be a child process"
    );
    let status = child.wait().expect("wait for offline observer");
    assert_success(status, "offline observer");
    let report =
        serde_json::from_slice(&std::fs::read(report_path.as_path()).expect("read offline report"))
            .expect("parse independent report wire");
    OfflineExecution {
        process_id,
        executable_sha256: executable_sha256(executable).expect("observer digest"),
        report,
    }
}

fn assert_success(status: ExitStatus, role: &str) {
    assert!(status.success(), "{role} exited with {status}");
}

fn fresh_identity(label: &str, root: &Path) -> [u8; 32] {
    Sha256::digest(format!("{label}:{}:{}", root.display(), std::process::id())).into()
}

fn hex(bytes: impl AsRef<[u8]>) -> String {
    bytes
        .as_ref()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}
