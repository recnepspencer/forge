use super::{UiInteractionStateSnapshot, UiLocalInputStop, UiPointerGestureStop};

/// Exact terminal evidence emitted when a lifecycle boundary settles every
/// interaction-owned resource family.
#[derive(Debug, Eq, PartialEq)]
pub struct UiInteractionLifecycleSettlementReceipt {
    pointer_stops: Box<[UiPointerGestureStop]>,
    local_input_stops: Box<[UiLocalInputStop]>,
    final_state: UiInteractionStateSnapshot,
}

impl UiInteractionLifecycleSettlementReceipt {
    pub(super) fn new(
        pointer_stops: Vec<UiPointerGestureStop>,
        local_input_stops: Vec<UiLocalInputStop>,
        final_state: UiInteractionStateSnapshot,
    ) -> Self {
        Self {
            pointer_stops: pointer_stops.into_boxed_slice(),
            local_input_stops: local_input_stops.into_boxed_slice(),
            final_state,
        }
    }

    pub fn stops(&self) -> &[UiPointerGestureStop] {
        &self.pointer_stops
    }

    pub fn local_input_stops(&self) -> &[UiLocalInputStop] {
        &self.local_input_stops
    }

    pub const fn final_state(&self) -> UiInteractionStateSnapshot {
        self.final_state
    }

    pub fn settled_gestures(&self) -> usize {
        self.pointer_stops.len()
    }

    pub fn settled_local_recipients(&self) -> usize {
        self.local_input_stops
            .iter()
            .filter(|stop| stop.settled_recipient())
            .count()
    }

    pub fn settled_draft_sessions(&self) -> usize {
        self.local_input_stops
            .iter()
            .filter(|stop| stop.settled_session())
            .count()
    }
}
