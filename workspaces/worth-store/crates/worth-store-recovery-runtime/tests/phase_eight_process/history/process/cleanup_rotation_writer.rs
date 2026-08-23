use std::path::Path;

use super::super::schedule::create_cleanup_rotation_operation_program_with_operation_count;
use super::lifecycle::{launch, KilledProductionWriter, WriterLaunch};

pub(crate) fn launch_killed_cleanup_writer_with_operation_count(
    parent: &Path,
    seed: u64,
    operation_count: usize,
) -> Result<KilledProductionWriter, String> {
    let operation_program = create_cleanup_rotation_operation_program_with_operation_count(
        parent,
        seed,
        seed,
        operation_count,
    )?;
    launch(WriterLaunch {
        root: parent.join("cleanup-rotation-writer-root"),
        stage: format!("candidate-publication@{seed}"),
        operation_program,
        start: parent.join("cleanup-rotation-writer-start"),
        reached: parent.join("cleanup-rotation-writer-reached"),
        durable_before_ack: true,
        capture_after_recovery: true,
    })
}
