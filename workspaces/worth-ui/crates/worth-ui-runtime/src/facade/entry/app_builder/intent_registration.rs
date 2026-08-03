use crate::capability::{UiIntent, UiIntentDefinition, UiIntentDefinitionRegistrationError};

use super::WorthUiApplicationBuilder;

impl<ChangeProfileState> WorthUiApplicationBuilder<ChangeProfileState> {
    pub fn register_intent_definition<I: UiIntent>(
        mut self,
        definition: UiIntentDefinition<I>,
    ) -> Result<Self, UiIntentDefinitionRegistrationError> {
        self.inner = self.inner.register_intent_definition(definition)?;
        Ok(self)
    }
}
