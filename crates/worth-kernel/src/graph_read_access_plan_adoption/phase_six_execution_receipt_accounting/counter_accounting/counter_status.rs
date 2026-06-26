#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthGraphReadAccessCounterAccountingStatus {
    QueryCountersAccounted,
    CounterGapRequiresQueryReceiptSurface,
    NoExecutionCountersRequired,
}

impl WorthGraphReadAccessCounterAccountingStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::QueryCountersAccounted => "query_counters_accounted",
            Self::CounterGapRequiresQueryReceiptSurface => {
                "counter_gap_requires_query_receipt_surface"
            }
            Self::NoExecutionCountersRequired => "no_execution_counters_required",
        }
    }

    pub const fn is_accounted_or_explicit_gap(self) -> bool {
        matches!(
            self,
            Self::QueryCountersAccounted
                | Self::CounterGapRequiresQueryReceiptSurface
                | Self::NoExecutionCountersRequired
        )
    }
}
