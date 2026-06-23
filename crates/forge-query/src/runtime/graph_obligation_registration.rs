use super::{
    ForgeQueryGraphObligationIndex, ForgeQueryGraphObligationOperatingWorldDescriptor,
    ForgeQueryGraphObligationRegistrationCatalog, ForgeQueryGraphObligationSelection,
    ForgeQueryGraphTouchDescriptor, ForgeQueryRuntime,
};

impl ForgeQueryRuntime {
    pub fn graph_obligation_registration_catalog(
        &self,
    ) -> &ForgeQueryGraphObligationRegistrationCatalog {
        &self.graph_obligation_registration_catalog
    }

    pub fn graph_obligation_index(&self) -> &ForgeQueryGraphObligationIndex {
        &self.graph_obligation_index
    }

    pub fn select_graph_obligations_for_touch(
        &self,
        touch_descriptor: &ForgeQueryGraphTouchDescriptor,
        operating_world: &ForgeQueryGraphObligationOperatingWorldDescriptor,
    ) -> ForgeQueryGraphObligationSelection {
        self.graph_obligation_index
            .select_for_touch(touch_descriptor, operating_world)
    }
}
