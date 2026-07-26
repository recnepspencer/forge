use worth_query_execution::facade::domain_computation::{
    WorthQueryGraphProviderCheckpoint, WorthQueryYieldedDirectRun,
};

fn mint_from_checkpoint(
    checkpoint: Box<dyn WorthQueryGraphProviderCheckpoint>,
) -> WorthQueryYieldedDirectRun {
    checkpoint.into()
}

fn main() {}
