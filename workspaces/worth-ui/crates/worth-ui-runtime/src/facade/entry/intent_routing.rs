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
        crate::runtime::intent::resolve_intent_route(
            self.application.prepared_authority().intent_catalog(),
            self.application
                .prepared_authority()
                .capabilities()
                .intent_definitions(),
            &self.mounted,
            source,
        )
    }
}
