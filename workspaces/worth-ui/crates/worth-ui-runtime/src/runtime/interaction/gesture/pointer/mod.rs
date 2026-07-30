mod accessors;
mod model;
mod transition;

use std::collections::BTreeMap;

use worth_ui_host_contract::{UiHostPointerIdentity, UiSurfaceBindingGeneration};

use super::UiPointerGestureStopReason;
use model::UiActivePointerGesture;
pub(crate) use model::UiInteractionRuntimeState;
pub use model::{
    UiHostInteractionIngressOutcome, UiInteractionBatchReceipt, UiInteractionLifecycleCounters,
    UiInteractionObservationDenial, UiInteractionShutdownReport, UiInteractionStateSnapshot,
    UiPointerGesturePressReceipt, UiPointerGestureTransition, UiQuarantinedHostInteractionBatch,
    UiTargetedPointerGesture, UI_ACTIVE_POINTER_GESTURE_LIMIT,
};

impl UiInteractionRuntimeState {
    pub(crate) fn new() -> Self {
        Self {
            active: BTreeMap::new(),
            counters: Default::default(),
        }
    }

    pub(crate) fn ingest(
        &mut self,
        batch: crate::facade::observation_report::UiValidatedHostObservationBatch,
        mounted: &crate::mounting::WorthUiMountedSessionState,
    ) -> UiInteractionBatchReceipt {
        let core = batch.canonical_core();
        let mut transitions = Vec::new();
        let mut ignored_reports = 0;
        for validated in batch.reports() {
            let emitted = self.process_report(core, validated.report(), mounted);
            if emitted.is_empty() {
                ignored_reports += 1;
            } else {
                transitions.extend(emitted);
            }
        }
        UiInteractionBatchReceipt {
            core,
            frame_relation: batch.frame_relation(),
            disposition: batch.disposition(),
            transitions: transitions.into_boxed_slice(),
            ignored_reports,
            state: self.snapshot(),
        }
    }

    pub(crate) fn snapshot(&self) -> UiInteractionStateSnapshot {
        UiInteractionStateSnapshot {
            active_gestures: self.active.len(),
            counters: self.counters,
        }
    }

    pub(crate) fn cancel_binding(
        &mut self,
        binding: UiSurfaceBindingGeneration,
        reason: UiPointerGestureStopReason,
    ) -> super::UiInteractionLifecycleSettlementReceipt {
        self.cancel_where(|active| active.target.binding() == binding, reason)
    }

    pub(crate) fn cancel_instance(
        &mut self,
        instance: worth_ui_host_contract::UiMountedInstanceIdentity,
        reason: UiPointerGestureStopReason,
    ) -> super::UiInteractionLifecycleSettlementReceipt {
        self.cancel_where(
            |active| active.target.mounted_instance() == instance,
            reason,
        )
    }

    pub(crate) fn unchanged_settlement(&self) -> super::UiInteractionLifecycleSettlementReceipt {
        super::UiInteractionLifecycleSettlementReceipt::new(Vec::new(), self.snapshot())
    }

    pub(crate) fn cancel_all(
        &mut self,
        reason: UiPointerGestureStopReason,
    ) -> super::UiInteractionLifecycleSettlementReceipt {
        self.cancel_where(|_| true, reason)
    }

    pub(crate) fn shutdown(&mut self) -> UiInteractionShutdownReport {
        let settlement = self.cancel_all(UiPointerGestureStopReason::Shutdown);
        UiInteractionShutdownReport {
            settlement: Some(settlement),
        }
    }

    fn cancel_where(
        &mut self,
        predicate: impl Fn(&UiActivePointerGesture) -> bool,
        reason: UiPointerGestureStopReason,
    ) -> super::UiInteractionLifecycleSettlementReceipt {
        let selected = take_matching(&mut self.active, predicate);
        self.counters.stop_outcomes = add(self.counters.stop_outcomes, selected.len());
        self.counters.active_gestures_settled =
            add(self.counters.active_gestures_settled, selected.len());
        let stops = selected
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
            .collect();
        super::UiInteractionLifecycleSettlementReceipt::new(stops, self.snapshot())
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
