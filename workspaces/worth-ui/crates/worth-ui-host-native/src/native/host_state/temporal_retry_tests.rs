use std::cell::Cell;
use std::rc::Rc;

use crate::native::physical_work_signal::UiNativePhysicalPresentationBasis;
use crate::native::presentation::{
    reserve_presentation_owners, settle_port_result, UiNativePendingExternalObligation,
    UiNativePresentationFailure, UiNativePresentationPortFailure,
};

use super::UiNativeHostState;

struct RetryablePresentationProbe {
    settled: Rc<Cell<bool>>,
    polls: Rc<Cell<u32>>,
}

impl UiNativePendingExternalObligation for RetryablePresentationProbe {
    fn poll_observation(
        &mut self,
        basis: crate::native::physical_work_signal::UiNativePhysicalSignalExternalBasis,
        _device: Option<&wgpu::Device>,
    ) -> crate::native::physical_work_signal::UiNativePhysicalSignalExternalObservation {
        self.polls.set(self.polls.get().saturating_add(1));
        basis.observe(if self.settled.get() {
            crate::native::physical_work_signal::UiNativePhysicalSignalStatus::Completed
        } else {
            crate::native::physical_work_signal::UiNativePhysicalSignalStatus::Pending
        })
    }
}

#[test]
fn temporal_retry_refreshes_the_exact_retained_presentation_attempt() {
    let settled = Rc::new(Cell::new(false));
    let polls = Rc::new(Cell::new(0));
    let mut state = UiNativeHostState::new();
    let owners = reserve_presentation_owners(
        &mut state.resources,
        &mut state.physical_signal,
        UiNativePhysicalPresentationBasis::test(),
    )
    .unwrap_or_else(|_| panic!("empty host admits one presentation obligation"));
    let Err(UiNativePresentationFailure::Indeterminate(pending)) = settle_port_result(
        &mut state.resources,
        &mut state.physical_signal,
        owners,
        Err(UiNativePresentationPortFailure::ReadbackUnsettled(
            Box::new(RetryablePresentationProbe {
                settled: Rc::clone(&settled),
                polls: Rc::clone(&polls),
            }),
        )),
    ) else {
        panic!("unsettled presentation must retain its physical obligation");
    };
    state.pending_presentations.push(pending);

    assert_eq!(state.physical_signal.next_due_tick(), Some(1));
    settled.set(true);
    state.physical_signal.advance_clock_to(1).unwrap();
    assert!(state.progress_one_physical_signal_ready());
    assert_eq!(polls.get(), 1);
    assert!(state.pending_presentations.is_empty());
    assert_eq!(state.physical_signal.observation().active_requests, 0);
    assert!(state.resources.current().is_zero());
}
