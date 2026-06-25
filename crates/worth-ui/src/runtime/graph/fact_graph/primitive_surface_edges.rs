use crate::runtime::{WorthUiRuntimeFactFamily, WorthUiRuntimeFactId};

use super::{WorthUiGraphDependencyEdge, WorthUiGraphFactDerivationKind, WorthUiGraphFactRegistry};

pub(super) fn primitive_surface_edges(surface_id: &str) -> Vec<WorthUiGraphDependencyEdge> {
    let construction = WorthUiRuntimeFactId::primitive_construction(surface_id);
    let content = WorthUiRuntimeFactId::primitive_content(surface_id);
    let container = WorthUiRuntimeFactId::primitive_container(surface_id);
    let measurement = WorthUiRuntimeFactId::primitive_measurement(surface_id);
    let appearance = WorthUiRuntimeFactId::primitive_appearance(surface_id);
    let appearance_state = WorthUiRuntimeFactId::primitive_appearance_state(surface_id);
    let active_appearance = WorthUiRuntimeFactId::primitive_active_appearance(surface_id);
    let interaction = WorthUiRuntimeFactId::primitive_interaction(surface_id);
    let motion = WorthUiRuntimeFactId::primitive_motion(surface_id);
    let flow_layout = WorthUiRuntimeFactId::primitive_flow_layout(surface_id);
    let draw_plan = WorthUiRuntimeFactId::primitive_draw_plan(surface_id);
    let event_geometry = WorthUiRuntimeFactId::primitive_event_geometry(surface_id);
    let event_region = WorthUiRuntimeFactId::primitive_event_region(surface_id);

    let primitive_families = [
        content.clone(),
        container,
        measurement,
        appearance,
        appearance_state.clone(),
        active_appearance.clone(),
        interaction.clone(),
        motion,
        flow_layout.clone(),
        event_geometry.clone(),
    ];
    let mut edges = Vec::new();
    for fact in primitive_families {
        edges.push(WorthUiGraphDependencyEdge::new(
            fact,
            construction.clone(),
            WorthUiGraphFactDerivationKind::PrimitiveConstruction,
        ));
    }
    edges.push(WorthUiGraphDependencyEdge::new(
        appearance_state,
        active_appearance.clone(),
        WorthUiGraphFactDerivationKind::PrimitiveActiveAppearance,
    ));
    edges.push(WorthUiGraphDependencyEdge::new(
        interaction,
        active_appearance,
        WorthUiGraphFactDerivationKind::PrimitiveActiveAppearance,
    ));
    edges.push(WorthUiGraphDependencyEdge::new(
        content,
        draw_plan.clone(),
        WorthUiGraphFactDerivationKind::PrimitiveDrawPlan,
    ));
    edges.push(WorthUiGraphDependencyEdge::new(
        flow_layout,
        draw_plan.clone(),
        WorthUiGraphFactDerivationKind::PrimitiveDrawPlan,
    ));
    edges.push(WorthUiGraphDependencyEdge::new(
        draw_plan,
        event_region.clone(),
        WorthUiGraphFactDerivationKind::PrimitiveEventRegion,
    ));
    edges.push(WorthUiGraphDependencyEdge::new(
        event_geometry,
        event_region,
        WorthUiGraphFactDerivationKind::PrimitiveEventRegion,
    ));
    edges
}

pub(crate) fn graph_registry_for_fact(
    fact: &WorthUiRuntimeFactId,
) -> Option<WorthUiGraphFactRegistry> {
    match fact.family() {
        WorthUiRuntimeFactFamily::PrimitiveConstruction
        | WorthUiRuntimeFactFamily::PrimitiveContent
        | WorthUiRuntimeFactFamily::PrimitiveContainer
        | WorthUiRuntimeFactFamily::PrimitiveMeasurement
        | WorthUiRuntimeFactFamily::PrimitiveAppearance
        | WorthUiRuntimeFactFamily::PrimitiveAppearanceState
        | WorthUiRuntimeFactFamily::PrimitiveActiveAppearance
        | WorthUiRuntimeFactFamily::PrimitiveInteraction
        | WorthUiRuntimeFactFamily::PrimitiveMotion
        | WorthUiRuntimeFactFamily::PrimitiveFlowLayout
        | WorthUiRuntimeFactFamily::PrimitiveDrawPlan
        | WorthUiRuntimeFactFamily::PrimitiveEventGeometry
        | WorthUiRuntimeFactFamily::PrimitiveEventRegion => Some(
            WorthUiGraphFactRegistry::for_primitive_surface(fact.identity()),
        ),
        _ => None,
    }
}
