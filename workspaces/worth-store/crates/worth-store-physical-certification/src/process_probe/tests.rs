use std::process::Command;
use std::time::{Duration, Instant};

use sha2::Digest;

use super::*;

#[test]
fn probe_input_rejects_decoded_expected_runtime_truth() {
    let encoded = br#"{
        "scenario_identity":"offline",
        "fault_schedule_identity":"after-cut",
        "artifacts":[],
        "identity":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        "expected_state":{"generation":7}
    }"#;

    assert!(SealedProcessProbeInput::decode_untrusted(encoded).is_err());
}

#[test]
fn graceful_exit_cannot_satisfy_a_parent_kill_declaration() {
    assert!(!super::execution::termination_satisfies(
        ProcessTerminationRequirement::ParentKill,
        &ProcessTermination::GracefulExit { code: Some(0) },
    ));
}

#[test]
fn process_intent_rejects_incoherent_role_isolation_and_termination_claims() {
    let command = Command::new(std::env::current_exe().unwrap());
    let input = SealedProcessProbeInput::new("contract", "none", Vec::new()).unwrap();
    for (role, isolation, termination) in [
        (
            ProcessRole::RecoveredRuntime,
            ProcessIsolationRequirement::ParentTerminated,
            ProcessTerminationRequirement::ParentKill,
        ),
        (
            ProcessRole::OfflineVerifier,
            ProcessIsolationRequirement::FreshProcess,
            ProcessTerminationRequirement::GracefulExit,
        ),
        (
            ProcessRole::Writer,
            ProcessIsolationRequirement::IndependentObserver,
            ProcessTerminationRequirement::GracefulExit,
        ),
        (
            ProcessRole::AllocatorIsolatedProbe,
            ProcessIsolationRequirement::IsolatedAllocator,
            ProcessTerminationRequirement::Abort,
        ),
        (
            ProcessRole::Writer,
            ProcessIsolationRequirement::FreshProcess,
            ProcessTerminationRequirement::ParentKill,
        ),
    ] {
        assert!(ProcessProbeIntent::for_current_executable(
            &command,
            &input,
            role,
            isolation,
            termination,
        )
        .is_err());
    }
}

#[test]
fn unassisted_exit_observation_distinguishes_panic_abort_and_os_termination() {
    if let Some(mode) = std::env::var_os(TERMINATION_CHILD_ENV) {
        match mode.to_string_lossy().as_ref() {
            "panic" => panic!("declared panic-unwind probe"),
            "abort" => std::process::abort(),
            "os" => std::process::exit(7),
            _ => panic!("unknown termination probe mode"),
        }
    }
    for (mode, required) in [
        ("panic", ProcessTerminationRequirement::PanicUnwind),
        ("abort", ProcessTerminationRequirement::Abort),
        ("os", ProcessTerminationRequirement::OsTermination),
    ] {
        let status = Command::new(std::env::current_exe().unwrap())
            .args(["--exact", TERMINATION_TEST_IDENTITY, "--nocapture"])
            .env(TERMINATION_CHILD_ENV, mode)
            .status()
            .unwrap();
        assert!(super::execution::observe_required_exit(status, required).is_ok());
    }
}

#[test]
fn artifact_identity_observes_bytes_and_preexisting_state() {
    let directory = tempfile::tempdir().unwrap();
    let artifact = directory.path().join("artifact");
    let absent = ProcessArtifactPath::new("channel", &artifact).unwrap();
    std::fs::write(&artifact, b"first").unwrap();
    let first = ProcessArtifactPath::new("input", &artifact).unwrap();
    std::fs::write(&artifact, b"second").unwrap();
    let second = ProcessArtifactPath::new("input", &artifact).unwrap();

    assert_eq!(
        absent.initial_observation(),
        &ProcessArtifactObservation::Absent
    );
    assert_ne!(first.initial_observation(), second.initial_observation());
}

#[test]
fn process_evidence_root_rejects_parent_traversal_before_creation() {
    let root = tempfile::tempdir().unwrap();
    let admitted = root.path().join("evidence");
    std::fs::create_dir_all(&admitted).unwrap();
    let admitted = admitted.canonicalize().unwrap();
    let safe = root
        .path()
        .join("evidence/runs/attempt/process-probes/unit");
    let escaped = admitted.join("process/../../outside");

    assert_eq!(
        super::execution::admit_declared_evidence_root(&admitted, &safe),
        Ok(())
    );
    assert_eq!(
        super::execution::admit_declared_evidence_root(&admitted, &escaped),
        Err(ProcessProbeEvidenceDenial::EvidenceWrite)
    );
    assert!(!root.path().join("outside").exists());
}

#[test]
fn untrusted_probe_input_rejects_an_output_channel_that_became_visible() {
    let directory = tempfile::tempdir().unwrap();
    let output = directory.path().join("probe-output");
    let input = SealedProcessProbeInput::new(
        "output-admission",
        "before-child",
        vec![ProcessArtifactPath::output_channel("result", &output).unwrap()],
    )
    .unwrap();
    let encoded = super::wire_encoding::encode(&input).unwrap();
    std::fs::write(&output, b"preexisting-output").unwrap();

    assert!(SealedProcessProbeInput::decode_untrusted(&encoded).is_err());
}

#[test]
fn fresh_process_roles_share_executable_but_not_process_or_runtime_identity() {
    if std::env::var_os(PROBE_CHILD_ENV).is_some() {
        run_probe_child();
        return;
    }
    let directory = tempfile::tempdir().unwrap();
    let persisted = directory.path().join("persisted-input");
    std::fs::write(&persisted, b"persisted-agreement").unwrap();
    let writer = run_role(
        directory.path(),
        &persisted,
        "writer-cut",
        ProcessRole::Writer,
        ProcessIsolationRequirement::ParentTerminated,
        ProcessTerminationRequirement::ParentKill,
    );
    let recovered = run_role(
        directory.path(),
        &persisted,
        "recovered-runtime",
        ProcessRole::RecoveredRuntime,
        ProcessIsolationRequirement::FreshProcess,
        ProcessTerminationRequirement::GracefulExit,
    );
    let verifier = run_role(
        directory.path(),
        &persisted,
        "offline-verifier-a",
        ProcessRole::OfflineVerifier,
        ProcessIsolationRequirement::IndependentObserver,
        ProcessTerminationRequirement::GracefulExit,
    );
    let second_verifier = run_role(
        directory.path(),
        &persisted,
        "offline-verifier-b",
        ProcessRole::OfflineVerifier,
        ProcessIsolationRequirement::IndependentObserver,
        ProcessTerminationRequirement::GracefulExit,
    );
    let executions = [&writer, &recovered, &verifier, &second_verifier];

    assert!(executions.iter().all(
        |execution| execution.process.executable_identity == writer.process.executable_identity
    ));
    assert!(executions
        .iter()
        .all(|execution| execution.output_artifact_identity == writer.output_artifact_identity));
    let process_ids = executions
        .iter()
        .map(|execution| execution.process.process_id)
        .collect::<std::collections::BTreeSet<_>>();
    let runtime_ids = executions
        .iter()
        .map(|execution| execution.process.runtime_identity.unwrap())
        .collect::<std::collections::BTreeSet<_>>();
    let evidence_ids = executions
        .iter()
        .map(|execution| execution.evidence_identity)
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(process_ids.len(), executions.len());
    assert_eq!(runtime_ids.len(), executions.len());
    assert_eq!(evidence_ids.len(), executions.len());
    assert!(executions.iter().all(|execution| execution
        .process
        .environment
        .iter()
        .all(|binding| binding.name != UNADMITTED_EXPECTED_STATE_ENV)));
    #[cfg(windows)]
    assert!(executions.iter().all(|execution| execution
        .process
        .environment
        .iter()
        .any(|binding| binding.name == "TEMP" || binding.name == "TMP")));
    assert!(matches!(
        writer.termination,
        ProcessTermination::ParentKill { .. }
    ));
    assert!(matches!(
        recovered.termination,
        ProcessTermination::GracefulExit { code: Some(0) }
    ));
}

fn run_role(
    root: &std::path::Path,
    persisted: &std::path::Path,
    scenario: &str,
    role: ProcessRole,
    isolation: ProcessIsolationRequirement,
    termination: ProcessTerminationRequirement,
) -> ProcessProbeExecution {
    let observation = root.join(format!("{scenario}.process.json"));
    let output = root.join(format!("{scenario}.output"));
    let executable = std::env::current_exe().unwrap();
    let mut command = Command::new(executable);
    command
        .args(["--exact", PROBE_TEST_IDENTITY, "--nocapture"])
        .env(PROBE_CHILD_ENV, role_token(role))
        .env(PROBE_OUTPUT_ENV, &output)
        .env(UNADMITTED_EXPECTED_STATE_ENV, "decoded-runtime-truth");
    let input = SealedProcessProbeInput::new(
        scenario,
        "declared-process-role",
        vec![
            ProcessArtifactPath::new("persisted-input", persisted).unwrap(),
            ProcessArtifactPath::output_channel("probe-output", &output).unwrap(),
        ],
    )
    .unwrap();
    let intent =
        ProcessProbeIntent::for_current_executable(&command, &input, role, isolation, termination)
            .unwrap();
    let declaration = configure_process_probe(
        &mut command,
        intent,
        &input,
        &observation,
        &[PROBE_CHILD_ENV, PROBE_OUTPUT_ENV],
    )
    .unwrap();
    let mut child = command.spawn().unwrap();
    wait_for_paths(&mut child, &[&observation, &output]);
    let process_id = child.id();
    let observed_termination = if termination == ProcessTerminationRequirement::ParentKill {
        terminate_by_parent(&mut child).unwrap()
    } else {
        observe_graceful_exit(child.wait().unwrap()).unwrap()
    };
    let process = read_process_observation(&observation, &declaration, process_id).unwrap();
    let execution = ProcessProbeExecution::observed(
        declaration,
        &input,
        process,
        observed_termination,
        &output,
    )
    .unwrap();
    persist_execution(root, &execution).unwrap();
    execution
}

fn run_probe_child() {
    assert!(
        std::env::var_os(UNADMITTED_EXPECTED_STATE_ENV).is_none(),
        "unadmitted ambient expected state crossed the process boundary"
    );
    let role = parse_test_role(&std::env::var(PROBE_CHILD_ENV).unwrap());
    let admission = admit_current_process_probe(role).unwrap();
    let output = std::path::PathBuf::from(std::env::var_os(PROBE_OUTPUT_ENV).unwrap());
    crate::certification_child_process::publish_new_synced(&output, b"persisted-agreement")
        .unwrap();
    let runtime_identity =
        sha2::Sha256::digest(format!("fresh-runtime-{}-{role:?}", std::process::id()).as_bytes())
            .into();
    write_current_process_observation(&admission, Some(runtime_identity)).unwrap();
    if role == ProcessRole::Writer {
        loop {
            std::thread::park();
        }
    }
}

fn wait_for_paths(child: &mut std::process::Child, paths: &[&std::path::Path]) {
    let deadline = Instant::now() + Duration::from_secs(10);
    while !paths.iter().all(|path| path.is_file()) {
        assert!(
            child.try_wait().unwrap().is_none(),
            "probe child exited early"
        );
        assert!(Instant::now() < deadline, "probe child readiness timed out");
        std::thread::sleep(Duration::from_millis(5));
    }
}

const fn role_token(role: ProcessRole) -> &'static str {
    match role {
        ProcessRole::Writer => "writer",
        ProcessRole::RecoveredRuntime => "recovered-runtime",
        ProcessRole::OfflineVerifier => "offline-verifier",
        _ => panic!("test only admits writer, recovered runtime, and verifier"),
    }
}

fn parse_test_role(value: &str) -> ProcessRole {
    match value {
        "writer" => ProcessRole::Writer,
        "recovered-runtime" => ProcessRole::RecoveredRuntime,
        "offline-verifier" => ProcessRole::OfflineVerifier,
        _ => panic!("unknown process probe test role"),
    }
}

const PROBE_CHILD_ENV: &str = "WORTH_STORE_PROCESS_PROBE_TEST_CHILD";
const PROBE_OUTPUT_ENV: &str = "WORTH_STORE_PROCESS_PROBE_TEST_OUTPUT";
const UNADMITTED_EXPECTED_STATE_ENV: &str = "WORTH_STORE_PROCESS_PROBE_UNADMITTED_EXPECTED_STATE";
const TERMINATION_CHILD_ENV: &str = "WORTH_STORE_PROCESS_PROBE_TERMINATION_CHILD";
const TERMINATION_TEST_IDENTITY: &str =
    "process_probe::tests::unassisted_exit_observation_distinguishes_panic_abort_and_os_termination";
const PROBE_TEST_IDENTITY: &str =
    "process_probe::tests::fresh_process_roles_share_executable_but_not_process_or_runtime_identity";
