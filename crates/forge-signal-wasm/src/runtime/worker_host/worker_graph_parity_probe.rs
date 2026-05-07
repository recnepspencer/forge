use crate::boundary::errors::ForgeSignalJsError;
use crate::recipe::model::TransactionOp;
use crate::runtime::core::RuntimeCore;

use super::{
    committed_truth_digest_for_runtime, publish_definition_envelope_into_worker_runtime,
    WorkerCompatibilityTruthReport, WorkerPortableGraphPublication, WorkerRuntimeShell,
};

pub fn probe_worker_graph_committed_truth_parity(
    publication: WorkerPortableGraphPublication,
    transaction_ops: Vec<TransactionOp>,
) -> Result<WorkerCompatibilityTruthReport, ForgeSignalJsError> {
    let worker_envelope =
        run_worker_graph_publication(publication.clone(), transaction_ops.clone())?;
    let compatibility_digest = run_compatibility_graph_publication(publication, transaction_ops)?;

    Ok(WorkerCompatibilityTruthReport::compare(
        &worker_envelope,
        compatibility_digest,
    ))
}

fn run_worker_graph_publication(
    publication: WorkerPortableGraphPublication,
    transaction_ops: Vec<TransactionOp>,
) -> Result<super::WorkerCommittedTransactionEnvelope, ForgeSignalJsError> {
    let mut worker_shell = WorkerRuntimeShell::new(publication.policy.clone())?;
    worker_shell.publish_graph(publication)?;
    worker_shell.apply_committed_transaction(transaction_ops)
}

fn run_compatibility_graph_publication(
    publication: WorkerPortableGraphPublication,
    transaction_ops: Vec<TransactionOp>,
) -> Result<String, ForgeSignalJsError> {
    let mut compatibility_runtime = RuntimeCore::new(publication.policy.clone())?;
    publish_definition_envelope_into_worker_runtime(
        &mut compatibility_runtime,
        publication.into_definition_envelope(),
    )?;
    compatibility_runtime.apply_transaction(transaction_ops)?;
    committed_truth_digest_for_runtime(&compatibility_runtime)
}
