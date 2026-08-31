#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct UiActiveThemeBinding {
    pub(super) surface: worth_ui_host_contract::UiSemanticSurfaceIdentity,
    pub(super) binding_generation: u64,
    pub(super) capability: super::UiThemeCapabilityReceipt,
}

impl UiActiveThemeBinding {
    pub(crate) const fn surface(&self) -> worth_ui_host_contract::UiSemanticSurfaceIdentity {
        self.surface
    }

    pub(crate) const fn binding_generation(&self) -> u64 {
        self.binding_generation
    }

    pub(crate) const fn capability(&self) -> &super::UiThemeCapabilityReceipt {
        &self.capability
    }
}
