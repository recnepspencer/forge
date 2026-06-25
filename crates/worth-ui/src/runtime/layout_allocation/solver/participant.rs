use crate::runtime::{
    WorthUiAdmittedHostFrameObservationReceipt, WorthUiCompositionGraphChildAccessRow,
    WorthUiLayoutAllocatedChildSizing, WorthUiLayoutParticipationPosture,
    WorthUiMountedNodeReceipt,
};

use super::natural_metrics::natural_metrics_for_mounted_node;

#[derive(Clone, Debug)]
pub(in crate::runtime::layout_allocation) struct WorthUiLayoutAllocationParticipant {
    pub(super) parent_id: String,
    pub(super) child_node_id: String,
    pub(super) order: u32,
    pub(super) sizing: WorthUiLayoutAllocatedChildSizing,
    pub(super) participation: WorthUiLayoutParticipationPosture,
    pub(super) natural_width: f32,
    pub(super) natural_height: f32,
    pub(super) natural_baseline: f32,
    pub(super) natural_metric_basis: String,
}

pub(in crate::runtime::layout_allocation) fn participant_from_row_and_node(
    row: &WorthUiCompositionGraphChildAccessRow,
    node: &WorthUiMountedNodeReceipt,
    observations: &WorthUiAdmittedHostFrameObservationReceipt,
) -> WorthUiLayoutAllocationParticipant {
    let natural = natural_metrics_for_mounted_node(node, observations);
    WorthUiLayoutAllocationParticipant {
        parent_id: row.parent_id().to_owned(),
        child_node_id: row.node().node_id().as_str().to_owned(),
        order: row.order(),
        sizing: WorthUiLayoutAllocatedChildSizing::from_composition_sizing(row.edge().sizing()),
        participation: participation_for_mounted_node(node, row),
        natural_width: natural.width,
        natural_height: natural.height,
        natural_baseline: natural.baseline,
        natural_metric_basis: natural.basis,
    }
}

fn participation_for_mounted_node(
    node: &WorthUiMountedNodeReceipt,
    row: &WorthUiCompositionGraphChildAccessRow,
) -> WorthUiLayoutParticipationPosture {
    match node {
        WorthUiMountedNodeReceipt::Control(frame) => {
            WorthUiLayoutParticipationPosture::from_live_view_participation(
                frame.participation(),
                row.node().participation(),
            )
        }
        _ => WorthUiLayoutParticipationPosture::from_composition_participation(
            row.node().participation(),
        ),
    }
}
