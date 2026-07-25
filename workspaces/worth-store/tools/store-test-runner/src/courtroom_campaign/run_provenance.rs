use std::path::{Path, PathBuf};

use worth_store::physical_runtime::PhysicalWorkRerunEvidence;

pub(super) fn rerun(
    courtroom: &str,
    target_root: Option<&Path>,
    mutant_report: &Path,
    report: &Path,
) -> Result<PhysicalWorkRerunEvidence, String> {
    let program = std::env::current_exe()
        .map_err(|error| format!("cannot locate courtroom runner for rerun: {error}"))?
        .canonicalize()
        .map_err(|error| format!("cannot canonicalize courtroom runner for rerun: {error}"))?;
    let mut arguments = vec![
        "courtrooms".to_owned(),
        "--courtroom".to_owned(),
        courtroom.to_owned(),
        "--mutant-report".to_owned(),
        absolute(mutant_report)?.display().to_string(),
        "--report".to_owned(),
        absolute(report)?.display().to_string(),
    ];
    if let Some(target_root) = target_root {
        arguments.extend([
            "--target-root".to_owned(),
            absolute(target_root)?.display().to_string(),
        ]);
    }
    PhysicalWorkRerunEvidence::new(program.display().to_string(), arguments)
        .map_err(|denial| format!("courtroom rerun evidence denied: {denial:?}"))
}

fn absolute(path: &Path) -> Result<PathBuf, String> {
    if path.is_absolute() {
        return Ok(path.to_path_buf());
    }
    std::env::current_dir()
        .map(|current| current.join(path))
        .map_err(|error| format!("cannot resolve courtroom rerun path: {error}"))
}
