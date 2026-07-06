use crate::runtime::WorthUiNodeLifecycleTransition;
use crate::runtime::{
    WorthUiDurableStateReconciliationCounters, WorthUiDurableStateReconciliationOutcome,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiSplitterPositionReconciliation {
    reconciles_on_stable_identity: bool,
}

impl WorthUiSplitterPositionReconciliation {
    pub fn stable_identity() -> Self {
        Self {
            reconciles_on_stable_identity: true,
        }
    }

    pub fn reconciles_on_stable_identity(&self) -> bool {
        self.reconciles_on_stable_identity
    }

    pub(crate) fn allows_carry_for_transition(transition: WorthUiNodeLifecycleTransition) -> bool {
        matches!(
            transition,
            WorthUiNodeLifecycleTransition::Preserve
                | WorthUiNodeLifecycleTransition::Move
                | WorthUiNodeLifecycleTransition::Rebind
        )
    }

    pub(crate) fn replacement_outcome(
        transition: WorthUiNodeLifecycleTransition,
        counters: &mut WorthUiDurableStateReconciliationCounters,
    ) -> Option<(WorthUiDurableStateReconciliationOutcome, &'static str)> {
        match transition {
            WorthUiNodeLifecycleTransition::Create => Some((
                WorthUiDurableStateReconciliationOutcome::Recreate,
                "created splitter begins with fresh durable resize state",
            )),
            WorthUiNodeLifecycleTransition::LaneChange => Some((
                WorthUiDurableStateReconciliationOutcome::Recreate,
                "splitter position remapped explicitly for changed resize lane",
            )),
            WorthUiNodeLifecycleTransition::Preserve
            | WorthUiNodeLifecycleTransition::Move
            | WorthUiNodeLifecycleTransition::Rebind
            | WorthUiNodeLifecycleTransition::Replace => {
                counters.record_incompatible_shape();
                Some((
                    WorthUiDurableStateReconciliationOutcome::Drop,
                    "splitter position requires compatible sibling shape and resize contract",
                ))
            }
            WorthUiNodeLifecycleTransition::Drop => None,
        }
    }
}
