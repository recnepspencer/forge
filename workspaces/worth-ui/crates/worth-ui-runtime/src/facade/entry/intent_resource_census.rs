use super::WorthUiActiveApplicationSession;

impl WorthUiActiveApplicationSession {
    pub(super) fn intent_resource_census(&self) -> crate::runtime::session::UiIntentResourceCensus {
        crate::runtime::session::UiIntentResourceCensus::from_owners(
            crate::runtime::session::UiIntentResourceCensusInput {
                observation: self.application.observation_resource_snapshot(),
                interaction: self.interaction.snapshot(),
                confirmation: self.intent_confirmation.metrics(),
                execution: self.intent_execution.census(),
                evidence: self.intent_evidence.snapshot(),
            },
        )
    }

    #[cfg(any(test, feature = "certification-support"))]
    pub(crate) fn snapshot_intent_resources_for_certification(
        &self,
    ) -> crate::runtime::session::UiIntentResourceCensus {
        self.intent_resource_census()
    }
}
