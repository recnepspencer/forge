use super::UiNativeInputObservationState;

impl UiNativeInputObservationState {
    pub(crate) fn install_input_recipient(
        &mut self,
        binding: worth_ui_host_contract::UiHostInputRecipientBindingReceipt,
    ) -> bool {
        if self.active_host_session != Some(binding.host_session()) {
            return false;
        }
        self.input_recipient = Some(binding);
        true
    }

    pub(crate) fn clear_input_recipient(
        &mut self,
        binding: worth_ui_host_contract::UiHostInputRecipientBindingReceipt,
    ) -> bool {
        if self.input_recipient != Some(binding) {
            return false;
        }
        self.input_recipient = None;
        true
    }

    pub(in crate::native::input) fn current_input_recipient(
        &self,
    ) -> Option<worth_ui_host_contract::UiHostInputRecipientBindingReceipt> {
        let recipient = self.input_recipient?;
        let (_, host_session, presentation) = self.completed?;
        (recipient.host_session() == host_session && recipient.binding() == presentation.binding())
            .then_some(recipient)
    }
}
