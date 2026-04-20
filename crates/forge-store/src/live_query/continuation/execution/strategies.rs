use crate::backend::records::DurableCursorIdentityRecord;
use crate::failure::{StoreError, StoreErrorKind};
use crate::live_query::basis::{StableBasisHandle, StableBasisReadScope};
use crate::live_query::restart::StableBasisSurvival;
use forge_relational::facade::history::CommitId;

use super::core::{
    ContinuationExecutionEffect, ContinuationExecutionMetrics, ExecutedContinuationBatch,
};
use crate::live_query::continuation::batch_surface::{
    AdmittedNarrowBatchReceipt, BroadenedBatchReceipt, ContinuationBatchId,
    ContinuationBatchResult,
};

pub(crate) fn execute_admitted_batch(
    basis: &StableBasisHandle,
    identity: &DurableCursorIdentityRecord,
    batch_id: ContinuationBatchId,
    commit_ids: Vec<CommitId>,
    covered_commit_range: (CommitId, CommitId),
    from_frontier_commit_id: CommitId,
    covered_commit_count: u64,
    support_rows_read: u64,
    scope_lookup_count: u64,
) -> Result<ExecutedContinuationBatch, StoreError> {
    if !matches!(basis.read_scope(), StableBasisReadScope::SingleEntity(_)) {
        return Err(StoreError::new(
            StoreErrorKind::ContinuationScopeIncompatibility,
            "admitted continuation currently supports only single-entity stable-basis scopes",
        ));
    }
    Ok(ExecutedContinuationBatch::new(
        ContinuationBatchResult::AdmittedNarrow(AdmittedNarrowBatchReceipt::new(
            batch_id,
            basis.stable_basis_id().clone(),
            identity.cursor_id.clone(),
            identity.subscriber_id.clone(),
            identity.branch_id.clone(),
            identity.feed_shape_id.clone(),
            identity.schema_interpretation_id.clone(),
            identity.cursor_semantics_version,
            basis.schema_boundary_artifact_id().to_string(),
            covered_commit_range,
            commit_ids,
            from_frontier_commit_id,
            covered_commit_range.1,
            basis.read_scope().clone(),
            1,
            covered_commit_count,
            covered_commit_count,
            support_rows_read,
            scope_lookup_count,
        )),
        ContinuationExecutionMetrics {
            support_rows_read,
            narrowed_item_count: covered_commit_count,
            broadened_item_count: 0,
            step_count: covered_commit_count,
        },
        vec![ContinuationExecutionEffect::Batch],
    ))
}

pub(crate) fn execute_broadened_batch(
    basis: &StableBasisHandle,
    batch_id: ContinuationBatchId,
    commit_ids: Vec<CommitId>,
    covered_commit_range: (CommitId, CommitId),
    from_frontier_commit_id: CommitId,
    covered_commit_count: u64,
    support_rows_read: u64,
    scope_lookup_count: u64,
) -> Result<ExecutedContinuationBatch, StoreError> {
    let survival = StableBasisSurvival::from_handle(basis);
    let fallback_class = match &survival {
        StableBasisSurvival::DegradedButRecoverable { fallback_class } => fallback_class.as_str(),
        _ => "explicit_broadening",
    };
    Ok(ExecutedContinuationBatch::new(
        ContinuationBatchResult::Broadened(BroadenedBatchReceipt::new(
            batch_id,
            covered_commit_range,
            commit_ids,
            from_frontier_commit_id,
            covered_commit_range.1,
            basis.read_scope().clone(),
            1,
            covered_commit_count,
            covered_commit_count,
            support_rows_read,
            scope_lookup_count,
            fallback_class,
        )),
        ContinuationExecutionMetrics {
            support_rows_read,
            narrowed_item_count: 0,
            broadened_item_count: covered_commit_count,
            step_count: covered_commit_count,
        },
        vec![
            ContinuationExecutionEffect::Batch,
            ContinuationExecutionEffect::Broadening,
        ],
    ))
}
