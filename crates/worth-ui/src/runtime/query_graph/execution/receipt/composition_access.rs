use super::super::super::{WorthUiQueryGraphOperatingWorld, WorthUiQueryGraphTouchDescriptor};
use super::super::registrations::composition_graph_access_registrations;
use super::WorthUiQueryGraphExecutionReceipt;

impl WorthUiQueryGraphExecutionReceipt {
    pub(crate) fn composition_graph_access(
        touch_descriptor: WorthUiQueryGraphTouchDescriptor,
        operating_world: WorthUiQueryGraphOperatingWorld,
    ) -> Self {
        Self::from_registrations(
            touch_descriptor,
            operating_world,
            composition_graph_access_registrations(),
            "Worth composition graph access registrations are generated from validated constants",
        )
    }
}
