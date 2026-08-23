use crate::runtime::interaction::{
    UiActivateInteraction, UiLocalInputRecipientAdmission, UiLocalInputRecipientBindingStop,
    UiLocalInputRecipientContract,
};

use super::WorthUiActiveApplicationSession;

impl WorthUiActiveApplicationSession {
    pub fn bind_local_input_recipient(
        &mut self,
        activation: UiActivateInteraction,
        contract: UiLocalInputRecipientContract,
    ) -> Result<UiLocalInputRecipientAdmission, UiLocalInputRecipientBindingStop> {
        let generation = self.active_generation_identity();
        let context = crate::runtime::interaction::draft::UiLocalInputRecipientBindingContext::new(
            self.host_session.identity().as_u64(),
            self.interaction.application_generation(),
            &generation,
            &self.mounted,
        );
        let host = &self.host_session;
        self.interaction
            .bind_local_recipient(activation, context, contract, |binding| {
                host.install_input_recipient(binding)
            })
    }
}

#[cfg(any(test, feature = "certification-support"))]
pub trait WorthUiLocalInputRecipientCertificationExt {
    fn bind_local_input_recipient(
        &mut self,
        activation: UiActivateInteraction,
        contract: UiLocalInputRecipientContract,
    ) -> Result<UiLocalInputRecipientAdmission, UiLocalInputRecipientBindingStop>;
}

#[cfg(any(test, feature = "certification-support"))]
impl WorthUiLocalInputRecipientCertificationExt for WorthUiActiveApplicationSession {
    fn bind_local_input_recipient(
        &mut self,
        activation: UiActivateInteraction,
        contract: UiLocalInputRecipientContract,
    ) -> Result<UiLocalInputRecipientAdmission, UiLocalInputRecipientBindingStop> {
        WorthUiActiveApplicationSession::bind_local_input_recipient(self, activation, contract)
    }
}
