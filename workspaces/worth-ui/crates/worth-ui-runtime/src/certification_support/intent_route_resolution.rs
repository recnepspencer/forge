pub trait WorthUiIntentRouteResolutionCertificationExt {
    fn intent_route_resolution_cost_for_certification(
        &self,
        graph_node: crate::graph::UiGraphNodeIdentity,
        interaction: crate::capability::UiSemanticInteractionFamily,
    ) -> Option<crate::declaration::UiIntentRouteResolutionCost>;
}

impl WorthUiIntentRouteResolutionCertificationExt for crate::facade::WorthUiApp {
    fn intent_route_resolution_cost_for_certification(
        &self,
        graph_node: crate::graph::UiGraphNodeIdentity,
        interaction: crate::capability::UiSemanticInteractionFamily,
    ) -> Option<crate::declaration::UiIntentRouteResolutionCost> {
        self.prepared_authority()
            .intent_catalog()
            .lookup(graph_node, interaction)
            .map(|(_, cost)| cost)
    }
}
