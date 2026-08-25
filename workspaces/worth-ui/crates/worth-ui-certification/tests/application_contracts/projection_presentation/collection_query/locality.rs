use worth_ui_host_contract::UiSemanticTextSlot;

pub(super) fn assert_content_update_is_local(
    before: &worth_ui_host_headless::UiHeadlessMountedFrameTranscript,
    after: &worth_ui_host_headless::UiHeadlessMountedFrameTranscript,
) {
    let before_value = before
        .semantic_text()
        .iter()
        .find(|row| row.text() == "Bravo")
        .expect("predecessor value row");
    let after_value = after
        .semantic_text()
        .iter()
        .find(|row| row.text() == "Bravo updated")
        .expect("successor value row");
    assert_ne!(
        before_value.layout_identity(),
        after_value.layout_identity()
    );
    let before_posture = before
        .semantic_text()
        .iter()
        .find(|row| row.slot() == UiSemanticTextSlot::Posture)
        .expect("predecessor posture row");
    let after_posture = after
        .semantic_text()
        .iter()
        .find(|row| row.slot() == UiSemanticTextSlot::Posture)
        .expect("successor posture row");
    assert_eq!(
        before_posture.layout_identity(),
        after_posture.layout_identity()
    );
    assert_eq!(
        after_value.qualified_layout_cost().analyzed_bytes(),
        "Bravo updated".len() as u32,
    );
}

#[test]
fn content_only_global_rescan_is_rejected_by_retained_layout_owners() {
    super::real_query_collection_snapshot_and_patch_publish_keyed_semantic_text();
}
