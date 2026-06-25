use crate::runtime::WorthUiProjectionDependencySet;

use super::{
    WorthUiFlowLayoutReceipt, WorthUiPrimitiveAppearanceReceipt, WorthUiPrimitiveContainerReceipt,
    WorthUiPrimitiveContentReceipt, WorthUiPrimitiveEventGeometryReceipt,
    WorthUiPrimitiveInteractionReceipt, WorthUiPrimitiveMeasurementReceipt,
    WorthUiPrimitiveMotionReceipt,
};

pub(super) fn primitive_receipt_digest(
    construction_graph_digest: u64,
    authored_digest: u64,
    flow_digest: u64,
    content_digest: u64,
    appearance_state_digest: u64,
    interaction_digest: u64,
    event_geometry_digest: u64,
    dependencies: &WorthUiProjectionDependencySet,
    container: &WorthUiPrimitiveContainerReceipt,
    measurement: &WorthUiPrimitiveMeasurementReceipt,
    content: &WorthUiPrimitiveContentReceipt,
    appearance: &WorthUiPrimitiveAppearanceReceipt,
    appearance_state: &crate::runtime::WorthUiStatefulAppearanceRecipeReceipt,
    interaction: &WorthUiPrimitiveInteractionReceipt,
    event_geometry: &WorthUiPrimitiveEventGeometryReceipt,
    motion: &WorthUiPrimitiveMotionReceipt,
    flow_layout: &WorthUiFlowLayoutReceipt,
) -> u64 {
    let content_item_count = content.items().len();
    let basis = format!(
        "primitive|construction_graph:{construction_graph_digest}|authored:{authored_digest}|flow:{flow_digest}|content:{content_digest}|state:{appearance_state_digest}|interaction_admission:{interaction_digest}|event_geometry:{event_geometry_digest}|deps:{}|align:{:?}|padding:{}:{}|radius:{}:{}|text:{}|items:{}|content_receipt:{}|bg:{}|fg:{}|state_receipt:{}|interaction:{:?}:{:?}:{:?}:{:?}:{}:{}|operability:{:?}:{:?}|affordance:{:?}:{:?}|event_geometry_receipt:{}:{:?}:{:?}:{:?}:{:?}|motion:{:?}:{:?}:{}:{}:{:?}|flow_receipt:{}",
        dependencies.digest().value(),
        container.align(),
        measurement.padding().token(),
        measurement.padding().edges().digest_basis(),
        measurement.radius().token(),
        measurement.radius().points(),
        content.text(),
        content_item_count,
        content.receipt_digest(),
        appearance.background_color().hex_triplet(),
        appearance.foreground_color().hex_triplet(),
        appearance_state.receipt_digest(),
        interaction.kind(),
        interaction.focus(),
        interaction.operability().posture(),
        interaction.selection_posture(),
        interaction.interaction_id(),
        interaction.submit_payload().digest(),
        interaction.operability().posture(),
        interaction.operability().basis(),
        interaction.affordance().cursor(),
        interaction.affordance().activation_posture(),
        event_geometry.receipt_digest(),
        event_geometry.cursor(),
        event_geometry.hit_area(),
        event_geometry.containment(),
        event_geometry.capture(),
        motion.kind(),
        motion.target(),
        motion.duration().token(),
        motion.duration().points(),
        motion.easing(),
        flow_layout.receipt_digest()
    );
    basis.bytes().fold(0xcbf2_9ce4_8422_2325, |mut acc, byte| {
        acc ^= u64::from(byte);
        acc.wrapping_mul(0x0000_0100_0000_01b3)
    })
}
