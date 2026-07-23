use crate::runtime::{
    WorthUiDurableStateReconciliationCounters, WorthUiDurableStateReconciliationOutcome,
    WorthUiNodeLifecycleTransition,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiSelectionRangeReconciliation;

impl WorthUiSelectionRangeReconciliation {
    pub(crate) fn allows_carry_for_transition(transition: WorthUiNodeLifecycleTransition) -> bool {
        matches!(
            transition,
            WorthUiNodeLifecycleTransition::Preserve | WorthUiNodeLifecycleTransition::Move
        )
    }

    pub(crate) fn replacement_outcome(
        transition: WorthUiNodeLifecycleTransition,
        counters: &mut WorthUiDurableStateReconciliationCounters,
    ) -> Option<(WorthUiDurableStateReconciliationOutcome, &'static str)> {
        if matches!(
            transition,
            WorthUiNodeLifecycleTransition::Rebind | WorthUiNodeLifecycleTransition::Replace
        ) {
            counters.record_query_posture_required();
            Some((
                WorthUiDurableStateReconciliationOutcome::Drop,
                "selection range requires admitted backing collection identity",
            ))
        } else {
            None
        }
    }
}
