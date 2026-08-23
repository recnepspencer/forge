use std::path::{Path, PathBuf};
use std::process::Command;

use super::super::super::child_lifecycle::ProcessChildGuard;
use super::super::history_io::{c8_writer_binary_path, wait_for_marker};
use super::super::{ExpectedWriterHistory, ParentPhysicalHistory, SubmittedOperationProgram};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct KilledProductionWriter {
    pub(crate) root: PathBuf,
    pub(crate) history: ParentPhysicalHistory,
    pub(crate) expected: ExpectedWriterHistory,
    pub(crate) process_id: u32,
    pub(crate) runtime_identity: u64,
}

pub(crate) struct WriterLaunch {
    pub(crate) root: PathBuf,
    pub(crate) stage: String,
    pub(crate) operation_program: SubmittedOperationProgram,
    pub(crate) start: PathBuf,
    pub(crate) reached: PathBuf,
    pub(crate) durable_before_ack: bool,
    pub(crate) capture_after_recovery: bool,
}

pub(crate) fn launch(mut declaration: WriterLaunch) -> Result<KilledProductionWriter, String> {
    let barrier_receipt = declaration.operation_program.barrier_receipt.clone();
    let mut child = ProcessChildGuard::new(
        Command::new(c8_writer_binary_path())
            .args(["--root"])
            .arg(&declaration.root)
            .args(["--start-marker"])
            .arg(&declaration.start)
            .args(["--reached-marker"])
            .arg(&declaration.reached)
            .args(["--checkpoint-stage"])
            .arg(&declaration.stage)
            .args(["--writer-durability-profile"])
            .arg(
                declaration
                    .operation_program
                    .writer_profile_selection
                    .cli_name(),
            )
            .args(["--operation-program"])
            .arg(&declaration.operation_program.path)
            .args(["--identity-receipt"])
            .arg(&declaration.operation_program.identity_receipt)
            .args(["--barrier-receipt"])
            .arg(&barrier_receipt)
            .args(
                declaration
                    .durable_before_ack
                    .then_some("--durable-before-ack"),
            )
            .spawn()
            .map_err(|error| format!("spawn {}: {error}", declaration.stage))?,
    );
    let process_id = child.id();
    wait_for_marker(
        &mut child,
        &declaration.start.with_extension("ready"),
        "writer ready",
    )?;
    let runtime_identity = read_runtime_identity(&declaration.start)?;
    declaration
        .operation_program
        .expected
        .bind_identity_receipt(&declaration.operation_program.identity_receipt)?;
    declaration
        .operation_program
        .expected
        .bind_checkpoint_redo_digests(&declaration.root)?;
    std::fs::write(&declaration.start, b"release")
        .map_err(|error| format!("release {}: {error}", declaration.stage))?;
    wait_for_marker(&mut child, &declaration.reached, "writer effect")?;
    let status = child
        .kill_and_wait()
        .map_err(|error| format!("wait for {}: {error}", declaration.stage))?;
    if status.success() {
        return Err(format!(
            "{} unexpectedly exited successfully",
            declaration.stage
        ));
    }
    let history = if declaration.capture_after_recovery {
        ParentPhysicalHistory::capture_after_recovery(
            &declaration.root,
            &declaration.operation_program.expected,
        )?
    } else {
        ParentPhysicalHistory::capture(&declaration.root, &declaration.operation_program.expected)?
    };
    Ok(KilledProductionWriter {
        root: declaration.root,
        history,
        expected: declaration.operation_program.expected,
        process_id,
        runtime_identity,
    })
}

fn read_runtime_identity(start: &Path) -> Result<u64, String> {
    let path = start.with_extension("runtime");
    std::fs::read_to_string(&path)
        .map_err(|error| format!("read writer runtime identity {}: {error}", path.display()))?
        .trim()
        .parse()
        .map_err(|error| format!("parse writer runtime identity {}: {error}", path.display()))
}
