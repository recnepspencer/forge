pub(crate) fn why_command_won(
    owner: Option<&crate::runtime::command_routing::UiCommandRoutingRuntimeState>,
) -> Option<worth_ui_inspection::UiCommandWonInspectionSummary> {
    let record = owner?.last_winner()?;
    let projected_items = u16::try_from(record.losers().len())
        .unwrap_or(u16::MAX)
        .saturating_add(1);
    let losers = record
        .losers()
        .iter()
        .map(|(command, reason)| worth_ui_inspection::UiCommandRouteLossInspection::new(
            command.clone(),
            match reason {
                crate::runtime::command_routing::UiCommandRouteLossReason::LowerScopePrecedence => worth_ui_inspection::UiCommandRouteLossInspectionReason::LowerScopePrecedence,
                crate::runtime::command_routing::UiCommandRouteLossReason::LowerDeclaredPriority => worth_ui_inspection::UiCommandRouteLossInspectionReason::LowerDeclaredPriority,
                crate::runtime::command_routing::UiCommandRouteLossReason::LowerSpecificity => worth_ui_inspection::UiCommandRouteLossInspectionReason::LowerSpecificity,
            },
        ))
        .collect::<Vec<_>>()
        .into_boxed_slice();
    Some(worth_ui_inspection::UiCommandWonInspectionSummary::new(
        worth_ui_inspection::UiRuntimeServiceInspectionSource::new(
            worth_ui_inspection::UiRuntimeServiceInspectionFamily::CommandRouting,
            None,
            record.invocation(),
        ),
        record.command().to_owned(),
        match record.scope() {
            crate::capability::UiCommandRouteScope::Application => {
                worth_ui_inspection::UiCommandRouteScopeInspection::Application
            }
            crate::capability::UiCommandRouteScope::Surface => {
                worth_ui_inspection::UiCommandRouteScopeInspection::Surface
            }
            crate::capability::UiCommandRouteScope::ActiveRegion => {
                worth_ui_inspection::UiCommandRouteScopeInspection::ActiveRegion
            }
            crate::capability::UiCommandRouteScope::FocusedControl => {
                worth_ui_inspection::UiCommandRouteScopeInspection::FocusedControl
            }
            crate::capability::UiCommandRouteScope::ActivePortal => {
                worth_ui_inspection::UiCommandRouteScopeInspection::ActivePortal
            }
        },
        losers,
        worth_ui_inspection::UiRuntimeServiceInspectionCost::latest_record_with_projection(
            1,
            1,
            projected_items,
        ),
    ))
}
