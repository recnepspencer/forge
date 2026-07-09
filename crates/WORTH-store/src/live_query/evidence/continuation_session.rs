use crate::failure::{StoreError, StoreErrorKind};
use crate::live_query::basis::StableBasisReadScope;
use crate::live_query::continuation::{ContinuationBatchResult, ContinuationStrategy};
use worth_relational::facade::history::CommitId;
use serde::Serialize;

use super::basis::LiveQueryComplexityStatus;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct LiveQueryContinuationSessionEvidence {
    pub resolved_strategy: ContinuationStrategy,
    pub resolved_scope_fingerprint: String,
    pub final_frontier_commit_id: CommitId,
    pub batch_count: u64,
    pub covered_commit_ids: Vec<CommitId>,
    pub covered_commit_count: u64,
    pub narrowed_item_count: u64,
    pub broadened_item_count: u64,
    pub support_rows_read: u64,
    pub scope_lookup_count: u64,
    pub fallback_classes: Vec<String>,
    pub complexity_status: LiveQueryComplexityStatus,
}

impl LiveQueryContinuationSessionEvidence {
    pub fn from_batch_results(
        resolved_strategy: ContinuationStrategy,
        resolved_scope: &StableBasisReadScope,
        final_frontier_commit_id: CommitId,
        results: &[ContinuationBatchResult],
    ) -> Result<Self, StoreError> {
        validate_batch_result_metadata(resolved_strategy, resolved_scope, results)?;
        let mut batch_count = 0_u64;
        let mut covered_commit_ids = Vec::new();
        let mut covered_commit_count = 0_u64;
        let mut narrowed_item_count = 0_u64;
        let mut broadened_item_count = 0_u64;
        let mut support_rows_read = 0_u64;
        let mut scope_lookup_count = 0_u64;
        let mut fallback_classes = Vec::new();
        let mut complexity_status = LiveQueryComplexityStatus::Verified;

        for result in results {
            if !matches!(result, ContinuationBatchResult::CaughtUp(_)) {
                batch_count += 1;
            }
            covered_commit_ids.extend(result.covered_commit_ids().iter().copied());
            covered_commit_count += result.covered_commit_count();
            narrowed_item_count += result.narrowed_item_count();
            broadened_item_count += result.broadened_item_count();
            support_rows_read += result.support_rows_read();
            scope_lookup_count += result.scope_lookup_count();
            if let Some(fallback_class) = result.fallback_class() {
                fallback_classes.push(fallback_class.to_string());
            }
            if result.complexity_status() == LiveQueryComplexityStatus::Debt {
                complexity_status = LiveQueryComplexityStatus::Debt;
            }
        }

        Ok(Self {
            resolved_strategy,
            resolved_scope_fingerprint: resolved_scope.fingerprint(),
            final_frontier_commit_id,
            batch_count,
            covered_commit_ids,
            covered_commit_count,
            narrowed_item_count,
            broadened_item_count,
            support_rows_read,
            scope_lookup_count,
            fallback_classes,
            complexity_status,
        })
    }
}

fn validate_batch_result_metadata(
    resolved_strategy: ContinuationStrategy,
    resolved_scope: &StableBasisReadScope,
    results: &[ContinuationBatchResult],
) -> Result<(), StoreError> {
    let expected_scope_fingerprint = resolved_scope.fingerprint();
    let observed_strategy = dominant_session_strategy(results);
    if let Some(observed_strategy) = observed_strategy {
        if observed_strategy != resolved_strategy {
            return Err(StoreError::new(
                StoreErrorKind::ContinuationCursorIncompatibility,
                format!(
                    "milestone 8 certification request claimed resolved strategy `{:?}` but the batch receipts required `{:?}`",
                    resolved_strategy, observed_strategy
                ),
            ));
        }
    }
    for result in results {
        if result.resolved_scope().fingerprint() != expected_scope_fingerprint {
            return Err(StoreError::new(
                StoreErrorKind::ContinuationScopeIncompatibility,
                format!(
                    "milestone 8 certification request claimed resolved scope `{}` but included receipt scope `{}`",
                    expected_scope_fingerprint,
                    result.resolved_scope().fingerprint()
                ),
            ));
        }
    }
    Ok(())
}

fn dominant_session_strategy(results: &[ContinuationBatchResult]) -> Option<ContinuationStrategy> {
    results
        .iter()
        .map(ContinuationBatchResult::resolved_strategy)
        .max_by_key(|strategy| match strategy {
            ContinuationStrategy::AdmittedLayoutNarrow => 0_u8,
            ContinuationStrategy::ExplicitBroadened => 1_u8,
            ContinuationStrategy::AuthorityReplayControlLane => 2_u8,
        })
}
