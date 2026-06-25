use worth_ui::facade::WorthUiEffectiveViewportParticipationReceipt;

pub(super) fn node_visible(
    viewport: Option<&WorthUiEffectiveViewportParticipationReceipt>,
    child_id: &str,
) -> bool {
    viewport
        .and_then(|receipt| receipt.row_for_node(child_id))
        .is_none_or(|row| row.visible())
}
