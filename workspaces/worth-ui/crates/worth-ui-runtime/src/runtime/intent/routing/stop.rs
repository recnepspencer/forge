#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiIntentRouteResolutionStop {
    ApplicationGenerationChanged,
    Targeting(crate::runtime::interaction::UiInteractionTargetingDenial),
    Unrouted {
        graph_node: crate::graph::UiGraphNodeIdentity,
        interaction: crate::capability::UiSemanticInteractionFamily,
    },
}
