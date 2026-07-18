use std::process::Command;

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
    let command = Command::new(executable);
    let input = SealedProcessProbeInput::new("cut", "fault", Vec::new()).unwrap();
    let declaration = ProcessProbeDeclaration::for_current_executable(
        &command,
        &input,
        ProcessRole::CrashTarget,
        ProcessIsolationRequirement::ParentTerminated,
        ProcessTerminationRequirement::ParentKill,
    )
    .unwrap();
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
            process,
            ProcessTermination::GracefulExit { code: Some(0) },
            output.path(),
        ),
        Err(ProcessProbeEvidenceDenial::TerminationMismatch)
    );
}
