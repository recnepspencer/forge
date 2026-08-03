mod accessors;
mod model;
mod transition;

use std::collections::BTreeMap;

use worth_ui_host_contract::{UiHostPointerIdentity, UiSurfaceBindingGeneration};

use super::UiPointerGestureStopReason;
use model::UiActivePointerGesture;
pub(crate) use model::{
    UiPointerGestureOutcome, UiPointerGestureRuntimeState, UiPointerGestureStateSnapshot,
};
pub use model::{
    UiPointerGesturePressReceipt, UiTargetedPointerGesture, UI_ACTIVE_POINTER_GESTURE_LIMIT,
};

impl UiPointerGestureRuntimeState {
    pub(crate) fn new() -> Self {
        Self {
            active: BTreeMap::new(),
            counters: Default::default(),
        }
    }

    pub(crate) fn process_report(
        &mut self,
        core: worth_ui_host_contract::UiHostObservationCanonicalCore,
        report: &worth_ui_host_contract::UiHostObservationReport,
        mounted: &crate::mounting::WorthUiMountedSessionState,
    ) -> Vec<UiPointerGestureOutcome> {
        self.process_pointer_report(core, report, mounted)
    }

    pub(crate) fn snapshot(&self) -> UiPointerGestureStateSnapshot {
        UiPointerGestureStateSnapshot {
            active_gestures: self.active.len(),
            counters: self.counters,
        }
    }

    pub(crate) fn cancel_binding(
        &mut self,
        binding: UiSurfaceBindingGeneration,
        reason: UiPointerGestureStopReason,
    ) -> Vec<super::UiPointerGestureStop> {
        self.cancel_where(|active| active.target.binding() == binding, reason)
    }

    pub(crate) fn cancel_instance(
        &mut self,
        instance: worth_ui_host_contract::UiMountedInstanceIdentity,
        reason: UiPointerGestureStopReason,
    ) -> Vec<super::UiPointerGestureStop> {
        self.cancel_where(
            |active| active.target.mounted_instance() == instance,
            reason,
        )
    }

    pub(crate) fn cancel_all(
        &mut self,
        reason: UiPointerGestureStopReason,
    ) -> Vec<super::UiPointerGestureStop> {
        self.cancel_where(|_| true, reason)
    }

    fn cancel_where(
        &mut self,
        predicate: impl Fn(&UiActivePointerGesture) -> bool,
        reason: UiPointerGestureStopReason,
    ) -> Vec<super::UiPointerGestureStop> {
        let selected = take_matching(&mut self.active, predicate);
        self.counters.stop_outcomes = add(self.counters.stop_outcomes, selected.len());
        self.counters.active_gestures_settled =
            add(self.counters.active_gestures_settled, selected.len());
        selected
            .into_iter()
            .map(|(pointer, active)| {
                super::UiPointerGestureStop::new(
                    pointer,
                    active.capture_epoch,
                    active.button,
                    None,
                    true,
                    reason,
                )
            })
            .collect()
    }

    pub(super) fn bump_button_reports(&mut self) {
        self.counters.button_reports = next(self.counters.button_reports);
    }
}

fn take_matching(
    active: &mut BTreeMap<UiHostPointerIdentity, UiActivePointerGesture>,
    predicate: impl Fn(&UiActivePointerGesture) -> bool,
) -> Vec<(UiHostPointerIdentity, UiActivePointerGesture)> {
    let selected = active
        .iter()
        .filter_map(|(pointer, gesture)| predicate(gesture).then_some(*pointer))
        .collect::<Vec<_>>();
    selected
        .into_iter()
        .map(|pointer| {
            let gesture = active
                .remove(&pointer)
                .expect("the selected gesture remains active");
            (pointer, gesture)
        })
        .collect()
}

fn next(value: u64) -> u64 {
    value
        .checked_add(1)
        .expect("host observation sequence exhaustion stops before counter overflow")
}

fn add(value: u64, count: usize) -> u64 {
    value
        .checked_add(u64::try_from(count).expect("bounded gesture count fits u64"))
        .expect("host observation sequence exhaustion stops before counter overflow")
}
