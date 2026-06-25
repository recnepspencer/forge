use crate::capability::SurfaceId;
use crate::runtime::{
    WorthUiGraphFactDerivationKind, WorthUiGraphInvalidationRequest,
    WorthUiPrimitiveConstructionFamily, WorthUiPrimitiveConstructionRequest,
    WorthUiQueryGraphObligationSemantic, WorthUiRuntimeFactFamily, WorthUiRuntimeFactId,
    WorthUiRuntimeFactSet, WorthUiRuntimeGraphAuthority,
};

#[test]
fn primitive_construction_plan_uses_query_graph_execution_rows() {
    let surface_id =
        SurfaceId::new("worth.surface.preview.primitive.proof").expect("valid surface id");
    let plan = WorthUiRuntimeGraphAuthority::new()
        .plan_primitive_construction(WorthUiPrimitiveConstructionRequest::for_surface(
            surface_id.clone(),
        ))
        .expect("primitive construction plan admits");
    let semantics = plan
        .query_graph_execution()
        .rows()
        .iter()
        .map(|row| row.semantic())
        .collect::<Vec<_>>();

    assert_eq!(plan.query_graph_execution().selected_obligation_count(), 4);
    for expected in WorthUiQueryGraphObligationSemantic::PRIMITIVE_CONSTRUCTION {
        assert!(
            semantics.contains(&expected),
            "missing primitive construction query semantic {expected:?}"
        );
    }
}

#[test]
fn primitive_construction_plan_publishes_dependency_contract_from_fact_graph() {
    let surface_id =
        SurfaceId::new("worth.surface.preview.primitive.proof").expect("valid surface id");
    let plan = WorthUiRuntimeGraphAuthority::new()
        .plan_primitive_construction(WorthUiPrimitiveConstructionRequest::for_surface(
            surface_id.clone(),
        ))
        .expect("primitive construction plan admits");
    let dependencies = plan.dependency_contract().dependencies();

    assert!(dependencies.facts().any(|fact| {
        fact.family() == WorthUiRuntimeFactFamily::AuthoredSurfaceProps
            && fact.identity() == surface_id.as_str()
    }));
    assert!(dependencies.facts().any(|fact| {
        fact.family() == WorthUiRuntimeFactFamily::PrimitiveConstruction
            && fact.identity() == surface_id.as_str()
    }));
}

#[test]
fn primitive_construction_plan_selects_family_admissions_before_resolution() {
    let surface_id =
        SurfaceId::new("worth.surface.preview.primitive.proof").expect("valid surface id");
    let plan = WorthUiRuntimeGraphAuthority::new()
        .plan_primitive_construction(WorthUiPrimitiveConstructionRequest::for_surface(
            surface_id.clone(),
        ))
        .expect("primitive construction plan admits");

    assert_eq!(plan.surface_id(), &surface_id);
    for family in [
        WorthUiPrimitiveConstructionFamily::BasePrimitive,
        WorthUiPrimitiveConstructionFamily::FlowLayout,
        WorthUiPrimitiveConstructionFamily::Content,
        WorthUiPrimitiveConstructionFamily::AppearanceState,
        WorthUiPrimitiveConstructionFamily::Interaction,
        WorthUiPrimitiveConstructionFamily::EventGeometry,
    ] {
        assert!(
            plan.family_selection().requires(family),
            "primitive construction plan did not select {family:?}"
        );
    }
    assert!(plan
        .dependency_contract()
        .dependencies()
        .facts()
        .any(|fact| fact.family() == WorthUiRuntimeFactFamily::PrimitiveConstruction));
}

#[test]
fn primitive_content_invalidation_reaches_draw_plan_and_event_region_through_graph_edges() {
    let surface = "worth.surface.preview.primitive.proof";
    let receipt = WorthUiRuntimeGraphAuthority::new().plan_invalidation(
        WorthUiGraphInvalidationRequest::from_authoritative_changed_facts(
            WorthUiRuntimeFactSet::single(WorthUiRuntimeFactId::primitive_content(surface)),
        ),
    );

    assert!(receipt
        .affected_facts()
        .contains_exact(&WorthUiRuntimeFactId::primitive_content(surface)));
    assert!(receipt
        .affected_facts()
        .contains_exact(&WorthUiRuntimeFactId::primitive_draw_plan(surface)));
    assert!(receipt
        .affected_facts()
        .contains_exact(&WorthUiRuntimeFactId::primitive_event_region(surface)));
    assert!(receipt.traversed_edges().iter().any(|edge| {
        edge.derivation() == WorthUiGraphFactDerivationKind::PrimitiveDrawPlan
            && edge.source() == &WorthUiRuntimeFactId::primitive_content(surface)
            && edge.target() == &WorthUiRuntimeFactId::primitive_draw_plan(surface)
    }));
    assert!(receipt.traversed_edges().iter().any(|edge| {
        edge.derivation() == WorthUiGraphFactDerivationKind::PrimitiveEventRegion
            && edge.source() == &WorthUiRuntimeFactId::primitive_draw_plan(surface)
            && edge.target() == &WorthUiRuntimeFactId::primitive_event_region(surface)
    }));
    assert_eq!(receipt.counters().authoritative_fact_count(), 1);
    assert_eq!(receipt.counters().registry_count(), 1);
}

#[test]
fn primitive_event_geometry_invalidation_reaches_event_region_only() {
    let surface = "worth.surface.preview.primitive.proof";
    let receipt = WorthUiRuntimeGraphAuthority::new().plan_invalidation(
        WorthUiGraphInvalidationRequest::from_authoritative_changed_facts(
            WorthUiRuntimeFactSet::single(WorthUiRuntimeFactId::primitive_event_geometry(surface)),
        ),
    );

    assert!(receipt
        .affected_facts()
        .contains_exact(&WorthUiRuntimeFactId::primitive_event_geometry(surface)));
    assert!(receipt
        .affected_facts()
        .contains_exact(&WorthUiRuntimeFactId::primitive_event_region(surface)));
    assert!(!receipt
        .affected_facts()
        .contains_exact(&WorthUiRuntimeFactId::primitive_draw_plan(surface)));
    assert!(receipt
        .traversed_edges()
        .iter()
        .all(|edge| { edge.derivation() != WorthUiGraphFactDerivationKind::PrimitiveDrawPlan }));
}

#[test]
fn primitive_appearance_state_invalidation_reaches_active_appearance_through_graph_edges() {
    let surface = "worth.surface.preview.primitive.proof";
    let receipt = WorthUiRuntimeGraphAuthority::new().plan_invalidation(
        WorthUiGraphInvalidationRequest::from_authoritative_changed_facts(
            WorthUiRuntimeFactSet::single(WorthUiRuntimeFactId::primitive_appearance_state(
                surface,
            )),
        ),
    );

    assert!(receipt
        .affected_facts()
        .contains_exact(&WorthUiRuntimeFactId::primitive_appearance_state(surface)));
    assert!(receipt
        .affected_facts()
        .contains_exact(&WorthUiRuntimeFactId::primitive_active_appearance(surface)));
    assert!(receipt
        .affected_facts()
        .contains_exact(&WorthUiRuntimeFactId::primitive_construction(surface)));
    assert!(receipt.traversed_edges().iter().any(|edge| {
        edge.derivation() == WorthUiGraphFactDerivationKind::PrimitiveActiveAppearance
            && edge.source() == &WorthUiRuntimeFactId::primitive_appearance_state(surface)
            && edge.target() == &WorthUiRuntimeFactId::primitive_active_appearance(surface)
    }));
}
