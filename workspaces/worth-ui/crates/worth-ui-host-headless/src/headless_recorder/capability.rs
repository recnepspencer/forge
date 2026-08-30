pub(super) fn report(
    recorder: &super::WorthUiHeadlessRecorder,
) -> worth_ui_host_contract::WorthUiHostCapabilityReport {
    let mut capabilities = vec![
        worth_ui_host_contract::WorthUiHostCapability::MountedFrameRecording,
        worth_ui_host_contract::WorthUiHostCapability::SemanticFocusPlacement,
    ];
    recorder
        .state
        .borrow()
        .measurement
        .append_capabilities(&mut capabilities);
    worth_ui_host_contract::WorthUiHostCapabilityReport::available(capabilities)
}
