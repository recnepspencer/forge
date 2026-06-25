use worth_ui::facade::WorthUiMountedNodeReceipt;

use crate::app::live_view::proof::ValidationLiveViewProjectionProof;

pub(super) fn first_surface_node_id(proof: &ValidationLiveViewProjectionProof) -> Option<String> {
    proof
        .mounted_product_view()
        .composition_tree()
        .root_children()
        .iter()
        .find_map(|child| match child.mounted_node() {
            WorthUiMountedNodeReceipt::Surface(_) => Some(child.node_id().to_owned()),
            _ => None,
        })
}
