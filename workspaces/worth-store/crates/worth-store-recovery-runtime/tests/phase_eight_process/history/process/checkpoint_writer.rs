use std::path::Path;

use super::super::schedule::{
    create_checkpoint_operation_program, create_checkpoint_operation_program_with_operation_count,
};
use super::lifecycle::{launch, KilledProductionWriter, WriterLaunch};

pub(crate) fn launch_killed_production_writer(
    parent: &Path,
    stage: &str,
    seed: u64,
) -> Result<KilledProductionWriter, String> {
    launch_killed_production_writer_with_operation_count(
        parent,
        stage,
        seed,
        super::super::DEFAULT_OPERATION_COUNT,
    )
}

pub(crate) fn launch_killed_production_writer_with_operation_count(
    parent: &Path,
    stage: &str,
    seed: u64,
    operation_count: usize,
) -> Result<KilledProductionWriter, String> {
    let operation_program = if operation_count == super::super::DEFAULT_OPERATION_COUNT {
        create_checkpoint_operation_program(parent, seed, seed)?
    } else {
        create_checkpoint_operation_program_with_operation_count(
            parent,
            seed,
            seed,
            operation_count,
        )?
    };
    launch(WriterLaunch {
        root: parent.join("production-writer-root"),
        stage: format!("{stage}@{seed}"),
        operation_program,
        start: parent.join("production-writer-start"),
        reached: parent.join("production-writer-reached"),
        durable_before_ack: false,
        capture_after_recovery: false,
    })
}

pub(crate) fn launch_killed_post_reclamation_writer(
    parent: &Path,
    seed: u64,
) -> Result<KilledProductionWriter, String> {
    let operation_program = create_checkpoint_operation_program(parent, seed, seed)?;
    launch(WriterLaunch {
        root: parent.join("production-writer-root"),
        stage: format!("namespace-synchronization-complete@{seed}"),
        operation_program,
        start: parent.join("production-writer-start"),
        reached: parent.join("production-writer-reached"),
        durable_before_ack: false,
        capture_after_recovery: false,
    })
}
