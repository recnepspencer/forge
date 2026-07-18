use std::collections::BTreeMap;
use std::io::Read;
use std::path::Path;
use std::process::{Command, ExitStatus, Stdio};
use std::time::{Duration, Instant};

use worth_store_test_support::structural_preflight::{
    StructuralPreflightPlan, StructuralToolDeclaration, StructuralToolExecutionEvidence,
};

use crate::evidence::sha256_bytes;

pub(super) struct ToolOutcome {
    pub failure: Option<String>,
    execution: StructuralToolExecutionEvidence,
}

impl ToolOutcome {
    pub fn identity(&self) -> &str {
        &self.execution.authority_identity
    }

    fn admit_tool_identity(&mut self, identity: &str) {
        self.execution
            .declared_tool_identities
            .push(identity.to_owned());
        self.execution.declared_tool_identities.sort();
        self.execution.declared_tool_identities.dedup();
        self.execution
            .seal_authority_identity()
            .expect("structural tool execution evidence is serializable");
    }
}

pub(super) fn execute_once(
    forge_root: &Path,
    plan: &StructuralPreflightPlan,
) -> BTreeMap<String, ToolOutcome> {
    let mut outcomes = BTreeMap::new();
    for tool in plan
        .predicates
        .iter()
        .filter_map(|predicate| predicate.tool.as_ref())
    {
        let key = command_identity(tool);
        match outcomes.entry(key) {
            std::collections::btree_map::Entry::Vacant(entry) => {
                entry.insert(run(forge_root, tool));
            }
            std::collections::btree_map::Entry::Occupied(mut entry) => {
                entry.get_mut().admit_tool_identity(&tool.tool_identity);
            }
        }
    }
    outcomes
}

pub(super) fn evidence(
    tools: &BTreeMap<String, ToolOutcome>,
) -> Vec<StructuralToolExecutionEvidence> {
    tools.values().map(|outcome| outcome.execution.clone()).collect()
}

fn run(root: &Path, tool: &StructuralToolDeclaration) -> ToolOutcome {
    let identity = command_identity(tool);
    let mut command = Command::new(&tool.resolved_program_path);
    command
        .args(&tool.arguments)
        .current_dir(root)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let mut child = match command.spawn() {
        Ok(child) => child,
        Err(error) => {
            return launch_failure(tool, identity, error.to_string());
        }
    };
    let process_id = child.id();
    let stdout = child.stdout.take().expect("piped structural-tool stdout");
    let stderr = child.stderr.take().expect("piped structural-tool stderr");
    let stdout_reader = std::thread::spawn(move || read_stream(stdout));
    let stderr_reader = std::thread::spawn(move || read_stream(stderr));
    let completion = wait_bounded(
        &mut child,
        Duration::from_millis(tool.timeout_millis),
    );
    let stdout = join_stream(stdout_reader, "stdout");
    let stderr = join_stream(stderr_reader, "stderr");
    let (status, timed_out, mut observation_failure) = match completion {
        Ok(completion) => (completion.status, completion.timed_out, None),
        Err(error) => (None, false, Some(error)),
    };
    if let Err(error) = &stdout {
        observation_failure.get_or_insert_with(|| error.clone());
    }
    if let Err(error) = &stderr {
        observation_failure.get_or_insert_with(|| error.clone());
    }
    let root_text = root.to_string_lossy();
    let stdout = stdout
        .as_deref()
        .ok()
        .map_or_else(String::new, |bytes| normalize(bytes, &root_text));
    let stderr = stderr
        .as_deref()
        .ok()
        .map_or_else(String::new, |bytes| normalize(bytes, &root_text));
    let exit_code = status.as_ref().and_then(ExitStatus::code);
    let successful = status.as_ref().is_some_and(ExitStatus::success)
        && !timed_out
        && observation_failure.is_none();
    let failure = observation_failure.clone().or_else(|| {
        (!successful).then(|| {
            if timed_out {
                format!(
                    "{} exceeded its declared {}ms timeout",
                    tool.tool_identity, tool.timeout_millis
                )
            } else {
                format!(
                    "{} rejected with {:?}: {}",
                    tool.tool_identity,
                    exit_code,
                    stderr
                )
            }
        })
    });
    outcome(
        tool,
        identity,
        ObservedToolExecution {
            process_id,
            exit_code,
            timed_out,
            stdout,
            stderr,
            observation_failure,
            successful,
        },
        failure,
    )
}

fn launch_failure(
    tool: &StructuralToolDeclaration,
    command_identity: String,
    reason: String,
) -> ToolOutcome {
    let failure = format!("could not launch {}: {reason}", tool.tool_identity);
    outcome(
        tool,
        command_identity,
        ObservedToolExecution {
            process_id: 0,
            exit_code: None,
            timed_out: false,
            stdout: String::new(),
            stderr: String::new(),
            observation_failure: Some(failure.clone()),
            successful: false,
        },
        Some(failure),
    )
}

fn outcome(
    tool: &StructuralToolDeclaration,
    command_identity: String,
    observed: ObservedToolExecution,
    failure: Option<String>,
) -> ToolOutcome {
    let mut execution = StructuralToolExecutionEvidence {
        command_identity,
        provenance: tool.provenance.clone(),
        program: tool.program.clone(),
        resolved_program_path: tool.resolved_program_path.clone(),
        program_sha256: tool.program_sha256.clone(),
        program_version_identity: tool.program_version_identity.clone(),
        arguments: tool.arguments.clone(),
        declared_tool_identities: vec![tool.tool_identity.clone()],
        timeout_millis: tool.timeout_millis,
        resource_posture: tool.resource_posture.clone(),
        process_id: observed.process_id,
        exit_code: observed.exit_code,
        timed_out: observed.timed_out,
        stdout_sha256: sha256_bytes(observed.stdout.as_bytes()),
        stderr_sha256: sha256_bytes(observed.stderr.as_bytes()),
        observation_failure: observed.observation_failure,
        successful: observed.successful,
        authority_identity: String::new(),
    };
    execution
        .seal_authority_identity()
        .expect("structural tool execution evidence is serializable");
    ToolOutcome { failure, execution }
}

struct ObservedToolExecution {
    process_id: u32,
    exit_code: Option<i32>,
    timed_out: bool,
    stdout: String,
    stderr: String,
    observation_failure: Option<String>,
    successful: bool,
}

fn wait_bounded(
    child: &mut std::process::Child,
    timeout: Duration,
) -> Result<ToolCompletion, String> {
    let deadline = Instant::now() + timeout;
    loop {
        let status = match child.try_wait() {
            Ok(status) => status,
            Err(error) => {
                let _ = child.kill();
                let _ = child.wait();
                return Err(format!("could not observe structural tool: {error}"));
            }
        };
        if let Some(status) = status {
            return Ok(ToolCompletion {
                status: Some(status),
                timed_out: false,
            });
        }
        if Instant::now() >= deadline {
            let kill_error = child.kill().err();
            let status = child
                .wait()
                .map_err(|error| format!("could not reap structural tool: {error}"))?;
            if let Some(error) = kill_error {
                return Err(format!("could not kill timed-out structural tool: {error}"));
            }
            return Ok(ToolCompletion {
                status: Some(status),
                timed_out: true,
            });
        }
        std::thread::sleep(Duration::from_millis(10));
    }
}

fn read_stream(mut stream: impl Read) -> std::io::Result<Vec<u8>> {
    const MAX_OUTPUT_BYTES: usize = 8 * 1024 * 1024;
    let mut bytes = Vec::new();
    let mut buffer = [0_u8; 64 * 1024];
    loop {
        let read = stream.read(&mut buffer)?;
        if read == 0 {
            break;
        }
        if bytes.len().saturating_add(read) > MAX_OUTPUT_BYTES {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "structural tool output exceeded 8 MiB",
            ));
        }
        bytes.extend_from_slice(&buffer[..read]);
    }
    Ok(bytes)
}

fn join_stream(
    reader: std::thread::JoinHandle<std::io::Result<Vec<u8>>>,
    stream: &str,
) -> Result<Vec<u8>, String> {
    reader
        .join()
        .map_err(|_| format!("structural tool {stream} reader panicked"))?
        .map_err(|error| format!("could not read structural tool {stream}: {error}"))
}

pub(super) fn command_identity(tool: &StructuralToolDeclaration) -> String {
    let encoded = serde_json::to_vec(&(
        &tool.provenance,
        &tool.program,
        &tool.resolved_program_path,
        &tool.program_sha256,
        &tool.program_version_identity,
        &tool.arguments,
        tool.timeout_millis,
        &tool.resource_posture,
    ))
    .expect("structural tool command identity is serializable");
    sha256_bytes(&encoded)
}

fn normalize(bytes: &[u8], root: &str) -> String {
    String::from_utf8_lossy(bytes)
        .replace(root, "<forge-root>")
        .replace('\\', "/")
}

struct ToolCompletion {
    status: Option<ExitStatus>,
    timed_out: bool,
}
