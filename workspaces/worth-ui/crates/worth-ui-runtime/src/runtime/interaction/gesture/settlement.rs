use super::{UiInteractionStateSnapshot, UiPointerGestureStop};

/// Exact terminal evidence emitted when a lifecycle boundary settles gestures.
#[derive(Debug, Eq, PartialEq)]
pub struct UiInteractionLifecycleSettlementReceipt {
    stops: Box<[UiPointerGestureStop]>,
    final_state: UiInteractionStateSnapshot,
}

impl UiInteractionLifecycleSettlementReceipt {
    pub(super) fn new(
        stops: Vec<UiPointerGestureStop>,
        final_state: UiInteractionStateSnapshot,
    ) -> Self {
        Self {
            stops: stops.into_boxed_slice(),
            final_state,
        }
    }

    pub fn stops(&self) -> &[UiPointerGestureStop] {
        &self.stops
    }

    pub const fn final_state(&self) -> UiInteractionStateSnapshot {
        self.final_state
    }

    pub fn settled_gestures(&self) -> usize {
        self.stops.len()
    }
}
