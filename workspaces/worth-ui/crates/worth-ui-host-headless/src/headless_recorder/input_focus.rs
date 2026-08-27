pub(super) fn install_recipient(
    recorder: &super::WorthUiHeadlessRecorder,
    binding: worth_ui_host_contract::UiHostInputRecipientBindingReceipt,
) -> bool {
    recorder
        .state
        .borrow_mut()
        .input_recipients
        .insert(binding.host_session(), binding);
    true
}

pub(super) fn clear_recipient(
    recorder: &super::WorthUiHeadlessRecorder,
    binding: worth_ui_host_contract::UiHostInputRecipientBindingReceipt,
) -> bool {
    let mut state = recorder.state.borrow_mut();
    if state.input_recipients.get(&binding.host_session()) != Some(&binding) {
        return false;
    }
    state.input_recipients.remove(&binding.host_session());
    true
}
