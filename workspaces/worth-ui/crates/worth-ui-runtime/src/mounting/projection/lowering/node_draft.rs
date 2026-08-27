use super::UiMountedProjectionNodeDraft;
use crate::mounting::projection::{
    frame_storage::UiMountedProjectionNodeRecord, node_receipt::UiMountedNodeReceiptInput,
};
use crate::mounting::UiMountedNodeReceipt;

impl UiMountedProjectionNodeDraft {
    pub(super) fn materialize(self) -> UiMountedProjectionNodeRecord {
        UiMountedProjectionNodeRecord {
            receipt: UiMountedNodeReceipt::from_input(UiMountedNodeReceiptInput {
                mounted_instance: self.mounted_instance,
                graph_node: self.graph_node,
                semantic_surface: self.semantic_surface,
                incarnation: self.incarnation,
                plan_digest: self.plan_digest,
                role: self.role,
                participation: self.participation,
                allocation: self.allocation,
            }),
            plan_index: self.plan_index,
            static_paint: self.static_paint,
            semantic_text: self.semantic_text,
            hit_test: self.hit_test,
            focus_support: self.focus_support,
            focus_scope: self.focus_scope,
            component_id: self.component_id,
            portal_child_owner: self.portal_child_owner,
        }
    }
}
