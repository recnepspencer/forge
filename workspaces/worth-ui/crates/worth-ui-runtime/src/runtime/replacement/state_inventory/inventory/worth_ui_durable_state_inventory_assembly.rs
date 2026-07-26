use crate::capability::FrozenMosaicStateCapabilities;
use crate::runtime::{
    WorthUiDurableStateFamily, WorthUiDurableStateInventory, WorthUiDurableStateInventoryCounters,
    WorthUiDurableStateInventoryDenial, WorthUiNodeReplacementPlan,
    WorthUiTransientInteractionState,
};

use super::super::family::admitted_mosaic_state_family;

impl WorthUiDurableStateInventory {
    pub(crate) fn assemble_for_replacement(
        node_plan: &WorthUiNodeReplacementPlan,
        admitted_state_capabilities: &FrozenMosaicStateCapabilities,
    ) -> Result<Self, WorthUiDurableStateInventoryDenial> {
        let mut counters = WorthUiDurableStateInventoryCounters::default();
        counters.record_replacement_classifications(node_plan.classifications().len());
        if !node_plan.is_unambiguous() {
            return Err(
                WorthUiDurableStateInventoryDenial::AmbiguousNodeReplacementPlan { counters },
            );
        }

        let mut families = platform_families().to_vec();
        counters.record_platform_families(families.len());
        families.extend(
            admitted_state_capabilities
                .entries()
                .iter()
                .map(admitted_mosaic_state_family),
        );
        counters.record_application_families(admitted_state_capabilities.len());
        counters.record_transient_drop_policies(WorthUiTransientInteractionState::all().len());

        Ok(Self::new(
            node_plan.active_artifact_digest(),
            node_plan.candidate_artifact_digest(),
            families,
            counters,
        ))
    }
}

fn platform_families() -> [WorthUiDurableStateFamily; 7] {
    [
        WorthUiDurableStateFamily::focus_chain(),
        WorthUiDurableStateFamily::scroll_anchor(),
        WorthUiDurableStateFamily::selection_range(),
        WorthUiDurableStateFamily::text_edit_buffer(),
        WorthUiDurableStateFamily::splitter_position(),
        WorthUiDurableStateFamily::tab_state(),
        WorthUiDurableStateFamily::panel_visibility(),
    ]
}
