use crate::AllocationScope;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AllocationBudgetDenial {
    AllocationBudgetIsZero,
    FixedMetadataReservationIsZero,
    MissingScopeBudget(AllocationScope),
    MissingFixedMetadataReservation,
}
