use crate::capability::{ComponentId, SurfaceId};

use super::super::WorthUiQueryGraphExecutionReceipt;
use super::WorthUiUserIntentOperationFamily;

pub(super) fn target_binding_digest(
    slot_name: &str,
    surface_id: &SurfaceId,
    component_id: &ComponentId,
    family: WorthUiUserIntentOperationFamily,
    graph_execution: &WorthUiQueryGraphExecutionReceipt,
) -> u64 {
    let basis = format!(
        "target|slot:{slot_name}|surface:{}|component:{}|family:{family:?}|graph:{}",
        surface_id.as_str(),
        component_id.as_str(),
        graph_execution.execution_digest()
    );
    basis.bytes().fold(0xcbf2_9ce4_8422_2325, |mut acc, byte| {
        acc ^= u64::from(byte);
        acc.wrapping_mul(0x0000_0100_0000_01b3)
    })
}
