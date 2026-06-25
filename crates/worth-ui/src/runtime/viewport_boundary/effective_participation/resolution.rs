use crate::runtime::{
    WorthUiMountedProductViewReceipt, WorthUiRuntimeFactId, WorthUiRuntimeHost,
    WorthUiViewportBoundaryReceipt,
};

use super::receipt::WorthUiEffectiveViewportParticipationReceipt;
use super::row::WorthUiEffectiveViewportParticipationRow;

impl WorthUiRuntimeHost {
    pub fn resolve_effective_viewport_participation(
        &self,
        mounted: &WorthUiMountedProductViewReceipt,
        viewport: &WorthUiViewportBoundaryReceipt,
    ) -> WorthUiEffectiveViewportParticipationReceipt {
        resolve_effective_viewport_participation(mounted, viewport)
    }
}

fn resolve_effective_viewport_participation(
    mounted: &WorthUiMountedProductViewReceipt,
    viewport: &WorthUiViewportBoundaryReceipt,
) -> WorthUiEffectiveViewportParticipationReceipt {
    WorthUiEffectiveViewportParticipationReceipt::new(
        effective_viewport_rows(mounted, viewport),
        effective_viewport_consumed_facts(mounted, viewport),
    )
}

fn effective_viewport_rows(
    mounted: &WorthUiMountedProductViewReceipt,
    viewport: &WorthUiViewportBoundaryReceipt,
) -> Vec<WorthUiEffectiveViewportParticipationRow> {
    mounted
        .composition_tree()
        .graph_access()
        .child_rows()
        .iter()
        .filter_map(|child| effective_row_for_node(child.node().node_id().as_str(), viewport))
        .collect()
}

fn effective_row_for_node(
    node_id: &str,
    viewport: &WorthUiViewportBoundaryReceipt,
) -> Option<WorthUiEffectiveViewportParticipationRow> {
    WorthUiEffectiveViewportParticipationRow::from_governing_rows(
        node_id,
        viewport.boundaries().iter().filter_map(|boundary| {
            boundary
                .descendants()
                .iter()
                .find(|descendant| descendant.node_id() == node_id)
        }),
    )
}

fn effective_viewport_consumed_facts(
    mounted: &WorthUiMountedProductViewReceipt,
    viewport: &WorthUiViewportBoundaryReceipt,
) -> Vec<WorthUiRuntimeFactId> {
    let mut facts = viewport.consumed_facts().to_vec();
    facts.extend(
        mounted
            .composition_tree()
            .graph_access()
            .plan()
            .consumed_facts()
            .iter()
            .cloned(),
    );
    facts
}
