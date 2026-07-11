use forge_store_physical_isolation::{
    CompactionCutoverState, CompactionCutoverTransition, CompactionCutoverTransitionKind,
};

pub const fn observe_physical_cutover(
    transition: &CompactionCutoverTransition,
) -> (
    CompactionCutoverState,
    CompactionCutoverTransitionKind,
    CompactionCutoverState,
) {
    (transition.from(), transition.kind(), transition.to())
}
