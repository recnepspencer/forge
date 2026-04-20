use crate::failure::{StoreError, StoreErrorKind};
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct FetchWidth(u32);

impl FetchWidth {
    pub fn new(value: u32) -> Self {
        Self(value)
    }

    pub fn get(self) -> u32 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct MaxBatchItems(u32);

impl MaxBatchItems {
    pub fn new(value: u32) -> Self {
        Self(value)
    }

    pub fn get(self) -> u32 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct MaxCoveredCommits(u32);

impl MaxCoveredCommits {
    pub fn new(value: u32) -> Self {
        Self(value)
    }

    pub fn get(self) -> u32 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct MaxMaterializedBytes(u64);

impl MaxMaterializedBytes {
    pub fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn get(self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct MaxSupportRowsPerBatch(u32);

impl MaxSupportRowsPerBatch {
    pub fn new(value: u32) -> Self {
        Self(value)
    }

    pub fn get(self) -> u32 {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ContinuationBatchBudget {
    fetch_width: FetchWidth,
    max_batch_items: MaxBatchItems,
    max_covered_commits: MaxCoveredCommits,
    max_materialized_bytes: MaxMaterializedBytes,
    max_support_rows_per_batch: MaxSupportRowsPerBatch,
}

impl ContinuationBatchBudget {
    pub fn new(
        fetch_width: FetchWidth,
        max_batch_items: MaxBatchItems,
        max_covered_commits: MaxCoveredCommits,
        max_materialized_bytes: MaxMaterializedBytes,
        max_support_rows_per_batch: MaxSupportRowsPerBatch,
    ) -> Self {
        Self {
            fetch_width,
            max_batch_items,
            max_covered_commits,
            max_materialized_bytes,
            max_support_rows_per_batch,
        }
    }

    pub fn fetch_width(&self) -> FetchWidth {
        self.fetch_width
    }

    pub fn max_batch_items(&self) -> MaxBatchItems {
        self.max_batch_items
    }

    pub fn max_covered_commits(&self) -> MaxCoveredCommits {
        self.max_covered_commits
    }

    pub fn max_materialized_bytes(&self) -> MaxMaterializedBytes {
        self.max_materialized_bytes
    }

    pub fn max_support_rows_per_batch(&self) -> MaxSupportRowsPerBatch {
        self.max_support_rows_per_batch
    }
}

pub(crate) fn continuation_batch_limit(
    budget: &ContinuationBatchBudget,
) -> Result<usize, StoreError> {
    let limit = budget
        .fetch_width()
        .get()
        .min(budget.max_batch_items().get())
        .min(budget.max_covered_commits().get())
        .min(budget.max_support_rows_per_batch().get());
    if limit == 0 {
        return Err(StoreError::new(
            StoreErrorKind::ContinuationCursorIncompatibility,
            "continuation batch budgets must admit at least one covered commit",
        ));
    }
    Ok(limit as usize)
}
