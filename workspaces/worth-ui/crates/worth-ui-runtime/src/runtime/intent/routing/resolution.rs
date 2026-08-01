use super::{
    UiIntentRouteResolution, UiIntentRouteResolutionStop, UiResolvedConfirmationIntentRoute,
    UiResolvedProductIntentRoute,
};

pub(crate) fn resolve_intent_route(
    catalog: &crate::declaration::UiIntentCatalog,
    definitions: &crate::capability::FrozenIntentDefinitionCapabilities,
    generation: &crate::runtime::WorthUiActiveApplicationGenerationIdentity,
    mounted: &crate::mounting::WorthUiMountedSessionState,
    source: crate::runtime::interaction::UiIntentRouteSource,
) -> Result<UiIntentRouteResolution, UiIntentRouteResolutionStop> {
    let interaction = source.into_mounted_interaction();
    if interaction.generation() != generation {
        return Err(UiIntentRouteResolutionStop::ApplicationGenerationChanged);
    }
    let family = interaction.family();
    let affinity =
        crate::runtime::interaction::targeting::admit_current_target(mounted, interaction.target())
            .map_err(UiIntentRouteResolutionStop::Targeting)?;
    let graph_node = affinity.graph_node();
    let (route, cost) =
        catalog
            .lookup(graph_node, family)
            .ok_or(UiIntentRouteResolutionStop::Unrouted {
                graph_node,
                interaction: family,
            })?;
    Ok(match route {
        crate::declaration::UiIntentCatalogResolvedRoute::Product { route, declaration } => {
            UiIntentRouteResolution::Product(UiResolvedProductIntentRoute::new(
                super::UiResolvedProductIntentRouteInput {
                    graph_node: route.graph_node(),
                    interaction: route.interaction(),
                    definition_id: definitions.definition_at(declaration.definition()).id(),
                    declaration,
                    source: interaction,
                    cost,
                },
            ))
        }
        crate::declaration::UiIntentCatalogResolvedRoute::Confirmation { route, declaration } => {
            UiIntentRouteResolution::Confirmation(UiResolvedConfirmationIntentRoute::new(
                super::UiResolvedConfirmationIntentRouteInput {
                    graph_node: route.graph_node(),
                    definition_id: definitions.definition_at(declaration.definition()).id(),
                    declaration,
                    source: interaction,
                    cost,
                },
            ))
        }
    })
}
