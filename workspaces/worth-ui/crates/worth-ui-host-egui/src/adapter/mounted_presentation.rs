use worth_ui_host_contract::{
    UiHostSurfacePresentationDenial, UiMountedCompletedEffects, UiMountedEffectFamily,
    UiMountedFrameConsumptionView,
};

#[derive(Clone)]
pub(super) struct UiEguiPreparedMountedPresentation {
    native_paint: super::native_paint::UiEguiPreparedNativePaint,
    identity_overlay: super::identity_overlay::UiEguiPreparedIdentityOverlay,
}

impl UiEguiPreparedMountedPresentation {
    pub(super) fn prepare(
        context: &egui::Context,
        view: &UiMountedFrameConsumptionView<'_>,
    ) -> Result<Self, UiHostSurfacePresentationDenial> {
        Ok(Self {
            native_paint: super::native_paint::UiEguiPreparedNativePaint::prepare(view)?,
            identity_overlay: super::identity_overlay::UiEguiPreparedIdentityOverlay::prepare(
                context,
                view.projection(),
            )?,
        })
    }

    pub(super) fn is_empty(&self) -> bool {
        self.native_paint.is_empty() && self.identity_overlay.is_empty()
    }

    pub(super) fn completed_effects(&self) -> UiMountedCompletedEffects {
        let mut effects = Vec::new();
        if !self.native_paint.is_empty() {
            effects.push(UiMountedEffectFamily::NativePaint);
        }
        if !self.identity_overlay.is_empty() {
            effects.push(UiMountedEffectFamily::IdentityOverlay);
        }
        UiMountedCompletedEffects::new(effects)
    }

    pub(super) fn paint(&self, context: &egui::Context) {
        self.native_paint.paint(context);
        self.identity_overlay.paint(context);
    }
}
