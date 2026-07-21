use crate::runtime::{
    WorthUiDurableStateReconciliationCounters, WorthUiDurableStateReconciliationOutcome,
    WorthUiNodeLifecycleTransition,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiTextEditStateReconciliation;

impl WorthUiTextEditStateReconciliation {
    pub(crate) fn allows_carry_for_transition(transition: WorthUiNodeLifecycleTransition) -> bool {
        matches!(transition, WorthUiNodeLifecycleTransition::Preserve)
    }

    pub(crate) fn replacement_outcome(
        transition: WorthUiNodeLifecycleTransition,
        counters: &mut WorthUiDurableStateReconciliationCounters,
    ) -> (WorthUiDurableStateReconciliationOutcome, &'static str) {
        if matches!(transition, WorthUiNodeLifecycleTransition::Create) {
            (
                WorthUiDurableStateReconciliationOutcome::Recreate,
                "created text input receives fresh edit state",
            )
        } else {
            counters.record_incompatible_shape();
            (
                WorthUiDurableStateReconciliationOutcome::Drop,
                "text edit state requires compatible component shape",
            )
        }
    }
}
