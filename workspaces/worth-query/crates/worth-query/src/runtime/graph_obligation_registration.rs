use super::{
    WorthQueryGraphObligationIndex, WorthQueryGraphObligationOperatingWorldDescriptor,
    WorthQueryGraphObligationRegistrationCatalog, WorthQueryGraphObligationSelection,
    WorthQueryGraphTouchDescriptor, WorthQueryRuntime,
};

impl WorthQueryRuntime {
    pub fn graph_obligation_registration_catalog(
        &self,
    ) -> &WorthQueryGraphObligationRegistrationCatalog {
        &self.graph_obligation_registration_catalog
    }

    pub fn graph_obligation_index(&self) -> &WorthQueryGraphObligationIndex {
        &self.graph_obligation_index
    }

    pub fn select_graph_obligations_for_touch(
        &self,
        touch_descriptor: &WorthQueryGraphTouchDescriptor,
        operating_world: &WorthQueryGraphObligationOperatingWorldDescriptor,
    ) -> WorthQueryGraphObligationSelection {
        self.graph_obligation_index
            .select_for_touch(touch_descriptor, operating_world)
    }
}
