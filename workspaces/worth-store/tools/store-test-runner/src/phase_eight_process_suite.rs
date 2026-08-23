use std::path::Path;
use std::process::Command;
use std::time::Duration;

use worth_store_process_bundle::FreshRecoveryProcessBundle;

#[path = "phase_eight_process_suite/child.rs"]
mod child;

pub(super) fn run(workspace: &Path, harness_arguments: &[String]) -> Result<(), String> {
    let repository = workspace
        .parent()
        .and_then(Path::parent)
        .ok_or_else(|| "Store workspace omitted repository ancestors".to_owned())?;
    let finalized = FreshRecoveryProcessBundle::build_production_finalized(workspace, repository)
        .map_err(|error| format!("build Phase 8 suite process bundle: {error}"))?;
    let mut command = Command::new(std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into()));
    command.current_dir(workspace).args([
        "test",
        "-j",
        "1",
        "-p",
        "worth-store-recovery-runtime",
        "--test",
        "phase_eight_process",
        "--features",
        "certification-test-authority",
    ]);
    if !harness_arguments.is_empty() {
        command.arg("--").args(harness_arguments);
    }
    finalized.install_environment(&mut command);
    let suite_result =
        child::run_within(&mut command, Duration::from_secs(60 * 60)).and_then(|status| {
            status
                .success()
                .then_some(())
                .ok_or_else(|| format!("Phase 8 process suite exited with {status}"))
        });
    finalized.finish(suite_result)
}
