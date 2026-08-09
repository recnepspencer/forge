/// Move-only authority for the consolidated certification host lane.
#[must_use]
pub struct UiCertificationHostBindingGrant {
    _private: (),
}

/// Move-only authority for the bounded egui migration host lane.
#[must_use]
pub struct UiLegacyEguiHostBindingGrant {
    _private: (),
}

impl UiCertificationHostBindingGrant {
    #[cfg(feature = "certification-host-binding-authority")]
    #[doc(hidden)]
    pub const fn for_certification() -> Self {
        Self { _private: () }
    }
}

impl UiLegacyEguiHostBindingGrant {
    #[cfg(feature = "legacy-egui-host-binding-authority")]
    #[doc(hidden)]
    pub const fn for_migration() -> Self {
        Self { _private: () }
    }
}
