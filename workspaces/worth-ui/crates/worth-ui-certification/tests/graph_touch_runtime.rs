use worth_ui::facade::graph::{
    UiGraphTouchAspectPosture, UiGraphTouchAspects, UiGraphTouchDenial, UiGraphTouchOriginClass,
    UiGraphTouchRuntimeLane, UiGraphTouchTargetClass, UiGraphTouchTiming, UiGraphWorldProfile,
    WorthQuerySessionLabel,
};

#[path = "fixtures/graph_touch_support.rs"]
mod graph_touch_support;

use graph_touch_support::{
    control_artifact, graph_node_identity, mosaic_artifact, mounted_receipt_transition,
    query_snapshot_world_profile, region_artifact, touch_app,
};

#[test]
fn equivalent_touches_converge_to_one_canonical_descriptor() {
    let app = touch_app(UiGraphWorldProfile::authoritative());
    let graph = app.graph();
    let transition = mounted_receipt_transition(&app, control_artifact(&app));
    let origin = graph
        .touches()
        .declaration_change_receipt(control_artifact(&app))
        .expect("declaration-backed touch origin should be admitted from graph-owned artifact");

    let left = graph
        .touches()
        .from_mounted_receipt_transition(
            origin.clone(),
            UiGraphTouchTiming::PostMutation,
            transition,
            UiGraphTouchAspects::new()
                .query_binding(UiGraphTouchAspectPosture::Invalidated)
                .measurement(UiGraphTouchAspectPosture::Preserved),
        )
        .expect("equivalent touch facts should admit one canonical descriptor");
    let right = graph
        .touches()
        .from_mounted_receipt_transition(
            origin,
            UiGraphTouchTiming::PostMutation,
            transition,
            UiGraphTouchAspects::new()
                .measurement(UiGraphTouchAspectPosture::Preserved)
                .query_binding(UiGraphTouchAspectPosture::Invalidated)
                .measurement(UiGraphTouchAspectPosture::Preserved),
        )
        .expect("duplicate equivalent facts should converge");

    assert_eq!(left, right);
    assert_eq!(left.identity_digest(), right.identity_digest());
    assert_eq!(
        left.origin().class(),
        UiGraphTouchOriginClass::DeclarationChange
    );
    assert_eq!(
        left.aspects()
            .iter()
            .map(|fact| fact.lane())
            .collect::<Vec<_>>(),
        vec![
            UiGraphTouchRuntimeLane::Measurement,
            UiGraphTouchRuntimeLane::QueryBinding,
        ]
    );
}

#[test]
fn coarse_or_contradictory_touch_construction_denies_before_selection() {
    let app = touch_app(UiGraphWorldProfile::authoritative());
    let graph = app.graph();
    let transition = mounted_receipt_transition(&app, control_artifact(&app));
    let origin = graph
        .touches()
        .declaration_change_receipt(control_artifact(&app))
        .expect("declaration-backed origin should exist");

    let missing_aspects = graph.touches().from_mounted_receipt_transition(
        origin.clone(),
        UiGraphTouchTiming::PostMutation,
        transition,
        UiGraphTouchAspects::new(),
    );
    let contradictory = graph.touches().from_mounted_receipt_transition(
        origin,
        UiGraphTouchTiming::PostMutation,
        transition,
        UiGraphTouchAspects::new()
            .measurement(UiGraphTouchAspectPosture::Read)
            .measurement(UiGraphTouchAspectPosture::Written),
    );

    assert!(matches!(
        missing_aspects,
        Err(UiGraphTouchDenial::MissingAspectPosture)
    ));
    assert!(matches!(
        contradictory,
        Err(UiGraphTouchDenial::ContradictoryAspectPosture {
            lane: UiGraphTouchRuntimeLane::Measurement,
            first: UiGraphTouchAspectPosture::Read,
            second: UiGraphTouchAspectPosture::Written,
        })
    ));
}

#[test]
fn ordinary_touch_authority_supports_precise_target_classes_and_origin_receipts() {
    let app = touch_app(UiGraphWorldProfile::authoritative());
    let graph = app.graph();
    let control_id = graph_node_identity(graph, control_artifact(&app));
    let region_id = graph_node_identity(graph, region_artifact(&app));
    let mosaic_id = graph_node_identity(graph, mosaic_artifact(&app));
    let control_origin = graph
        .touches()
        .declaration_change_receipt(control_artifact(&app))
        .expect("declaration changes must come from admitted artifacts");
    let region_origin = graph
        .touches()
        .declaration_change_receipt(region_artifact(&app))
        .expect("region declaration change must come from admitted artifacts");
    let mosaic_origin = graph
        .touches()
        .declaration_change_receipt(mosaic_artifact(&app))
        .expect("mosaic declaration change must come from admitted artifacts");

    let node_touch = graph
        .touches()
        .from_node(
            control_origin.clone(),
            UiGraphTouchTiming::PostMutation,
            control_id,
            UiGraphTouchAspects::new().structural(UiGraphTouchAspectPosture::Invalidated),
        )
        .expect("node touches should be graph-owned");
    let slot_touch = graph
        .touches()
        .from_slot_occupancy(
            control_origin.clone(),
            UiGraphTouchTiming::PostMutation,
            control_id,
            UiGraphTouchAspects::new().participation(UiGraphTouchAspectPosture::Written),
        )
        .expect("slot occupancy touches should derive from graph topology");
    let page_touch = graph
        .touches()
        .from_page_membership(
            control_origin,
            UiGraphTouchTiming::PreMutation,
            control_id,
            UiGraphTouchAspects::new().participation(UiGraphTouchAspectPosture::Read),
        )
        .expect("page membership touches should derive from graph topology");
    let region_touch = graph
        .touches()
        .from_region_membership(
            region_origin,
            UiGraphTouchTiming::PostMutation,
            region_id,
            UiGraphTouchAspects::new().structural(UiGraphTouchAspectPosture::Preserved),
        )
        .expect("region membership touches should derive from graph topology");
    let mosaic_touch = graph
        .touches()
        .from_mosaic_membership(
            mosaic_origin,
            UiGraphTouchTiming::ReplayEvaluation,
            mosaic_id,
            UiGraphTouchAspects::new().structural(UiGraphTouchAspectPosture::Invalidated),
        )
        .expect("mosaic membership touches should derive from graph topology");

    assert_eq!(
        node_touch.origin().class(),
        UiGraphTouchOriginClass::DeclarationChange
    );
    assert_eq!(node_touch.target().class(), UiGraphTouchTargetClass::Node);
    assert_eq!(
        slot_touch.origin().class(),
        UiGraphTouchOriginClass::DeclarationChange
    );
    assert_eq!(
        slot_touch.target().class(),
        UiGraphTouchTargetClass::SlotOccupancy
    );
    assert_eq!(slot_touch.target().slot_name(), Some("footer"));
    assert_eq!(
        page_touch.origin().class(),
        UiGraphTouchOriginClass::DeclarationChange
    );
    assert_eq!(
        page_touch.target().class(),
        UiGraphTouchTargetClass::PageMembership
    );
    assert!(page_touch.target().page_node_identity().is_some());
    assert_eq!(
        region_touch.origin().class(),
        UiGraphTouchOriginClass::DeclarationChange
    );
    assert_eq!(
        region_touch.target().class(),
        UiGraphTouchTargetClass::RegionMembership
    );
    assert_eq!(region_touch.target().region_name(), Some("sidebar"));
    assert_eq!(
        mosaic_touch.origin().class(),
        UiGraphTouchOriginClass::DeclarationChange
    );
    assert_eq!(
        mosaic_touch.target().class(),
        UiGraphTouchTargetClass::MosaicMembership
    );
    assert_eq!(mosaic_touch.target().mosaic_name(), Some("workspace"));
}

#[test]
fn query_origin_and_world_are_explicit_on_the_ordinary_touch_path() {
    let authoritative = touch_app(UiGraphWorldProfile::authoritative());
    let query_world = touch_app(query_snapshot_world_profile(
        "snapshot:graph-touch",
        ["worth-ui.graph", "touch", "query"],
    ));

    let denied = authoritative.graph().touches().query_fact_change_receipt();
    let query_touch = query_world
        .graph()
        .touches()
        .from_mounted_receipt_transition(
            query_world
                .graph()
                .touches()
                .query_fact_change_receipt()
                .expect(
                    "query-backed worlds should mint query touch receipts from basis authority",
                ),
            UiGraphTouchTiming::PostMutation,
            mounted_receipt_transition(&query_world, control_artifact(&query_world)),
            UiGraphTouchAspects::new().query_binding(UiGraphTouchAspectPosture::Invalidated),
        )
        .expect("query-backed touch should admit in query world");

    assert!(matches!(
        denied,
        Err(UiGraphTouchDenial::QueryFactChangeUnavailableInCurrentWorld)
    ));
    assert_eq!(
        query_touch.origin().class(),
        UiGraphTouchOriginClass::QueryFactChange
    );
    assert_eq!(
        query_touch.world().world_profile(),
        query_world.graph().world_profile()
    );
}

#[test]
fn unavailable_target_classes_deny_instead_of_broadening_touch_authority() {
    let app = touch_app(UiGraphWorldProfile::authoritative());
    let graph = app.graph();
    let control_id = graph_node_identity(graph, control_artifact(&app));

    let region_denial = graph.touches().from_region_membership(
        graph
            .touches()
            .declaration_change_receipt(control_artifact(&app))
            .expect("control artifact should admit declaration-change witness"),
        UiGraphTouchTiming::PostMutation,
        control_id,
        UiGraphTouchAspects::new().diagnostic(UiGraphTouchAspectPosture::Read),
    );

    assert!(matches!(
        region_denial,
        Err(UiGraphTouchDenial::RegionMembershipUnavailable { graph_node_identity })
        if graph_node_identity == control_id
    ));
}

#[test]
fn touch_world_preserves_specialized_operating_world_families() {
    let worlds = [
        UiGraphWorldProfile::branch_session_label(
            WorthQuerySessionLabel::scoped_strs("worth-ui", ["branch", "touch"])
                .expect("branch session label should admit"),
        ),
        UiGraphWorldProfile::hot_reload_candidate(
            WorthQuerySessionLabel::scoped_strs("worth-ui", ["hot-reload", "touch"])
                .expect("hot-reload session label should admit"),
        ),
        UiGraphWorldProfile::diagnostic(
            WorthQuerySessionLabel::scoped_strs("worth-ui", ["diagnostic", "touch"])
                .expect("diagnostic session label should admit"),
        ),
        UiGraphWorldProfile::host_observation(
            WorthQuerySessionLabel::scoped_strs("worth-ui", ["host-observation", "touch"])
                .expect("host-observation session label should admit"),
        ),
        UiGraphWorldProfile::test_certification(
            WorthQuerySessionLabel::scoped_strs("worth-ui", ["test-certification", "touch"])
                .expect("test-certification session label should admit"),
        ),
    ];

    for world in worlds {
        let app = touch_app(world.clone());
        let touch = app
            .graph()
            .touches()
            .from_node(
                app.graph()
                    .touches()
                    .declaration_change_receipt(control_artifact(&app))
                    .expect("specialized worlds should still admit declaration-origin touches"),
                UiGraphTouchTiming::PostMutation,
                graph_node_identity(app.graph(), control_artifact(&app)),
                UiGraphTouchAspects::new().structural(UiGraphTouchAspectPosture::Read),
            )
            .expect("typed specialized worlds should survive ordinary touch construction");

        assert_eq!(touch.world().world_profile(), &world);
    }
}
