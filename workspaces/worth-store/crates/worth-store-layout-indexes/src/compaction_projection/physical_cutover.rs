use worth_store_physical_isolation::{
    CompactionCutoverState, CompactionOwnerCaseId, CompactionOwnerCaseObservation,
};

pub const fn observe_physical_cutover(
    owner_case: &CompactionOwnerCaseObservation,
) -> (
    CompactionCutoverState,
    CompactionOwnerCaseId,
    CompactionCutoverState,
) {
    (owner_case.from(), owner_case.id(), owner_case.to())
}
