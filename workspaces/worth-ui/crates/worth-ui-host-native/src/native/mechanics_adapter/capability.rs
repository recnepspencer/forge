pub(super) fn report() -> worth_ui_host_contract::WorthUiHostCapabilityReport {
    use worth_ui_host_contract::WorthUiHostCapability;

    worth_ui_host_contract::WorthUiHostCapabilityReport::available(vec![
        WorthUiHostCapability::ViewportObservation,
        WorthUiHostCapability::DpiObservation,
        WorthUiHostCapability::PointerInput,
        WorthUiHostCapability::KeyboardInput,
        WorthUiHostCapability::TextInput,
        WorthUiHostCapability::Ime,
        WorthUiHostCapability::NativePaint,
        WorthUiHostCapability::IdentityOverlay,
        WorthUiHostCapability::SemanticFocusPlacement,
    ])
}
