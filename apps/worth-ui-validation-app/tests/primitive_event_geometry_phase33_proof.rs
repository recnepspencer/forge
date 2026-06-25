mod primitive_event_geometry_phase33_support;

use primitive_event_geometry_phase33_support::{
    activate_edits, center_of, edit, launch_workbench, nested_event_plan,
    nested_event_plan_without_required_inner, outer_event_region, prepare_reload,
    primitive_proof_for_authored_surface, surface_id, INNER_SURFACE, OUTER_SURFACE,
};
use worth_ui::facade::{
    WorthUiEventGeometryValueDenialCode, WorthUiPrimitiveEventContainment,
    WorthUiPrimitiveEventCursor, WorthUiPrimitiveEventHitTestPoint, WorthUiPrimitiveHitArea,
    WorthUiPrimitiveHitFrameDerivationBasis, WorthUiPrimitivePointerCapture,
    WorthUiPrimitiveProofDenial, WorthUiPrimitiveResolvedCursorPosture, WorthUiRuntimeFactFamily,
    WorthUiRuntimeFactId, WorthUiSemanticSliceId,
};

#[test]
fn event_geometry_admits_cursor_hit_area_capture_and_containment() {
    let mut workbench = launch_workbench();
    activate_edits(
        &mut workbench,
        &[
            edit(OUTER_SURFACE, "event_cursor", "text"),
            edit(OUTER_SURFACE, "event_hit_area", "explicit_hit_slop"),
            edit(
                OUTER_SURFACE,
                "event_hit_slop",
                "validation.density.primitive.event.hit_slop.comfortable",
            ),
            edit(OUTER_SURFACE, "event_containment", "bubble"),
            edit(OUTER_SURFACE, "event_capture", "press_drag"),
        ],
    );

    let primitive = primitive_proof_for_authored_surface(&workbench, OUTER_SURFACE);
    let event_geometry = primitive.event_geometry();

    assert_eq!(event_geometry.cursor(), WorthUiPrimitiveEventCursor::Text);
    assert_eq!(
        event_geometry.resolved_cursor(),
        WorthUiPrimitiveResolvedCursorPosture::Text
    );
    assert_eq!(
        event_geometry.hit_area(),
        WorthUiPrimitiveHitArea::ExplicitHitSlop
    );
    assert_eq!(
        event_geometry.hit_slop_token(),
        "validation.density.primitive.event.hit_slop.comfortable"
    );
    assert!(event_geometry.hit_slop_edges().left() > 0.0);
    assert_eq!(
        event_geometry.containment(),
        WorthUiPrimitiveEventContainment::Bubble
    );
    assert_eq!(
        event_geometry.capture(),
        WorthUiPrimitivePointerCapture::PressDrag
    );
}

#[test]
fn invalid_event_geometry_values_report_one_schema_ordered_denial_set() {
    let mut workbench = launch_workbench();
    activate_edits(
        &mut workbench,
        &[
            edit(OUTER_SURFACE, "event_cursor", "banana"),
            edit(OUTER_SURFACE, "event_hit_area", "mist"),
            edit(OUTER_SURFACE, "event_hit_slop", "7"),
            edit(OUTER_SURFACE, "event_containment", "sideways"),
            edit(OUTER_SURFACE, "event_capture", "forever"),
            edit(OUTER_SURFACE, "event_surprise", "nope"),
        ],
    );

    let target = workbench
        .runtime()
        .bind_authored_primitive_proof_target(&surface_id(OUTER_SURFACE))
        .expect("event geometry target binds");
    let denial = workbench
        .runtime()
        .resolve_primitive_proof_for_target(&target)
        .expect_err("invalid event geometry rejects primitive proof");
    let WorthUiPrimitiveProofDenial::InvalidEventGeometryValues { report } = denial else {
        panic!("expected event geometry report");
    };
    let denial_set = report.status().denial_set().expect("denial set");

    assert_eq!(report.counters().denials_emitted(), 6);
    assert_eq!(
        denial_set
            .denials()
            .iter()
            .map(|denial| denial.prop_key())
            .collect::<Vec<_>>(),
        vec![
            "event_cursor",
            "event_hit_area",
            "event_hit_slop",
            "event_containment",
            "event_capture",
            "event_surprise",
        ]
    );
    assert_eq!(
        denial_set.denials()[0].denial_code(),
        WorthUiEventGeometryValueDenialCode::InvalidCursor
    );
    let presentation = denial_set.denials()[0].presentation();
    assert_eq!(presentation.title(), "Event geometry value rejected");
    assert!(presentation
        .rows()
        .iter()
        .any(|row| row.label() == "expected" && row.value().contains("pointer")));
    assert_ne!(denial_set.denial_set_digest(), 0);
}

#[test]
fn event_geometry_edits_rebind_only_event_geometry_consumers() {
    let workbench = launch_workbench();
    let prepared = prepare_reload(&workbench, &[edit(OUTER_SURFACE, "event_cursor", "grab")]);
    let changed = prepared
        .changed_fact_mapping_receipt()
        .expect("event cursor edit emits changed fact mapping");

    let mut saw_event_geometry = false;
    let mut saw_interaction = false;
    for row in changed.rows() {
        if row.semantic_row().slice_id() == WorthUiSemanticSliceId::PrimitiveEventGeometry {
            saw_event_geometry = row
                .changed_facts()
                .contains_family(WorthUiRuntimeFactFamily::PrimitiveEventGeometry);
        }
        if row.semantic_row().slice_id() == WorthUiSemanticSliceId::PrimitiveInteraction {
            saw_interaction = true;
        }
    }

    assert!(saw_event_geometry);
    assert!(!saw_interaction);
}

#[test]
fn nested_containment_routes_inner_and_outer_clicks_distinctly() {
    let mut workbench = launch_workbench();
    activate_edits(
        &mut workbench,
        &[
            edit(OUTER_SURFACE, "primitive_disabled", "false"),
            edit(OUTER_SURFACE, "interaction_readiness", "enabled"),
            edit(INNER_SURFACE, "primitive_disabled", "false"),
            edit(INNER_SURFACE, "interaction_readiness", "enabled"),
        ],
    );
    let event_plan = nested_event_plan(&workbench);
    let inner_region = event_plan
        .regions()
        .iter()
        .find(|region| region.surface_id() == INNER_SURFACE)
        .expect("inner event region exists");
    let inner_point = center_of(inner_region.hit_frame());
    let inner_target = workbench
        .runtime()
        .bind_primitive_event_dispatch_target(&event_plan, inner_point)
        .expect("inner event target binds");
    let inner_dispatch = event_plan.dispatch_primary_click_for_target(&inner_target, inner_point);

    assert_eq!(inner_dispatch.primary_surface_id(), Some(INNER_SURFACE));
    assert_eq!(
        inner_dispatch.emitted_surface_ids(),
        &[INNER_SURFACE.to_owned()]
    );
    assert_eq!(
        inner_dispatch.cursor(),
        WorthUiPrimitiveResolvedCursorPosture::Pointer
    );

    let outer_region = event_plan
        .regions()
        .iter()
        .find(|region| region.surface_id() == OUTER_SURFACE)
        .expect("outer event region exists");
    let outer_only_point = WorthUiPrimitiveEventHitTestPoint::new(
        outer_region.hit_frame().x() + 4.0,
        outer_region.hit_frame().y() + 4.0,
    );
    let outer_target = workbench
        .runtime()
        .bind_primitive_event_dispatch_target(&event_plan, outer_only_point)
        .expect("outer event target binds");
    let outer_dispatch =
        event_plan.dispatch_primary_click_for_target(&outer_target, outer_only_point);

    assert_eq!(outer_dispatch.primary_surface_id(), Some(OUTER_SURFACE));
    assert_eq!(
        outer_dispatch.emitted_surface_ids(),
        &[OUTER_SURFACE.to_owned()]
    );
}

#[test]
fn event_click_binds_target_before_dispatching_interaction_surface() {
    let mut workbench = launch_workbench();
    activate_edits(
        &mut workbench,
        &[
            edit(OUTER_SURFACE, "primitive_disabled", "false"),
            edit(OUTER_SURFACE, "interaction_readiness", "enabled"),
            edit(INNER_SURFACE, "primitive_disabled", "false"),
            edit(INNER_SURFACE, "interaction_readiness", "enabled"),
        ],
    );
    let event_plan = nested_event_plan(&workbench);
    let inner_region = event_plan
        .regions()
        .iter()
        .find(|region| region.surface_id() == INNER_SURFACE)
        .expect("inner event region exists");
    let inner_point = center_of(inner_region.hit_frame());
    let target = workbench
        .runtime()
        .bind_primitive_event_dispatch_target(&event_plan, inner_point)
        .expect("event target binds from hit-tested point");
    let dispatch = event_plan.dispatch_primary_click_for_target(&target, inner_point);

    assert_eq!(target.surface_id().as_str(), INNER_SURFACE);
    assert_ne!(target.binding_digest(), 0);
    assert_eq!(dispatch.primary_surface_id(), Some(INNER_SURFACE));
    assert_eq!(dispatch.emitted_surface_ids(), &[INNER_SURFACE.to_owned()]);
}

#[test]
fn event_dispatch_with_mismatched_target_cannot_emit_surface() {
    let mut workbench = launch_workbench();
    activate_edits(
        &mut workbench,
        &[
            edit(OUTER_SURFACE, "primitive_disabled", "false"),
            edit(OUTER_SURFACE, "interaction_readiness", "enabled"),
            edit(INNER_SURFACE, "primitive_disabled", "false"),
            edit(INNER_SURFACE, "interaction_readiness", "enabled"),
        ],
    );
    let event_plan = nested_event_plan(&workbench);
    let inner_region = event_plan
        .regions()
        .iter()
        .find(|region| region.surface_id() == INNER_SURFACE)
        .expect("inner event region exists");
    let outer_region = event_plan
        .regions()
        .iter()
        .find(|region| region.surface_id() == OUTER_SURFACE)
        .expect("outer event region exists");
    let inner_point = center_of(inner_region.hit_frame());
    let outer_only_point = WorthUiPrimitiveEventHitTestPoint::new(
        outer_region.hit_frame().x() + 4.0,
        outer_region.hit_frame().y() + 4.0,
    );
    let outer_target = workbench
        .runtime()
        .bind_primitive_event_dispatch_target(&event_plan, outer_only_point)
        .expect("outer target binds");
    let dispatch = event_plan.dispatch_primary_click_for_target(&outer_target, inner_point);

    assert_eq!(dispatch.primary_surface_id(), Some(INNER_SURFACE));
    assert!(dispatch.emitted_surface_ids().is_empty());
}

#[test]
fn authored_bubble_containment_propagates_from_inner_to_parent() {
    let mut workbench = launch_workbench();
    activate_edits(
        &mut workbench,
        &[
            edit(OUTER_SURFACE, "primitive_disabled", "false"),
            edit(OUTER_SURFACE, "interaction_readiness", "enabled"),
            edit(INNER_SURFACE, "primitive_disabled", "false"),
            edit(INNER_SURFACE, "interaction_readiness", "enabled"),
            edit(INNER_SURFACE, "event_containment", "bubble"),
        ],
    );
    let event_plan = nested_event_plan(&workbench);
    let inner_region = event_plan
        .regions()
        .iter()
        .find(|region| region.surface_id() == INNER_SURFACE)
        .expect("inner event region exists");
    let inner_point = center_of(inner_region.hit_frame());
    let target = workbench
        .runtime()
        .bind_primitive_event_dispatch_target(&event_plan, inner_point)
        .expect("inner event target binds");
    let dispatch = event_plan.dispatch_primary_click_for_target(&target, inner_point);

    assert_eq!(
        dispatch.emitted_surface_ids(),
        &[INNER_SURFACE.to_owned(), OUTER_SURFACE.to_owned()]
    );
    assert_eq!(
        dispatch.containment(),
        Some(WorthUiPrimitiveEventContainment::Bubble)
    );
}

#[test]
fn padded_bounds_and_explicit_hit_slop_use_distinct_authorities() {
    let mut workbench = launch_workbench();
    activate_edits(
        &mut workbench,
        &[
            edit(
                OUTER_SURFACE,
                "flow_padding",
                "validation.density.primitive.flow.padding.fat",
            ),
            edit(OUTER_SURFACE, "event_hit_area", "padded_bounds"),
            edit(
                OUTER_SURFACE,
                "event_hit_slop",
                "validation.density.primitive.event.hit_slop.compact",
            ),
        ],
    );
    let padded_region = outer_event_region(&workbench);
    let padded_visual = padded_region.visual_frame();
    let padded_hit = padded_region.hit_frame();

    activate_edits(
        &mut workbench,
        &[
            edit(
                OUTER_SURFACE,
                "flow_padding",
                "validation.density.primitive.flow.padding.fat",
            ),
            edit(OUTER_SURFACE, "event_hit_area", "explicit_hit_slop"),
        ],
    );
    let explicit_region = outer_event_region(&workbench);
    let explicit_hit = explicit_region.hit_frame();

    assert!(padded_hit.width() > explicit_hit.width());
    assert!(padded_hit.height() > explicit_hit.height());
    assert_eq!(padded_visual, explicit_region.visual_frame());
    assert_eq!(
        padded_region.hit_frame_derivation().basis(),
        WorthUiPrimitiveHitFrameDerivationBasis::FlowPadding
    );
    assert_eq!(
        padded_region.graph_basis().produced_fact().family(),
        WorthUiRuntimeFactFamily::PrimitiveEventRegion
    );
    assert_eq!(
        padded_region.graph_basis().produced_fact(),
        &WorthUiRuntimeFactId::primitive_event_region(OUTER_SURFACE)
    );
    assert!(padded_region
        .graph_basis()
        .consumed_facts()
        .contains(&WorthUiRuntimeFactId::primitive_flow_layout(OUTER_SURFACE)));
    assert!(padded_region
        .graph_basis()
        .consumed_facts()
        .contains(&WorthUiRuntimeFactId::primitive_content(OUTER_SURFACE)));
    assert!(padded_region
        .graph_basis()
        .consumed_facts()
        .contains(&WorthUiRuntimeFactId::primitive_draw_plan(OUTER_SURFACE)));
    assert!(padded_region.graph_basis().consumed_facts().contains(
        &WorthUiRuntimeFactId::primitive_event_geometry(OUTER_SURFACE)
    ));
    assert_eq!(padded_region.graph_basis().source_parse_count(), 0);
    assert_eq!(padded_region.graph_basis().artifact_scan_count(), 0);
    assert_eq!(
        explicit_region.hit_frame_derivation().basis(),
        WorthUiPrimitiveHitFrameDerivationBasis::ExplicitHitSlop
    );
}

#[test]
fn disabled_none_removes_disabled_region_from_hit_testing() {
    let mut workbench = launch_workbench();
    activate_edits(
        &mut workbench,
        &[
            edit(INNER_SURFACE, "interaction_readiness", "disabled"),
            edit(INNER_SURFACE, "event_hit_area", "disabled_none"),
        ],
    );
    let event_plan = nested_event_plan_without_required_inner(&workbench);

    assert!(event_plan
        .regions()
        .iter()
        .all(|region| region.surface_id() != INNER_SURFACE));
}
