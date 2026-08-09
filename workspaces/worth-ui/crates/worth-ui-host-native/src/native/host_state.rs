use std::collections::BTreeMap;

use worth_ui_host_contract::UiHostSurfaceRegistrationRequest;

use super::{
    UiNativeGraphics, UiNativePendingPresentation, UiNativePresentationObservation,
    UiNativeResourceCensus, UiNativeResourceOwner, UiNativeResourceRegistry,
};

pub(crate) struct UiNativeHostState {
    pub(crate) registrations: BTreeMap<u64, UiHostSurfaceRegistrationRequest>,
    pub(crate) registration_resources: BTreeMap<u64, UiNativeResourceOwner>,
    pub(crate) graphics: Option<UiNativeGraphics>,
    pub(crate) graphics_resources: Vec<UiNativeResourceOwner>,
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
            graphics: None,
            graphics_resources: Vec::new(),
            last_presentation: None,
            resources: UiNativeResourceRegistry::new(),
            effect_posture: UiNativeEffectPosture::BeforeEffects,
            pending_presentations: Vec::new(),
        }
    }

    pub(crate) fn close(&mut self) -> UiNativeResourceCensus {
        self.last_presentation = None;
        self.graphics = None;
        for owner in self.graphics_resources.drain(..) {
            self.resources
                .release(owner)
                .expect("graphics resource owner must remain exact");
        }
        self.resources.current()
    }
}
