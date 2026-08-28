#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiIntentRouteResolutionStop {
    ApplicationGenerationChanged,
    CommandInvocationBasisMissing,
    CommandDestinationUnrouted {
        intent: crate::capability::UiIntentId,
    },
    CommandDestinationAmbiguous {
        intent: crate::capability::UiIntentId,
        candidates: usize,
    },
    CommandFocusedTargetMismatch {
        expected: crate::graph::UiGraphNodeIdentity,
        observed: crate::graph::UiGraphNodeIdentity,
    },
    Targeting(crate::runtime::interaction::UiInteractionTargetingDenial),
    Unrouted {
        graph_node: crate::graph::UiGraphNodeIdentity,
        interaction: crate::capability::UiSemanticInteractionFamily,
    },
}
