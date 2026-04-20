#![allow(dead_code)]

use crate::RetentionClosureSummary;
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum RetainedReadPath {
    CanonicalRetainedAuthority,
    CompactionDerived,
    ExplicitFallback,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RetainedReadCostSurface {
    read_path: RetainedReadPath,
    closure_summary: RetentionClosureSummary,
    compacted_family_count: u64,
    rewritten_range_count: u64,
    reclaim_deletion_count: u64,
    live_basis_rejection_count: u64,
    rebuild_debt_delta: i64,
}

impl RetainedReadCostSurface {
    pub(crate) fn new(
        read_path: RetainedReadPath,
        closure_summary: RetentionClosureSummary,
        compacted_family_count: u64,
        rewritten_range_count: u64,
        reclaim_deletion_count: u64,
        live_basis_rejection_count: u64,
        rebuild_debt_delta: i64,
    ) -> Self {
        Self {
            read_path,
            closure_summary,
            compacted_family_count,
            rewritten_range_count,
            reclaim_deletion_count,
            live_basis_rejection_count,
            rebuild_debt_delta,
        }
    }

    pub fn read_path(&self) -> RetainedReadPath {
        self.read_path
    }

    pub fn closure_summary(&self) -> &RetentionClosureSummary {
        &self.closure_summary
    }

    pub fn compacted_family_count(&self) -> u64 {
        self.compacted_family_count
    }

    pub fn rewritten_range_count(&self) -> u64 {
        self.rewritten_range_count
    }

    pub fn reclaim_deletion_count(&self) -> u64 {
        self.reclaim_deletion_count
    }

    pub fn live_basis_rejection_count(&self) -> u64 {
        self.live_basis_rejection_count
    }

    pub fn rebuild_debt_delta(&self) -> i64 {
        self.rebuild_debt_delta
    }
}
