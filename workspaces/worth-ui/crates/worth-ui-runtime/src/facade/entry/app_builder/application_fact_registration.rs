use std::sync::Arc;

use crate::capability::{UiIntentBoolean, UiIntentText, UiIntentUnsigned64};
use crate::declaration::{UiIntentApplicationFact, UiIntentApplicationFactRegistrationError};

use super::WorthUiApplicationBuilder;

impl<ChangeProfileState> WorthUiApplicationBuilder<ChangeProfileState> {
    pub fn register_intent_text_fact(
        mut self,
        fact: UiIntentApplicationFact<UiIntentText>,
        initial: impl Into<Arc<str>>,
    ) -> Result<Self, UiIntentApplicationFactRegistrationError> {
        self.intent_application_facts.register_text(fact, initial)?;
        Ok(self)
    }

    pub fn register_intent_boolean_fact(
        mut self,
        fact: UiIntentApplicationFact<UiIntentBoolean>,
        initial: bool,
    ) -> Result<Self, UiIntentApplicationFactRegistrationError> {
        self.intent_application_facts
            .register_boolean(fact, initial)?;
        Ok(self)
    }

    pub fn register_intent_unsigned64_fact(
        mut self,
        fact: UiIntentApplicationFact<UiIntentUnsigned64>,
        initial: u64,
    ) -> Result<Self, UiIntentApplicationFactRegistrationError> {
        self.intent_application_facts
            .register_unsigned64(fact, initial)?;
        Ok(self)
    }
}
