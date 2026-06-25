use crate::runtime::{
    WorthUiAdmittedCompositionGraphReceipt, WorthUiGraphBackedLiveViewProjectionReceipt,
};

pub(super) fn admitted_live_view_composition(
    projection: &WorthUiGraphBackedLiveViewProjectionReceipt,
) -> WorthUiAdmittedCompositionGraphReceipt {
    projection.composition_graph().clone()
}
