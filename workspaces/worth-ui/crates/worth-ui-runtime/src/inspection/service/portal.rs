pub(crate) fn why_portal_closed(
    owner: Option<&crate::runtime::portal::UiPortalRuntimeState>,
) -> Option<worth_ui_inspection::UiPortalClosedInspectionSummary> {
    let record = owner?.last_closed()?;
    Some(worth_ui_inspection::UiPortalClosedInspectionSummary::new(
        worth_ui_inspection::UiRuntimeServiceInspectionSource::new(
            worth_ui_inspection::UiRuntimeServiceInspectionFamily::Portal,
            Some(record.portal().diagnostic_value()),
            record.revision(),
        ),
        match record.cause() {
            crate::runtime::portal::UiPortalDismissalCause::Escape => {
                worth_ui_inspection::UiPortalClosedInspectionReason::Escape
            }
            crate::runtime::portal::UiPortalDismissalCause::OutsidePress => {
                worth_ui_inspection::UiPortalClosedInspectionReason::OutsidePress
            }
            crate::runtime::portal::UiPortalDismissalCause::AcceptedSelection => {
                worth_ui_inspection::UiPortalClosedInspectionReason::AcceptedSelection
            }
            crate::runtime::portal::UiPortalDismissalCause::ExplicitOwnerRequest => {
                worth_ui_inspection::UiPortalClosedInspectionReason::ExplicitOwnerRequest
            }
            crate::runtime::portal::UiPortalDismissalCause::AnchorLoss => {
                worth_ui_inspection::UiPortalClosedInspectionReason::AnchorLoss
            }
            crate::runtime::portal::UiPortalDismissalCause::ParentClosed => {
                worth_ui_inspection::UiPortalClosedInspectionReason::ParentClosed
            }
            crate::runtime::portal::UiPortalDismissalCause::OwnerLoss => {
                worth_ui_inspection::UiPortalClosedInspectionReason::OwnerLoss
            }
            crate::runtime::portal::UiPortalDismissalCause::ApplicationShutdown => {
                worth_ui_inspection::UiPortalClosedInspectionReason::ApplicationShutdown
            }
            crate::runtime::portal::UiPortalDismissalCause::WindowFocusPolicy => {
                worth_ui_inspection::UiPortalClosedInspectionReason::WindowFocusPolicy
            }
        },
        record.closed_descendants(),
        worth_ui_inspection::UiRuntimeServiceInspectionCost::latest_record(1, 1),
    ))
}
