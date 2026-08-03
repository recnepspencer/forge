use super::WorthUiActiveApplicationSession;

impl WorthUiActiveApplicationSession {
    pub fn prepare_intent_payload(
        &mut self,
        route: crate::facade::intent::UiResolvedProductIntentRoute,
    ) -> Result<
        crate::facade::intent::UiPreparedIntentPayload,
        crate::facade::intent::UiIntentPayloadStop,
    > {
        crate::runtime::intent::prepare_intent_payload(
            route,
            self.application
                .prepared_authority()
                .capabilities()
                .intent_definitions(),
            self.application.generation_identity(),
            &self.mounted,
            &self.intent_application_facts,
        )
    }
}
