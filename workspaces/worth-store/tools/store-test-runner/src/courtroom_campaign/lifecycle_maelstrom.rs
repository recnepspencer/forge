use std::{
    path::{Path, PathBuf},
    process::Command,
    time::Duration,
};

use super::process_execution;

const TEST: &str = "physical_work::phase_16_lifecycle_maelstrom::\
    lifecycle_maelstrom_joins_real_authority_effects_and_shutdown";
const REPORT_ENV: &str = "WORTH_STORE_C5_1_COURTROOM_A_REPORT";
const MUTANT_REPORT_ENV: &str = "WORTH_STORE_C5_1_MUTANT_REPORT";
const RUNNER_ENV: &str = "WORTH_STORE_C5_1_COURTROOM_A_RUNNER";
const TIMEOUT: Duration = Duration::from_secs(60);

pub(super) fn run(workspace: &Path, mutant_report: &Path, report: &Path) -> Result<(), String> {
    let report = absolute(report)?;
    invalidate_prior_report(&report)?;
    let mutant_report = mutant_report
        .canonicalize()
        .map_err(|error| format!("cannot locate Courtroom A mutant report: {error}"))?;
    let runner = std::env::current_exe()
        .map_err(|error| format!("cannot locate Courtroom A runner: {error}"))?
        .canonicalize()
        .map_err(|error| format!("cannot canonicalize Courtroom A runner: {error}"))?;
    let mut command = Command::new(std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into()));
    command.current_dir(workspace).args([
        "test",
        "--quiet",
        "--package",
        "worth-store",
        "--features",
        "certification-test-authority",
        "--test",
        "physical_record_journeys",
        TEST,
        "--",
        "--exact",
        "--nocapture",
    ]);
    command
        .env(REPORT_ENV, &report)
        .env(MUTANT_REPORT_ENV, &mutant_report)
        .env(RUNNER_ENV, &runner);
    if let Err(failure) =
        process_execution::run_success(&mut command, TIMEOUT, "Courtroom A maelstrom")
    {
        let _ = std::fs::remove_file(&report);
        return Err(failure);
    }
    let metadata = report
        .metadata()
        .map_err(|error| format!("Courtroom A did not publish its report: {error}"))?;
    if !metadata.is_file() || metadata.len() == 0 {
        let _ = std::fs::remove_file(&report);
        return Err("Courtroom A published an empty or non-file report".into());
    }
    println!(
        "courtroom:a accepted the lifecycle maelstrom and published {} bytes to {}",
        metadata.len(),
        report.display()
    );
    Ok(())
}

fn invalidate_prior_report(report: &Path) -> Result<(), String> {
    match report.symlink_metadata() {
        Ok(metadata) if metadata.is_file() => std::fs::remove_file(report)
            .map_err(|error| format!("cannot invalidate prior Courtroom A report: {error}")),
        Ok(_) => Err(format!(
            "Courtroom A report path {} is not a file",
            report.display()
        )),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(format!("cannot inspect prior Courtroom A report: {error}")),
    }
}

fn absolute(path: &Path) -> Result<PathBuf, String> {
    if path.is_absolute() {
        return Ok(path.to_path_buf());
    }
    std::env::current_dir()
        .map(|current| current.join(path))
        .map_err(|error| format!("cannot resolve Courtroom A report path: {error}"))
}

#[cfg(test)]
mod tests {
    use super::invalidate_prior_report;

    #[test]
    fn stale_success_is_removed_but_a_non_file_target_is_rejected() {
        let root = tempfile::tempdir().unwrap();
        let report = root.path().join("courtroom-a.json");
        std::fs::write(&report, b"stale").unwrap();
        invalidate_prior_report(&report).unwrap();
        assert!(!report.exists());

        std::fs::create_dir(&report).unwrap();
        assert!(invalidate_prior_report(&report).is_err());
        assert!(report.is_dir());
    }
}
