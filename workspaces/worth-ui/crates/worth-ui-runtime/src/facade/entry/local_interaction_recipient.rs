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
        self.interaction
            .bind_local_recipient(activation, &generation, contract, &self.mounted)
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
