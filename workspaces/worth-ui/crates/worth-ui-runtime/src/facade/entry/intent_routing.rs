use super::WorthUiActiveApplicationSession;

impl WorthUiActiveApplicationSession {
    pub fn intent_catalog_metrics(&self) -> crate::facade::intent::UiIntentCatalogMetrics {
        self.application
            .prepared_authority()
            .intent_catalog_metrics()
    }

    pub fn resolve_intent_route(
        &self,
        source: crate::facade::interaction::UiIntentRouteSource,
    ) -> Result<
        crate::facade::intent::UiIntentRouteResolution,
        crate::facade::intent::UiIntentRouteResolutionStop,
    > {
        let mounted_evidence_input = source.evidence_input();
        let generation = self.active_generation_identity();
        let resolution = {
            let prepared = self.application.prepared_authority();
            crate::runtime::intent::resolve_intent_route(
                prepared.intent_catalog(),
                prepared.capabilities().intent_definitions(),
                &generation,
                &self.mounted,
                source,
            )
        }?;
        let evidence_reference = mounted_evidence_input
            .and_then(|input| self.intent_evidence.reference_for_input(input))
            .or_else(|| resolution.command_evidence_reference());
        Ok(resolution.with_evidence_reference(evidence_reference))
    }

    pub fn update_intent_text_fact(
        &mut self,
        fact: &crate::facade::intent::UiIntentApplicationFact<crate::facade::intent::UiIntentText>,
        value: impl Into<std::sync::Arc<str>>,
    ) -> Result<
        crate::facade::intent::UiIntentApplicationFactUpdateReceipt,
        crate::facade::intent::UiIntentApplicationFactUpdateDenial,
    > {
        self.intent_application_facts.update_text(fact, value)
    }

    pub fn update_intent_boolean_fact(
        &mut self,
        fact: &crate::facade::intent::UiIntentApplicationFact<
            crate::facade::intent::UiIntentBoolean,
        >,
        value: bool,
    ) -> Result<
        crate::facade::intent::UiIntentApplicationFactUpdateReceipt,
        crate::facade::intent::UiIntentApplicationFactUpdateDenial,
    > {
        self.intent_application_facts.update_boolean(fact, value)
    }

    pub fn update_intent_unsigned64_fact(
        &mut self,
        fact: &crate::facade::intent::UiIntentApplicationFact<
            crate::facade::intent::UiIntentUnsigned64,
        >,
        value: u64,
    ) -> Result<
        crate::facade::intent::UiIntentApplicationFactUpdateReceipt,
        crate::facade::intent::UiIntentApplicationFactUpdateDenial,
    > {
        self.intent_application_facts.update_unsigned64(fact, value)
    }
}
