pub(crate) fn why_selection_dropped(
    owner: Option<&crate::runtime::selection::UiSelectionRuntimeState>,
) -> Option<worth_ui_inspection::UiSelectionDroppedInspectionSummary> {
    let record = owner?.last_drop()?;
    let identity = record.owner().semantic_surface().diagnostic_value()
        ^ record.owner().graph_node().digest().rotate_left(17)
        ^ record
            .owner()
            .key_family()
            .diagnostic_value()
            .rotate_left(37);
    Some(worth_ui_inspection::UiSelectionDroppedInspectionSummary::new(
        worth_ui_inspection::UiRuntimeServiceInspectionSource::new(
            worth_ui_inspection::UiRuntimeServiceInspectionFamily::Selection,
            Some(identity),
            record.revision(),
        ),
        match record.reason() {
            crate::runtime::selection::UiSelectionDropInspectionReason::Interaction => worth_ui_inspection::UiSelectionDropInspectionReason::Interaction,
            crate::runtime::selection::UiSelectionDropInspectionReason::CatalogReconciliation => worth_ui_inspection::UiSelectionDropInspectionReason::CatalogReconciliation,
        },
        record.removed_count(),
        record.selected_count(),
        worth_ui_inspection::UiRuntimeServiceInspectionCost::latest_record(1, 1),
    ))
}
