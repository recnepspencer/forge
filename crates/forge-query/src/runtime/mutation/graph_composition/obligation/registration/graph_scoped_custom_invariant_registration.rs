use forge_relational::facade::runtime::CustomInvariantRegistration;

use super::operating_world_selector::ForgeQueryGraphObligationOperatingWorldSelector;
use super::registration::ForgeQueryGraphObligationRegistration;
use super::support_posture::ForgeQueryGraphObligationSupportPosture;
use super::touch_selector::ForgeQueryGraphTouchSelector;

#[derive(Clone, Debug)]
pub struct ForgeQueryGraphScopedCustomInvariantRegistration {
    custom_invariant: CustomInvariantRegistration,
    touch_selector: ForgeQueryGraphTouchSelector,
    operating_world_selector: ForgeQueryGraphObligationOperatingWorldSelector,
    support_posture: ForgeQueryGraphObligationSupportPosture,
}

impl ForgeQueryGraphScopedCustomInvariantRegistration {
    pub fn new(
        custom_invariant: CustomInvariantRegistration,
        touch_selector: ForgeQueryGraphTouchSelector,
        operating_world_selector: ForgeQueryGraphObligationOperatingWorldSelector,
    ) -> Self {
        Self {
            custom_invariant,
            touch_selector,
            operating_world_selector,
            support_posture: ForgeQueryGraphObligationSupportPosture::default_selection_posture(),
        }
    }

    pub fn with_support_posture(
        mut self,
        support_posture: ForgeQueryGraphObligationSupportPosture,
    ) -> Self {
        self.support_posture = support_posture;
        self
    }

    pub fn custom_invariant(&self) -> &CustomInvariantRegistration {
        &self.custom_invariant
    }

    pub fn touch_selector(&self) -> &ForgeQueryGraphTouchSelector {
        &self.touch_selector
    }

    pub fn operating_world_selector(&self) -> ForgeQueryGraphObligationOperatingWorldSelector {
        self.operating_world_selector
    }

    pub fn support_posture(&self) -> &ForgeQueryGraphObligationSupportPosture {
        &self.support_posture
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        CustomInvariantRegistration,
        ForgeQueryGraphObligationRegistration,
    ) {
        let graph_obligation = ForgeQueryGraphObligationRegistration::custom_invariant(
            &self.custom_invariant,
            self.touch_selector.clone(),
            self.operating_world_selector,
        )
        .with_support_posture(self.support_posture);
        (self.custom_invariant, graph_obligation)
    }
}
