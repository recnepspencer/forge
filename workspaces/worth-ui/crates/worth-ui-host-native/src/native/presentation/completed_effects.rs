use worth_ui_host_contract::{UiMountedCompletedEffects, UiMountedEffectFamily};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct UiNativePresentationEffects {
    native_paint: bool,
    identity_overlay: bool,
}

impl UiNativePresentationEffects {
    pub(crate) const fn new(native_paint: bool, identity_overlay: bool) -> Self {
        Self {
            native_paint,
            identity_overlay,
        }
    }

    pub(crate) fn inherit(&mut self, predecessor: Self) {
        self.native_paint |= predecessor.native_paint;
        self.identity_overlay |= predecessor.identity_overlay;
    }

    pub(crate) const fn without_native_paint(self) -> Self {
        Self::new(false, self.identity_overlay)
    }

    pub(crate) fn completion(self) -> UiMountedCompletedEffects {
        let mut families =
            Vec::with_capacity(usize::from(self.native_paint) + usize::from(self.identity_overlay));
        if self.native_paint {
            families.push(UiMountedEffectFamily::NativePaint);
        }
        if self.identity_overlay {
            families.push(UiMountedEffectFamily::IdentityOverlay);
        }
        UiMountedCompletedEffects::new(families)
    }
}
