use std::collections::BTreeMap;

use worth_ui_host_contract::UiHostSurfaceRegistrationRequest;

use super::{
    event_loop::UiNativeOwnedWindow, UiNativeOwnedGraphics, UiNativePendingPresentation,
    UiNativePresentationObservation, UiNativeResourceCensus, UiNativeResourceOwner,
    UiNativeResourceRegistry,
};

pub(crate) struct UiNativeHostState {
    pub(crate) registrations: BTreeMap<u64, UiHostSurfaceRegistrationRequest>,
    pub(crate) registration_resources: BTreeMap<u64, UiNativeResourceOwner>,
    pub(crate) window: Option<UiNativeOwnedWindow>,
    pub(crate) graphics: Option<UiNativeOwnedGraphics>,
    pub(crate) last_presentation: Option<UiNativePresentationObservation>,
    pub(crate) resources: UiNativeResourceRegistry,
    pub(crate) effect_posture: UiNativeEffectPosture,
    pub(crate) pending_presentations: Vec<UiNativePendingPresentation>,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum UiNativeEffectPosture {
    #[default]
    BeforeEffects,
    PresentationIndeterminate,
    Presented,
}

impl UiNativeHostState {
    pub(crate) fn new() -> Self {
        Self {
            registrations: BTreeMap::new(),
            registration_resources: BTreeMap::new(),
            window: None,
            graphics: None,
            last_presentation: None,
            resources: UiNativeResourceRegistry::new(),
            effect_posture: UiNativeEffectPosture::BeforeEffects,
            pending_presentations: Vec::new(),
        }
    }

    pub(crate) fn close(&mut self) -> UiNativeResourceCensus {
        self.last_presentation = None;
        let mut retained = Vec::new();
        {
            let device = self.graphics.as_ref().map(|graphics| &graphics.device);
            for mut pending in self.pending_presentations.drain(..) {
                if pending.try_settle(device) {
                    pending.release(&mut self.resources);
                } else {
                    retained.push(pending);
                }
            }
        }
        self.pending_presentations = retained;
        if self.pending_presentations.is_empty() {
            if let Some(graphics) = self.graphics.take() {
                graphics.close(&mut self.resources);
            }
            if let Some(window) = self.window.take() {
                window.close(&mut self.resources);
            }
        }
        self.resources.current()
    }
}

#[cfg(test)]
mod tests {
    use super::UiNativeHostState;
    use crate::native::presentation::{
        reserve_presentation_owners, settle_port_result, UiNativePendingExternalObligation,
        UiNativePresentationFailure, UiNativePresentationPortFailure,
    };
    use std::cell::Cell;
    use std::rc::Rc;

    struct PendingProbe {
        dropped: Rc<Cell<bool>>,
        settles: Rc<Cell<bool>>,
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
    fn pending_external_work_retains_cleanup_authority_until_it_settles() {
        let mut state = UiNativeHostState::new();
        let dropped = Rc::new(Cell::new(false));
        let settles = Rc::new(Cell::new(false));
        let owners = reserve_presentation_owners(&mut state.resources)
            .unwrap_or_else(|_| panic!("empty registry must reserve presentation owners"));
        let pending = settle_port_result(
            &mut state.resources,
            owners,
            Err(UiNativePresentationPortFailure::ReadbackUnsettled(
                Box::new(PendingProbe {
                    dropped: Rc::clone(&dropped),
                    settles: Rc::clone(&settles),
                }),
            )),
        );
        let Err(UiNativePresentationFailure::Indeterminate(pending)) = pending else {
            panic!("unsettled external work must enter pending host state");
        };
        state.pending_presentations.push(pending);

        let pending = state.close();
        assert_eq!(pending.readback_buffers, 1);
        assert_eq!(pending.pending_submissions, 1);
        assert!(!dropped.get());

        settles.set(true);
        assert!(state.close().is_zero());
        assert!(dropped.get());
    }
}
