use crate::access::budget::S8PlannedCounterEnvelope;
use crate::access::shape::{S8AccessLaneClassification, S8AccessShape, S8AccessShapeContract};
use crate::strategy::S8LayoutStrategyFamily;
use forge_store_budgets::{
    S8PreExecutionBudgetRequest, S8PreExecutionBudgetScope, S8PreExecutionPlanBinding,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S8AccessPlanCostEstimate {
    estimated_memory_bytes: u64,
    estimated_page_reads: u16,
    estimated_chunk_reads: u16,
    estimated_range_touches: u16,
    estimated_byte_reads: u64,
}

impl S8AccessPlanCostEstimate {
    pub(crate) const fn new(
        estimated_memory_bytes: u64,
        estimated_page_reads: u16,
        estimated_chunk_reads: u16,
        estimated_range_touches: u16,
        estimated_byte_reads: u64,
    ) -> Self {
        Self {
            estimated_memory_bytes,
            estimated_page_reads,
            estimated_chunk_reads,
            estimated_range_touches,
            estimated_byte_reads,
        }
    }

    pub(crate) fn from_selected_plan(
        family: S8LayoutStrategyFamily,
        shape: S8AccessShapeContract,
        envelope: S8PlannedCounterEnvelope,
    ) -> Self {
        let profile = envelope.aggregate_profile();
        let page_reads = (profile.point_lookups()
            + profile.range_lookups()
            + profile.publications()
            + profile.maintenance_reads())
        .max(1);
        let chunk_reads = match shape.shape() {
            S8AccessShape::ChunkTreeWalk
            | S8AccessShape::StreamingRead
            | S8AccessShape::StreamingContinuationRead => match family {
                S8LayoutStrategyFamily::BaselineLsmWriteOptimized => profile.wal_replays().max(1),
                _ => 1,
            },
            _ => 0,
        };
        let range_touches = match shape.shape() {
            S8AccessShape::RangeLookup
            | S8AccessShape::MultiRangeLookup
            | S8AccessShape::PrefixLookup
            | S8AccessShape::GroupedPrefixLookup
            | S8AccessShape::BoundedScan
            | S8AccessShape::FullDeclaredScan
            | S8AccessShape::DegradedExactScan => shape
                .budget_rows()
                .unwrap_or(profile.range_lookups().max(1) as u64)
                .min(u16::MAX as u64) as u16,
            _ => profile.range_lookups().max(1),
        };
        let estimated_memory_bytes = (page_reads as u64 * 1_024)
            + (chunk_reads as u64 * 2_048)
            + (range_touches as u64 * 64);
        let estimated_byte_reads = (page_reads as u64 * 4_096)
            + (chunk_reads as u64 * 8_192)
            + (range_touches as u64 * 64);

        Self::new(
            estimated_memory_bytes,
            page_reads,
            chunk_reads,
            range_touches,
            estimated_byte_reads,
        )
    }

    pub(crate) const fn to_budget_request(
        self,
        plan_binding: S8PreExecutionPlanBinding,
        scope: S8PreExecutionBudgetScope,
    ) -> S8PreExecutionBudgetRequest {
        S8PreExecutionBudgetRequest::new(
            plan_binding,
            scope,
            self.estimated_memory_bytes,
            self.estimated_page_reads,
            self.estimated_chunk_reads,
            self.estimated_range_touches,
            self.estimated_byte_reads,
        )
    }

    pub const fn budget_scope_for(shape: S8AccessShapeContract) -> S8PreExecutionBudgetScope {
        match shape.shape() {
            S8AccessShape::ChunkTreeWalk
            | S8AccessShape::StreamingRead
            | S8AccessShape::StreamingContinuationRead => S8PreExecutionBudgetScope::Streaming,
            _ => match shape.lane() {
                S8AccessLaneClassification::Foreground => S8PreExecutionBudgetScope::Foreground,
                S8AccessLaneClassification::Maintenance => S8PreExecutionBudgetScope::Maintenance,
                S8AccessLaneClassification::Verifier => S8PreExecutionBudgetScope::Verifier,
                S8AccessLaneClassification::Terminal => S8PreExecutionBudgetScope::Terminal,
            },
        }
    }

    pub const fn estimated_memory_bytes(self) -> u64 {
        self.estimated_memory_bytes
    }

    pub const fn estimated_page_reads(self) -> u16 {
        self.estimated_page_reads
    }

    pub const fn estimated_chunk_reads(self) -> u16 {
        self.estimated_chunk_reads
    }

    pub const fn estimated_range_touches(self) -> u16 {
        self.estimated_range_touches
    }

    pub const fn estimated_byte_reads(self) -> u64 {
        self.estimated_byte_reads
    }
}
