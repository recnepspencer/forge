use super::{
    reserve_presentation_owners, settle_port_result, UiNativePendingExternalObligation,
    UiNativePresentationFailure, UiNativePresentationPortFailure,
};

struct PendingProbe {
    dropped: std::rc::Rc<std::cell::Cell<bool>>,
    settles: std::rc::Rc<std::cell::Cell<bool>>,
}

impl UiNativePendingExternalObligation for PendingProbe {
    fn try_settle(&mut self, _device: Option<&wgpu::Device>) -> bool {
        self.settles.get()
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
    let owners = reserve_presentation_owners(&mut resources)
        .unwrap_or_else(|_| panic!("empty registry must reserve presentation owners"));
    let denied = settle_port_result(
        &mut resources,
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
    let owners = reserve_presentation_owners(&mut resources)
        .unwrap_or_else(|_| panic!("released registry must reserve presentation owners"));
    let pending = Box::new(PendingProbe {
        dropped: std::rc::Rc::clone(&dropped),
        settles: std::rc::Rc::new(std::cell::Cell::new(false)),
    });
    let unsettled = settle_port_result(
        &mut resources,
        owners,
        Err(UiNativePresentationPortFailure::ReadbackUnsettled(pending)),
    );
    let Err(UiNativePresentationFailure::Indeterminate(pending)) = unsettled else {
        panic!("readback failure must remain indeterminate");
    };
    assert_eq!(resources.current().readback_buffers, 1);
    assert_eq!(resources.current().pending_submissions, 1);
    assert!(!dropped.get());
    pending.release(&mut resources);
    assert!(dropped.get());
    assert!(resources.current().is_zero());
}
