use std::path::Path;

use worth_store_offline_verifier::{observe_recovery_artifacts, RecoveryObserverLimits};

pub(super) fn run(
    root: &Path,
    output: &Path,
    maximum_directory_entries: &str,
    maximum_directories: &str,
    maximum_artifacts: &str,
    maximum_bytes: &str,
) {
    let maximum_directory_entries = maximum_directory_entries
        .parse::<u64>()
        .expect("maximum-directory-entries must be a positive integer");
    let maximum_directories = maximum_directories
        .parse::<u64>()
        .expect("maximum-directories must be a positive integer");
    let maximum_artifacts = maximum_artifacts
        .parse::<u64>()
        .expect("maximum-artifacts must be a positive integer");
    let maximum_bytes = maximum_bytes
        .parse::<u64>()
        .expect("maximum-bytes must be a positive integer");
    let limits = RecoveryObserverLimits::new(
        maximum_directory_entries,
        maximum_directories,
        maximum_artifacts,
        maximum_bytes,
    )
    .expect("observer limits must be nonzero");
    let report = observe_recovery_artifacts(root, limits)
        .unwrap_or_else(|denial| panic!("recovery observation denied: {denial:?}"));
    std::fs::write(output, report.encode())
        .unwrap_or_else(|error| panic!("could not write recovery observer report: {error}"));
    eprintln!(
        "observed {} recovery artifacts and {} bytes; artifact set {}",
        report.artifact_count(),
        report.bytes_read(),
        super::hex(&report.artifact_set_digest())
    );
}
