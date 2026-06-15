#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum WorthUiDurableStateIneligibilityReason {
    NoDurableStateSurface,
    NoRestorableStateSlots,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum WorthUiDurableStateEligibility {
    Ineligible {
        reason: WorthUiDurableStateIneligibilityReason,
    },
    Eligible {
        restorable_state_slot_count: usize,
    },
}
