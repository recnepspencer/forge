use super::LayoutPlanBudgetScope;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutPlanBudgetDenial {
    ScopeMismatch {
        requested: LayoutPlanBudgetScope,
        admitted: LayoutPlanBudgetScope,
    },
    PageReadsExceeded {
        planned: u32,
        admitted: u32,
    },
    ByteReadsExceeded {
        planned: u64,
        admitted: u64,
    },
    AllocationsExceeded {
        planned: u32,
        admitted: u32,
    },
}
