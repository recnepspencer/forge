use crate::capability::{ComponentId, SurfaceId};

use super::super::{
    WorthUiQueryGraphExecutionReceipt, WorthUiRuntimeFactId, WorthUiRuntimeGraphAuthority,
};
use super::{WorthUiUserIntentOperationFamily, WorthUiUserIntentTargetPosture};

pub(super) fn target_graph_execution_with_authority(
    graph_authority: &WorthUiRuntimeGraphAuthority,
    slot_name: &str,
    surface_id: &SurfaceId,
    component_id: &ComponentId,
    operation_family: WorthUiUserIntentOperationFamily,
    posture: WorthUiUserIntentTargetPosture,
) -> WorthUiQueryGraphExecutionReceipt {
    graph_authority
        .plan_user_intent_target_binding_graph_operation(
            slot_name,
            surface_id,
            component_id,
            operation_family,
            posture,
            [
                WorthUiRuntimeFactId::active_artifact(),
                WorthUiRuntimeFactId::surface_mount(surface_id),
                WorthUiRuntimeFactId::authored_mount_component_selection(surface_id.as_str()),
                WorthUiRuntimeFactId::authored_surface_props(surface_id.as_str()),
                WorthUiRuntimeFactId::component(component_id),
            ],
        )
        .into_execution_receipt()
}
