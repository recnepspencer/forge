use std::collections::BTreeMap;

use forge_query::facade::{
    ForgeQueryBatchWriteReceipt, ForgeQueryBatchWriteReceiptInspection, ForgeQueryInspection,
    ForgeQueryMutationBatchBuilder,
};
use schema::facade::platform::entities::TopologyEntityKind;

use super::super::{
    load_post_write_materialized_topology, NamingEditContinuityMatrix, TopologyEditApplicationMode,
    TopologyEditContract, TopologyEditDigest, TopologyEditFamily, TopologyEditNamingReport,
    TopologyOperatorExecution, TopologyOperatorExecutionError, TopologyOperatorExecutionPath,
    TopologyOperatorRunner, TopologyQueryBindingIndex,
};

pub(super) fn lower_contracts(
    runner: &TopologyOperatorRunner<'_, '_>,
    contracts: &[TopologyEditContract],
    bindings: &TopologyQueryBindingIndex,
    created_entity_kinds: &BTreeMap<String, TopologyEntityKind>,
) -> Result<ForgeQueryMutationBatchBuilder, TopologyOperatorExecutionError> {
    contracts.iter().try_fold(
        ForgeQueryMutationBatchBuilder::new(),
        |builder, contract| {
            runner.lower_contract(builder, bindings, created_entity_kinds, contract)
        },
    )
}

pub(super) fn finalize_lowered_batch(
    runner: &mut TopologyOperatorRunner<'_, '_>,
    lowered_batch: ForgeQueryMutationBatchBuilder,
    path: TopologyOperatorExecutionPath,
    mode: TopologyEditApplicationMode,
    families: Vec<TopologyEditFamily>,
    topology_edit_digest: TopologyEditDigest,
    naming_continuity_matrix: NamingEditContinuityMatrix,
    naming_report: TopologyEditNamingReport,
) -> Result<TopologyOperatorExecution, TopologyOperatorExecutionError> {
    let receipt = runner.workspace.batch(|_| lowered_batch)?;
    let inspection = load_batch_write_receipt_inspection(runner, &receipt)?;
    let materialized = load_post_write_materialized_topology(runner.workspace, runner.surfaces)?;
    Ok(TopologyOperatorExecution {
        mode,
        path,
        families,
        receipt,
        inspection,
        materialized,
        topology_edit_digest,
        naming_continuity_matrix,
        naming_report,
    })
}

fn load_batch_write_receipt_inspection(
    runner: &mut TopologyOperatorRunner<'_, '_>,
    receipt: &ForgeQueryBatchWriteReceipt,
) -> Result<ForgeQueryBatchWriteReceiptInspection, TopologyOperatorExecutionError> {
    match runner.workspace.inspect(receipt)? {
        ForgeQueryInspection::BatchWriteReceipt(inspection) => Ok(inspection),
        _ => Err(TopologyOperatorExecutionError::UnexpectedInspectionFamily),
    }
}
