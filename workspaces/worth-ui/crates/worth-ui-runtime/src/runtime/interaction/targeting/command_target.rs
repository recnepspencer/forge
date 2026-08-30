pub(crate) fn resolve_presented_command_target(
    mounted: &crate::mounting::WorthUiMountedSessionState,
    presentation: worth_ui_host_contract::UiHostObservationPresentationBasis,
    receipt: &crate::runtime::command_routing::UiCommandRouteReceipt,
) -> Result<super::UiPresentedInteractionTargetView, super::UiInteractionTargetingDenial> {
    if let Some(focused) = receipt
        .focused_target()
        .or_else(|| receipt.invocation_target())
    {
        return super::resolve_presented_focus_target(mounted, presentation, focused)?
            .ok_or(super::UiInteractionTargetingDenial::GraphTargetNotPresented);
    }
    super::resolve_presented_surface_target(mounted, presentation)
}
