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
    let executable = std::env::current_exe().unwrap();
    let mut command = Command::new(executable);
    let input = SealedProcessProbeInput::new("cut", "fault", Vec::new()).unwrap();
    let intent = ProcessProbeIntent::for_current_executable(
        &command,
        &input,
        ProcessRole::CrashTarget,
        ProcessIsolationRequirement::ParentTerminated,
        ProcessTerminationRequirement::ParentKill,
    )
    .unwrap();
    let observation = tempfile::NamedTempFile::new().unwrap();
    let declaration =
        configure_process_probe(&mut command, intent, &input, observation.path(), &[]).unwrap();
    let working_directory = declaration.working_directory().to_owned();
    let process = ProcessIdentityEvidence {
        role: ProcessRole::CrashTarget,
        executable_identity: declaration.executable_identity(),
        process_id: std::process::id().saturating_add(1),
        launch_parent_process_id: std::process::id(),
        working_directory_identity: sha2::Sha256::digest(working_directory.as_bytes()).into(),
        working_directory,
        environment: Vec::new(),
        environment_identity: sha2::Sha256::digest(b"[]").into(),
        input_artifact_identity: declaration.input_identity(),
        runtime_identity: None,
    };
    let output = tempfile::NamedTempFile::new().unwrap();

    assert_eq!(
        ProcessProbeExecution::observed(
            declaration,
            &input,
            process,
            ProcessTermination::GracefulExit { code: Some(0) },
            output.path(),
        ),
        Err(ProcessProbeEvidenceDenial::TerminationMismatch)
    );
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
    let escaped = admitted.join("process/../../outside");

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
    let encoded = serde_json::to_vec(&input).unwrap();
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
        child.kill().unwrap();
        let status = child.wait().unwrap();
        ProcessTermination::ParentKill {
            platform_status: format!("{status:?}"),
        }
    } else {
        classify_exit(child.wait().unwrap())
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
const PROBE_TEST_IDENTITY: &str =
    "process_probe::tests::fresh_process_roles_share_executable_but_not_process_or_runtime_identity";
