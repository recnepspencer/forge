use std::panic::{catch_unwind, AssertUnwindSafe};
use std::path::Path;
use std::process::{Command, Output, Stdio};
use std::time::Duration;

use worth_store_offline_verifier::RecoveryObserverReport;
use worth_store_process_bundle::{
    FreshRecoveryProcessBundle, FINALIZED_OBSERVER_ENV, FINALIZED_RECOVERY_ENV,
    FINALIZED_WRITER_ENV,
};
use worth_store_recovery_runtime::{RecoveryReportEnvelope, RecoveryReportOutcome};

#[allow(dead_code)]
#[path = "../phase_eight_process/child_lifecycle.rs"]
mod child_lifecycle;
#[path = "../phase_eight_process/process_lane.rs"]
mod process_lane;
// The documented-process proof reuses the complete process-history fixture, while
// the Phase 8 process binary exercises the rest of that fixture's observers and
// comparisons. Keep the shared fixture single-sourced without pretending that
// ledger-only compilation uses every process-lane helper.
#[allow(dead_code, unused_imports)]
#[path = "../phase_eight_process/history.rs"]
mod process_history;
#[allow(dead_code, unused_imports)]
#[path = "../phase_eight_process/support_binaries.rs"]
mod support_binaries;

const OPERATOR_ROOT: &str = "D:\\stores\\orders";
const REPORT_ROOT: &str = "D:\\reports";

pub(super) fn execute(repository_root: &Path) {
    let workspace = repository_root.join("workspaces/worth-store");
    let process_lane = process_lane::acquire().expect("acquire documented process lane");
    assert!(!process_lane::lane_name().is_empty());
    let finalized =
        FreshRecoveryProcessBundle::build_production_finalized(&workspace, repository_root)
            .expect("build documented process bundle");
    let environment = InstalledBundleEnvironment::install(finalized.bundle());
    let proof = catch_unwind(AssertUnwindSafe(|| {
        execute_owned(repository_root, finalized.bundle(), &process_lane)
    }))
    .map_err(panic_message);
    let environment_result = environment.close();
    let bundle_result = finalized.finish(proof);
    let lane_result = process_lane.close();
    let errors = [environment_result, bundle_result, lane_result]
        .into_iter()
        .filter_map(Result::err)
        .collect::<Vec<_>>();
    if !errors.is_empty() {
        panic!("{}", errors.join("; "));
    }
}

struct InstalledBundleEnvironment {
    prior: [(&'static str, Option<std::ffi::OsString>); 3],
}

impl InstalledBundleEnvironment {
    fn install(bundle: &worth_store_process_bundle::FreshRecoveryProcessBundle) -> Self {
        let bindings = [
            (FINALIZED_WRITER_ENV, bundle.writer().path()),
            (FINALIZED_OBSERVER_ENV, bundle.observer().path()),
            (FINALIZED_RECOVERY_ENV, bundle.recovery().path()),
        ];
        let prior = bindings.map(|(name, path)| {
            let previous = std::env::var_os(name);
            std::env::set_var(name, path);
            (name, previous)
        });
        Self { prior }
    }

    fn close(self) -> Result<(), String> {
        for (name, value) in self.prior {
            match value {
                Some(value) => std::env::set_var(name, value),
                None => std::env::remove_var(name),
            }
        }
        Ok(())
    }
}

fn execute_owned(
    repository_root: &Path,
    binaries: &worth_store_process_bundle::FreshRecoveryProcessBundle,
    process_lane: &process_lane::ProcessLaneGuard,
) {
    let document = std::fs::read_to_string(
        repository_root.join("_docs/worth-store/physical-recovery-and-reopen.md"),
    )
    .expect("read operator recovery guide");
    let commands = extract_real_example_commands(&document);
    assert_eq!(commands.len(), 2, "the real example must have two commands");
    assert_eq!(commands[0].program, "physical_store_recover");
    assert_eq!(commands[1].program, "physical_store_offline_observer");

    let parent = tempfile::tempdir().expect("documented process fixture");
    let writer =
        process_history::launch_killed_production_writer(parent.path(), "candidate-publication", 0)
            .expect("documented process fixture must leave a real Store root");
    let report_root = parent.path().join("reports");
    std::fs::create_dir_all(&report_root).expect("documented report directory");
    for command in commands {
        let executable = match command.program.as_str() {
            "physical_store_recover" => binaries.recovery().path(),
            "physical_store_offline_observer" => binaries.observer().path(),
            program => panic!("unexpected documented program {program}"),
        };
        let arguments = command
            .arguments
            .iter()
            .map(|argument| substitute_paths(argument, &writer.root, &report_root))
            .collect::<Vec<_>>();
        let mut process = Command::new(executable);
        process
            .args(&arguments)
            .env("TMP", parent.path())
            .env("TEMP", parent.path())
            .env("TMPDIR", parent.path())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        let output = bounded_output(&mut process, Duration::from_secs(120))
            .expect("launch documented command");
        assert_success(&command.program, &output);
    }
    assert!(report_root.join("runtime-v1.bin").is_file());
    assert!(report_root.join("observer-v1.bin").is_file());
    let runtime = RecoveryReportEnvelope::decode(
        &std::fs::read(report_root.join("runtime-v1.bin")).expect("documented runtime report"),
    )
    .expect("decode documented runtime report");
    assert_eq!(runtime.outcome(), RecoveryReportOutcome::Recovered);
    assert!(runtime.store_identity().is_some());
    assert!(runtime.root_generation().is_some());
    assert!(runtime.counters().peak_recovery_bytes() > 0);
    let observer = RecoveryObserverReport::decode(
        &std::fs::read(report_root.join("observer-v1.bin")).expect("documented observer report"),
    )
    .expect("decode documented observer report");
    assert!(observer.artifact_count() > 0);
    assert!(observer.bytes_read() > 0);
    assert!(observer.selector_store_identity().is_some());
    assert!(observer.current_root_generation().is_some());
    process_lane.assert_within_budget("documented process proof");
}

#[derive(Debug, PartialEq, Eq)]
struct DocumentedCommand {
    program: String,
    arguments: Vec<String>,
}

fn extract_real_example_commands(document: &str) -> Vec<DocumentedCommand> {
    let start = document
        .find("## Real Example")
        .expect("operator guide real example heading")
        + "## Real Example".len();
    let end = document[start..]
        .find("## How It Relates To Other Features")
        .map(|offset| start + offset)
        .expect("operator guide real example boundary");
    let mut commands = Vec::new();
    let mut logical = String::new();
    let mut in_code_block = false;
    for line in document[start..end].lines() {
        let line = line.trim();
        if line == "```text" {
            in_code_block = true;
            continue;
        }
        if line == "```" {
            in_code_block = false;
            continue;
        }
        if !in_code_block || line.is_empty() {
            continue;
        }
        let continued = line.ends_with('\\');
        let fragment = line.strip_suffix('\\').unwrap_or(line).trim_end();
        if !logical.is_empty() {
            logical.push(' ');
        }
        logical.push_str(fragment);
        if !continued {
            let mut fields = logical.split_whitespace();
            let Some(program) = fields.next() else {
                logical.clear();
                continue;
            };
            commands.push(DocumentedCommand {
                program: program.to_owned(),
                arguments: fields.map(str::to_owned).collect(),
            });
            logical.clear();
        }
    }
    assert!(!in_code_block, "unterminated documented code block");
    assert!(logical.is_empty(), "unterminated documented command");
    commands
}

fn substitute_paths(argument: &str, store_root: &Path, report_root: &Path) -> String {
    argument
        .replace(OPERATOR_ROOT, &store_root.to_string_lossy())
        .replace(REPORT_ROOT, &report_root.to_string_lossy())
}

fn bounded_output(command: &mut Command, timeout: Duration) -> Result<Output, String> {
    let child = child_lifecycle::ProcessChildGuard::new(
        command
            .spawn()
            .map_err(|error| format!("spawn bounded documented process: {error}"))?,
    );
    child.wait_with_output_within(timeout)
}

fn assert_success(description: &str, output: &Output) {
    assert!(
        output.status.success(),
        "{description} failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );
}

fn panic_message(payload: Box<dyn std::any::Any + Send>) -> String {
    payload
        .downcast_ref::<String>()
        .cloned()
        .or_else(|| {
            payload
                .downcast_ref::<&str>()
                .map(|message| (*message).to_owned())
        })
        .unwrap_or_else(|| "documented process proof panicked".to_owned())
}
