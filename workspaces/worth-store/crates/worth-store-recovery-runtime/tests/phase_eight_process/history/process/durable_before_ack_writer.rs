use std::path::Path;

use super::super::schedule::create_durable_before_ack_operation_program_with_operation_count;
use super::lifecycle::{launch, KilledProductionWriter, WriterLaunch};

pub(crate) fn launch_killed_durable_unacknowledged_writer_with_operation_count(
    parent: &Path,
    seed: u64,
    operation_count: usize,
) -> Result<KilledProductionWriter, String> {
    let operation_program = create_durable_before_ack_operation_program_with_operation_count(
        parent,
        seed,
        seed,
        operation_count,
    )?;
    launch(WriterLaunch {
        root: parent.join("durable-unacknowledged-writer-root"),
        stage: format!("candidate-publication@{seed}"),
        operation_program,
        start: parent.join("durable-unacknowledged-writer-start"),
        reached: parent.join("durable-unacknowledged-writer-reached"),
        durable_before_ack: true,
        capture_after_recovery: true,
    })
}
