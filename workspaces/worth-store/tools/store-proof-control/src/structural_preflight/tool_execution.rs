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
    pub identity: String,
    pub failure: Option<String>,
    declaration: StructuralToolDeclaration,
    command_identity: String,
    declared_tool_identities: Vec<String>,
    process_id: u32,
    exit_code: Option<i32>,
    timed_out: bool,
    successful: bool,
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
                entry
                    .get_mut()
                    .declared_tool_identities
                    .push(tool.tool_identity.clone());
            }
        }
    }
    outcomes
}

pub(super) fn evidence(
    tools: &BTreeMap<String, ToolOutcome>,
) -> Vec<StructuralToolExecutionEvidence> {
    tools
        .values()
        .map(|outcome| {
            let mut declared_tool_identities = outcome.declared_tool_identities.clone();
            declared_tool_identities.sort();
            declared_tool_identities.dedup();
            StructuralToolExecutionEvidence {
                command_identity: outcome.command_identity.clone(),
                provenance: outcome.declaration.provenance.clone(),
                program: outcome.declaration.program.clone(),
                resolved_program_path: outcome.declaration.resolved_program_path.clone(),
                program_sha256: outcome.declaration.program_sha256.clone(),
                program_version_identity: outcome
                    .declaration
                    .program_version_identity
                    .clone(),
                arguments: outcome.declaration.arguments.clone(),
                declared_tool_identities,
                timeout_millis: outcome.declaration.timeout_millis,
                resource_posture: outcome.declaration.resource_posture.clone(),
                process_id: outcome.process_id,
                exit_code: outcome.exit_code,
                timed_out: outcome.timed_out,
                successful: outcome.successful,
                authority_identity: outcome.identity.clone(),
            }
        })
        .collect()
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
    let authority_identity = sha256_bytes(
        format!(
            "{}\n{:?}\n{}\n{}\n{}",
            identity,
            exit_code,
            timed_out,
            stdout,
            stderr
        )
        .as_bytes(),
    );
    let failure = observation_failure.or_else(|| {
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
    ToolOutcome {
        identity: authority_identity,
        failure,
        declaration: tool.clone(),
        command_identity: identity,
        declared_tool_identities: vec![tool.tool_identity.clone()],
        process_id,
        exit_code,
        timed_out,
        successful,
    }
}

fn launch_failure(
    tool: &StructuralToolDeclaration,
    command_identity: String,
    reason: String,
) -> ToolOutcome {
    ToolOutcome {
        identity: command_identity.clone(),
        failure: Some(format!("could not launch {}: {reason}", tool.tool_identity)),
        declaration: tool.clone(),
        command_identity,
        declared_tool_identities: vec![tool.tool_identity.clone()],
        process_id: 0,
        exit_code: None,
        timed_out: false,
        successful: false,
    }
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
    let mut bytes = Vec::new();
    stream.read_to_end(&mut bytes)?;
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
