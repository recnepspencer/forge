use worth_relational::facade::runtime::CustomInvariantRegistration;

use super::operating_world_selector::WorthQueryGraphObligationOperatingWorldSelector;
use super::registration::WorthQueryGraphObligationRegistration;
use super::support_posture::WorthQueryGraphObligationSupportPosture;
use super::touch_selector::WorthQueryGraphTouchSelector;

#[derive(Clone, Debug)]
pub struct WorthQueryGraphScopedCustomInvariantRegistration {
    custom_invariant: CustomInvariantRegistration,
    touch_selector: WorthQueryGraphTouchSelector,
    operating_world_selector: WorthQueryGraphObligationOperatingWorldSelector,
    support_posture: WorthQueryGraphObligationSupportPosture,
}

impl WorthQueryGraphScopedCustomInvariantRegistration {
    pub fn new(
        custom_invariant: CustomInvariantRegistration,
        touch_selector: WorthQueryGraphTouchSelector,
        operating_world_selector: WorthQueryGraphObligationOperatingWorldSelector,
    ) -> Self {
        Self {
            custom_invariant,
            touch_selector,
            operating_world_selector,
            support_posture: WorthQueryGraphObligationSupportPosture::default_selection_posture(),
        }
    }

    pub fn with_support_posture(
        mut self,
        support_posture: WorthQueryGraphObligationSupportPosture,
    ) -> Self {
        self.support_posture = support_posture;
        self
    }

    pub fn custom_invariant(&self) -> &CustomInvariantRegistration {
        &self.custom_invariant
    }

    pub fn touch_selector(&self) -> &WorthQueryGraphTouchSelector {
        &self.touch_selector
    }

    pub fn operating_world_selector(&self) -> WorthQueryGraphObligationOperatingWorldSelector {
        self.operating_world_selector
    }

    pub fn support_posture(&self) -> &WorthQueryGraphObligationSupportPosture {
        &self.support_posture
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        CustomInvariantRegistration,
        WorthQueryGraphObligationRegistration,
    ) {
        let graph_obligation = WorthQueryGraphObligationRegistration::custom_invariant(
            &self.custom_invariant,
            self.touch_selector.clone(),
            self.operating_world_selector,
        )
        .with_support_posture(self.support_posture);
        (self.custom_invariant, graph_obligation)
    }
}
