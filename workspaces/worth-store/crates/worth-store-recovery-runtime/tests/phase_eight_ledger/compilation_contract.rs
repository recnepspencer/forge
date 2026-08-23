use std::path::Path;
use std::process::{Command, Stdio};
use std::time::Duration;

use worth_store_process_bundle::{target_parent, FreshProcessCargoTarget};

#[allow(dead_code)]
#[path = "../phase_eight_process/child_lifecycle.rs"]
mod child_lifecycle;

pub(super) fn execute(repository_root: &Path) {
    let workspace = repository_root.join("workspaces/worth-store");
    let target = FreshProcessCargoTarget::allocate(&target_parent(&workspace))
        .expect("allocate Phase 8 compilation-contract target");
    let checks = run_checks(&workspace, target.path());
    let close = target.close();
    match (checks, close) {
        (Ok(()), Ok(())) => {}
        (Err(error), Ok(())) | (Ok(()), Err(error)) => panic!("{error}"),
        (Err(checks), Err(close)) => {
            panic!("{checks}; compilation-contract target cleanup failed: {close}")
        }
    }
}

fn run_checks(workspace: &Path, target: &Path) -> Result<(), String> {
    let cargo = std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into());
    for package in [
        "worth-store-recovery-runtime",
        "worth-store-offline-verifier",
        "worth-store",
        "worth-store-operations",
    ] {
        let description = format!("warnings-denied all-target/all-feature check for {package}");
        let mut command = Command::new(&cargo);
        command
            .current_dir(workspace)
            .args(["check", "-p", package, "--all-targets", "--all-features"])
            .env("CARGO_TARGET_DIR", target)
            .env("RUSTFLAGS", "-D warnings")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        let child = command
            .spawn()
            .map(child_lifecycle::ProcessChildGuard::new)
            .map_err(|error| format!("spawn {description}: {error}"))?;
        let output = child.wait_with_output_within(Duration::from_secs(600))?;
        if !output.status.success() {
            return Err(format!(
                "{description} failed\nstdout:\n{}\nstderr:\n{}",
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr),
            ));
        }
    }
    Ok(())
}
