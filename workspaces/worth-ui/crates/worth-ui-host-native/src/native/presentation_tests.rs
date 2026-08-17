use super::{
    reserve_presentation_owners, settle_port_result, UiNativePendingExternalObligation,
    UiNativePresentationFailure, UiNativePresentationPortFailure,
};

struct PendingProbe {
    dropped: std::rc::Rc<std::cell::Cell<bool>>,
    settles: std::rc::Rc<std::cell::Cell<bool>>,
}

impl UiNativePendingExternalObligation for PendingProbe {
    fn poll_observation(
        &mut self,
        basis: crate::native::physical_work_signal::UiNativePhysicalSignalExternalBasis,
        _device: Option<&wgpu::Device>,
    ) -> crate::native::physical_work_signal::UiNativePhysicalSignalExternalObservation {
        basis.observe(if self.settles.get() {
            crate::native::physical_work_signal::UiNativePhysicalSignalStatus::Completed
        } else {
            crate::native::physical_work_signal::UiNativePhysicalSignalStatus::Pending
        })
    }
}

impl Drop for PendingProbe {
    fn drop(&mut self) {
        self.dropped.set(true);
    }
}

#[test]
fn external_port_failures_cross_the_real_framework_settlement_transition() {
    let mut resources = crate::native::UiNativeResourceRegistry::new();
    let mut physical_signal =
        crate::native::physical_work_signal::UiNativePhysicalSignalOwner::new();
    let owners = reserve_presentation_owners(
        &mut resources,
        &mut physical_signal,
        crate::native::physical_work_signal::UiNativePhysicalPresentationBasis::test(),
    )
    .unwrap_or_else(|_| panic!("empty registry must reserve presentation owners"));
    let denied = settle_port_result(
        &mut resources,
        &mut physical_signal,
        owners,
        Err(UiNativePresentationPortFailure::SurfaceUnavailable),
    );
    assert!(matches!(
        denied,
        Err(UiNativePresentationFailure::BeforeEffects(
            worth_ui_host_contract::UiHostSurfacePresentationDenial::AdapterDeclined
        ))
    ));
    assert!(resources.current().is_zero());

    let dropped = std::rc::Rc::new(std::cell::Cell::new(false));
    let owners = reserve_presentation_owners(
        &mut resources,
        &mut physical_signal,
        crate::native::physical_work_signal::UiNativePhysicalPresentationBasis::test(),
    )
    .unwrap_or_else(|_| panic!("released registry must reserve presentation owners"));
    let pending = Box::new(PendingProbe {
        dropped: std::rc::Rc::clone(&dropped),
        settles: std::rc::Rc::new(std::cell::Cell::new(false)),
    });
    let unsettled = settle_port_result(
        &mut resources,
        &mut physical_signal,
        owners,
        Err(UiNativePresentationPortFailure::ReadbackUnsettled(pending)),
    );
    let Err(UiNativePresentationFailure::Indeterminate(pending)) = unsettled else {
        panic!("readback failure must remain indeterminate");
    };
    assert_eq!(resources.current().readback_buffers, 1);
    assert_eq!(resources.current().pending_submissions, 1);
    assert!(!dropped.get());
    let due = physical_signal
        .next_due_tick()
        .expect("pending presentation must retain one Signal-owned poll wake");
    physical_signal
        .advance_clock_to(due)
        .expect("the exact pending poll wake must become ready");
    let token = physical_signal
        .take_ready_presentation(pending.physical_work())
        .unwrap();
    assert!(matches!(
        physical_signal.reconcile(
            token.observe(
                crate::native::physical_work_signal::UiNativePhysicalSignalStatus::Completed,
            )
        ),
        crate::native::physical_work_signal::UiNativePhysicalSignalSettlement::Completed
    ));
    pending.release(&mut resources);
    assert!(dropped.get());
    assert!(resources.current().is_zero());
}
