use worth_ui::facade::{
    SurfaceId, WorthUiEventGeometryValueDenialCode, WorthUiPrimitiveEventContainment,
    WorthUiPrimitiveEventCursor, WorthUiPrimitiveEventDispatchPlan,
    WorthUiPrimitiveEventHitTestPoint, WorthUiPrimitiveEventRegionOrder,
    WorthUiPrimitiveEventRegionReceipt, WorthUiPrimitiveHitArea,
    WorthUiPrimitiveHitFrameDerivationBasis, WorthUiPrimitivePointerCapture,
    WorthUiPrimitiveProofDenial, WorthUiPrimitiveResolvedCursorPosture, WorthUiRuntimeFactFamily,
    WorthUiSemanticSliceId,
};
use worth_ui_validation_app::reload::{
    ValidationAuthoredReloadEdit, ValidationPreparedReload, ValidationReloadRequest,
};
use worth_ui_validation_app::{
    ValidationRuntimeWorkbench, ValidationWorkbenchAuthoredInputs, ValidationWorkbenchLaunch,
};

const OUTER_SURFACE: &str = "worth.surface.preview.primitive.proof";
const INNER_SURFACE: &str = "worth.surface.preview.primitive.inner";

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

    let primitive = workbench
        .runtime()
        .resolve_primitive_proof(&surface_id(OUTER_SURFACE))
        .expect("event geometry proof resolves");
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

    let denial = workbench
        .runtime()
        .resolve_primitive_proof(&surface_id(OUTER_SURFACE))
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
    let workbench = launch_workbench();
    let event_plan = nested_event_plan(&workbench);
    let inner_region = event_plan
        .regions()
        .iter()
        .find(|region| region.surface_id() == INNER_SURFACE)
        .expect("inner event region exists");
    let inner_point = center_of(inner_region.hit_frame());
    let inner_dispatch = event_plan.dispatch_primary_click(inner_point);

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
    let outer_dispatch = event_plan.dispatch_primary_click(outer_only_point);

    assert_eq!(outer_dispatch.primary_surface_id(), Some(OUTER_SURFACE));
    assert_eq!(
        outer_dispatch.emitted_surface_ids(),
        &[OUTER_SURFACE.to_owned()]
    );
}

#[test]
fn authored_bubble_containment_propagates_from_inner_to_parent() {
    let mut workbench = launch_workbench();
    activate_edits(
        &mut workbench,
        &[edit(INNER_SURFACE, "event_containment", "bubble")],
    );
    let event_plan = nested_event_plan(&workbench);
    let inner_region = event_plan
        .regions()
        .iter()
        .find(|region| region.surface_id() == INNER_SURFACE)
        .expect("inner event region exists");
    let dispatch = event_plan.dispatch_primary_click(center_of(inner_region.hit_frame()));

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
        &[edit(OUTER_SURFACE, "event_hit_area", "explicit_hit_slop")],
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
        padded_region.hit_frame_derivation().edges(),
        workbench
            .runtime()
            .resolve_primitive_proof(&surface_id(OUTER_SURFACE))
            .expect("outer primitive resolves")
            .flow_layout()
            .padding_edges()
    );
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

fn launch_workbench() -> ValidationRuntimeWorkbench {
    ValidationWorkbenchLaunch::new()
        .prepare_from_authored_inputs(ValidationWorkbenchAuthoredInputs::sample())
        .expect("validation workbench should prepare")
        .into_runtime_workbench()
}

fn activate_edits(
    workbench: &mut ValidationRuntimeWorkbench,
    edits: &[ValidationAuthoredReloadEdit],
) {
    let prepared = prepare_reload(workbench, edits);
    workbench
        .activate_reload(prepared)
        .expect("event geometry reload activates");
}

fn prepare_reload(
    workbench: &ValidationRuntimeWorkbench,
    edits: &[ValidationAuthoredReloadEdit],
) -> ValidationPreparedReload {
    let inputs = ValidationWorkbenchAuthoredInputs::sample();
    let mut source_text = inputs.source().source_text().to_owned();
    for edit in edits {
        source_text = edit
            .apply_to_source_text(&source_text)
            .expect("event geometry edit applies to source");
    }
    workbench.runtime().prepare_validation_reload(
        workbench.runtime().active_capability_snapshot(),
        ValidationReloadRequest::from_source_module(inputs.source().module_path(), source_text),
    )
}

fn nested_event_plan(workbench: &ValidationRuntimeWorkbench) -> WorthUiPrimitiveEventDispatchPlan {
    nested_event_plan_with_inner_requirement(workbench, true)
}

fn nested_event_plan_without_required_inner(
    workbench: &ValidationRuntimeWorkbench,
) -> WorthUiPrimitiveEventDispatchPlan {
    nested_event_plan_with_inner_requirement(workbench, false)
}

fn nested_event_plan_with_inner_requirement(
    workbench: &ValidationRuntimeWorkbench,
    require_inner: bool,
) -> WorthUiPrimitiveEventDispatchPlan {
    let outer = workbench
        .runtime()
        .resolve_primitive_proof(&surface_id(OUTER_SURFACE))
        .expect("outer primitive resolves");
    let inner = workbench
        .runtime()
        .resolve_primitive_proof(&surface_id(INNER_SURFACE))
        .expect("inner primitive resolves");
    let outer_plan = outer.draw_plan(900.0, 600.0);
    let outer_frame = outer_plan.frame();
    let inner_plan = inner.draw_plan(outer_frame.width(), outer_frame.height());
    let outer_region = WorthUiPrimitiveEventRegionReceipt::from_primitive_draw_plan(
        &outer,
        &outer_plan,
        WorthUiPrimitiveEventRegionOrder::new(0, 0),
    )
    .expect("outer hit region exists");
    let maybe_inner_region = WorthUiPrimitiveEventRegionReceipt::from_child_primitive_draw_plan_at(
        &inner,
        &inner_plan,
        WorthUiPrimitiveEventRegionOrder::new(1, 0),
        outer.surface_id(),
        outer_frame.x(),
        outer_frame.y(),
    );
    let Some(inner_region) = maybe_inner_region else {
        assert!(!require_inner, "inner hit region exists");
        return WorthUiPrimitiveEventDispatchPlan::from_regions([outer_region]);
    };

    WorthUiPrimitiveEventDispatchPlan::from_regions([outer_region, inner_region])
}

fn outer_event_region(
    workbench: &ValidationRuntimeWorkbench,
) -> WorthUiPrimitiveEventRegionReceipt {
    let outer = workbench
        .runtime()
        .resolve_primitive_proof(&surface_id(OUTER_SURFACE))
        .expect("outer primitive resolves");
    let outer_plan = outer.draw_plan(900.0, 600.0);
    WorthUiPrimitiveEventRegionReceipt::from_primitive_draw_plan(
        &outer,
        &outer_plan,
        WorthUiPrimitiveEventRegionOrder::new(0, 0),
    )
    .expect("outer hit region exists")
}

fn edit(surface_id: &str, prop_key: &str, value: &str) -> ValidationAuthoredReloadEdit {
    ValidationAuthoredReloadEdit::set_surface_prop(surface_id, prop_key, value)
}

fn surface_id(surface_id: &str) -> SurfaceId {
    SurfaceId::new(surface_id).expect("valid surface id")
}

fn center_of(frame: worth_ui::facade::WorthUiPrimitiveFrame) -> WorthUiPrimitiveEventHitTestPoint {
    WorthUiPrimitiveEventHitTestPoint::new(
        frame.x() + frame.width() * 0.5,
        frame.y() + frame.height() * 0.5,
    )
}
