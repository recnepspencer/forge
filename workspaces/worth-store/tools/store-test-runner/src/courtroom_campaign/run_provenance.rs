use std::path::{Path, PathBuf};

use worth_store::physical_runtime::PhysicalWorkRerunEvidence;

pub(super) struct CourtroomRerunRequest<'path> {
    pub(super) courtroom: &'path str,
    pub(super) target_root: Option<&'path Path>,
    pub(super) controlled_case_report: &'path Path,
    pub(super) report: &'path Path,
    pub(super) schedule_seed: Option<u64>,
    pub(super) termination_point: Option<&'path str>,
}

pub(super) fn rerun(
    request: CourtroomRerunRequest<'_>,
) -> Result<PhysicalWorkRerunEvidence, String> {
    let program = std::env::current_exe()
        .map_err(|error| format!("cannot locate courtroom runner for rerun: {error}"))?
        .canonicalize()
        .map_err(|error| format!("cannot canonicalize courtroom runner for rerun: {error}"))?;
    let mut arguments = vec![
        "courtrooms".to_owned(),
        "--courtroom".to_owned(),
        request.courtroom.to_owned(),
        "--mutant-report".to_owned(),
        absolute(request.controlled_case_report)?
            .display()
            .to_string(),
        "--report".to_owned(),
        absolute(request.report)?.display().to_string(),
    ];
    if let Some(target_root) = request.target_root {
        arguments.extend([
            "--target-root".to_owned(),
            absolute(target_root)?.display().to_string(),
        ]);
    }
    if let Some(schedule_seed) = request.schedule_seed {
        arguments.extend(["--schedule-seed".to_owned(), schedule_seed.to_string()]);
    }
    if let Some(termination_point) = request.termination_point {
        arguments.extend(["--crash-seam".to_owned(), termination_point.to_owned()]);
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

pub(super) fn display(rerun: &PhysicalWorkRerunEvidence) -> String {
    let mut command = std::process::Command::new(rerun.program());
    command.args(rerun.arguments().iter().map(AsRef::as_ref));
    format!("{command:?}")
}
