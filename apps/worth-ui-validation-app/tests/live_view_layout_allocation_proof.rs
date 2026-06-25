use std::fs;
use std::path::Path;

use worth_ui::facade::{
    WorthUiAuthoredLiveViewDocument, WorthUiFlowLayoutCrossAlign,
    WorthUiLayoutAllocationDenialReason, WorthUiLayoutAllocationRequest, WorthUiLiveViewStateValue,
    WorthUiRuntimeFactFamily,
};

#[path = "live_view_phase34_4/support.rs"]
#[allow(dead_code)]
mod phase34_4_support;

#[path = "support/live_view_layout_allocation.rs"]
#[allow(dead_code)]
mod allocation_support;

use allocation_support::{
    allocate, allocate_input_stack, allocate_input_stack_for_mounted, allocated_child,
    assert_close, input_stack_policy, measured_view_with_observations, mounted_product_view,
    prepared_app_with_live_view_source, row_contact_form_source, text_icon_baseline_source,
    weighted_row_with_hug_text_source,
};

#[test]
fn weighted_fill_accounts_for_hug_children_gap_and_padding() {
    let app = prepared_app_with_live_view_source(weighted_row_with_hug_text_source());
    let mounted = mounted_product_view(&app);
    let measured =
        measured_view_with_observations(&app, &mounted, "input_stack", 640.0, 180.0, |draft| {
            draft.observe_text_metric("helper_copy", 42, 90.0, 24.0, 18.0)
        });
    let allocation = allocate(&app, &measured, "input_stack");
    let policy = input_stack_policy(&allocation);

    let first = allocated_child(&allocation, "live_view.control.first_name_input");
    let helper = allocated_child(&allocation, "helper_copy");
    let second = allocated_child(&allocation, "live_view.control.contact_mode_input");
    let fill_pool = 640.0
        - policy.padding_edges().horizontal()
        - policy.gap_points() * 2.0
        - helper.frame().width();

    assert_close(first.frame().width(), fill_pool / 3.0);
    assert_close(second.frame().width(), fill_pool * 2.0 / 3.0);
    assert_close(helper.frame().width(), 90.0);
    assert_eq!(
        helper.natural_metric_basis(),
        "host_text_metric:helper_copy:42"
    );
    assert_eq!(allocation.counters().fill_child_count(), 2);
    assert_eq!(allocation.counters().hug_child_count(), 1);
    assert_eq!(
        allocation.participating_child_ids(),
        &[
            "live_view.control.first_name_input".to_owned(),
            "helper_copy".to_owned(),
            "live_view.control.contact_mode_input".to_owned()
        ]
    );
}

#[test]
fn baseline_alignment_changes_child_frames_from_host_metrics() {
    let app = prepared_app_with_live_view_source(text_icon_baseline_source());
    let mounted = mounted_product_view(&app);
    let measured =
        measured_view_with_observations(&app, &mounted, "input_stack", 360.0, 120.0, |draft| {
            draft
                .observe_text_metric("label_text", 11, 120.0, 24.0, 18.0)
                .observe_icon_metric("status_icon", 22, 20.0, 20.0, 8.0)
        });
    let allocation = allocate(&app, &measured, "input_stack");
    let policy = input_stack_policy(&allocation);
    assert_eq!(policy.cross_align(), WorthUiFlowLayoutCrossAlign::Baseline);

    let text = allocated_child(&allocation, "label_text");
    let icon = allocated_child(&allocation, "status_icon");
    assert_close(
        text.frame().y() + text.baseline_points(),
        icon.frame().y() + icon.baseline_points(),
    );
    assert!(
        icon.frame().y() > text.frame().y(),
        "shorter icon baseline should be lowered"
    );
    assert_eq!(
        text.natural_metric_basis(),
        "host_text_metric:label_text:11"
    );
    assert_eq!(
        icon.natural_metric_basis(),
        "host_icon_metric:status_icon:22"
    );
}

#[test]
fn conditional_absence_removes_child_from_fill_pool_and_retains_state() {
    let mut app = prepared_app_with_live_view_source(conditional_row_source());
    phase34_4_support::apply_text(&mut app, "contact_mode", "yes");
    phase34_4_support::apply_text(&mut app, "company_name", "Acme");
    let shown = allocate_input_stack(&app, 720.0);
    let shown_company = allocated_child(&shown, "live_view.control.company_name_input");
    assert!(shown_company.participation().participates_in_layout());

    phase34_4_support::apply_text(&mut app, "contact_mode", "no");
    let hidden = allocate_input_stack(&app, 720.0);
    let hidden_first = allocated_child(&hidden, "live_view.control.first_name_input");
    let hidden_second = allocated_child(&hidden, "live_view.control.contact_mode_input");
    let hidden_company = allocated_child(&hidden, "live_view.control.company_name_input");
    assert_eq!(hidden_company.frame().width(), 0.0);
    assert!(!hidden_company.participation().participates_in_layout());
    assert!(hidden_first.frame().width() > shown_company.frame().width());
    assert_close(hidden_first.frame().width(), hidden_second.frame().width());

    phase34_4_support::apply_text(&mut app, "contact_mode", "yes");
    let proof = app.live_view_projection_proof().expect("projection admits");
    let company_binding = proof
        .declaration()
        .binding("company_name")
        .expect("company binding is retained");
    assert_eq!(
        app.workbench()
            .runtime()
            .live_view_state_value(company_binding)
            .map(WorthUiLiveViewStateValue::as_display_text),
        Some("Acme".to_owned())
    );
}

#[test]
fn flow_gap_hot_reload_changes_allocation_without_composition_rebuild() {
    let mut app = prepared_app_with_live_view_source(row_contact_form_source());
    let prior = mounted_product_view(&app);
    let prior_allocation = allocate_input_stack_for_mounted(&app, &prior, 640.0);

    let next = app
        .hot_reload_live_view_source(row_contact_form_source().replace(
            "validation.density.primitive.flow.gap.default",
            "validation.density.primitive.flow.gap.compact",
        ))
        .expect("gap edit hot reloads");
    let next_mounted = next.mounted_product_view().clone();
    let next_allocation = allocate_input_stack_for_mounted(&app, &next_mounted, 640.0);

    assert_eq!(
        prior.composition_graph_digest(),
        next_mounted.composition_graph_digest()
    );
    assert_ne!(
        prior_allocation.receipt_digest(),
        next_allocation.receipt_digest()
    );
    let allocation_rebind = app
        .workbench()
        .runtime()
        .rebind_layout_allocation(&prior_allocation, &next_allocation);
    assert_eq!(
        allocation_rebind
            .changed_fact_families()
            .collect::<Vec<_>>(),
        vec![WorthUiRuntimeFactFamily::LayoutAllocation]
    );
    assert_eq!(allocation_rebind.counters().source_reparse_count(), 0);
    assert_eq!(allocation_rebind.counters().renderer_parse_count(), 0);
    assert_eq!(allocation_rebind.counters().artifact_scan_count(), 0);
    assert_eq!(
        next.last_rebind()
            .expect("hot reload records projection rebind")
            .control_rebind()
            .changed_facts()
            .len(),
        0,
        "flow-only edits must not rebind control projections"
    );
}

#[test]
fn invalid_child_sizing_rejects_at_source_before_allocation() {
    let denial = WorthUiAuthoredLiveViewDocument::parse(&row_contact_form_source().replace(
        "child control first_name_input sizing fill(1)",
        "child control first_name_input sizing fill(0)",
    ))
    .expect_err("invalid fill weight rejects during source admission");

    assert!(denial
        .message()
        .contains("composition child fill sizing weight must be positive"));
}

#[test]
fn missing_host_bounds_reject_before_adapter_rendering() {
    let app = prepared_app_with_live_view_source(row_contact_form_source());
    let mounted = mounted_product_view(&app);
    let admitted = app
        .workbench()
        .runtime()
        .admit_host_frame_observations(
            &mounted,
            worth_ui::facade::WorthUiHostFrameObservationDraft::for_mounted_product_view(
                mounted.receipt_digest(),
                21,
            ),
        )
        .expect("empty host observations admit as not ready");
    let measured = app
        .workbench()
        .runtime()
        .measure_mounted_product_view(&mounted, admitted)
        .expect("measurement receipt preserves not-ready posture");
    let denial = app
        .workbench()
        .runtime()
        .allocate_mounted_product_view(
            &measured,
            WorthUiLayoutAllocationRequest::for_root_node("input_stack"),
        )
        .expect_err("layout allocation requires admitted host bounds");

    assert_eq!(
        denial.reason(),
        WorthUiLayoutAllocationDenialReason::MissingAvailableBounds
    );
}

#[test]
fn live_view_renderer_does_not_allocate_rows_or_spacing_locally() {
    let rendering = fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("src")
            .join("app")
            .join("live_view")
            .join("rendering.rs"),
    )
    .expect("rendering adapter should be readable");

    for forbidden in [
        "ui.horizontal(",
        "ui.vertical(",
        "with_layout(",
        "allocate_ui_at_rect(",
        "spacing_mut().item_spacing",
        "inner_margin(",
        "add_space(",
        "set_min_width(",
        "desired_width(",
        "Painter",
    ] {
        assert!(
            !rendering.contains(forbidden),
            "mounted live-view renderer must consume layout allocation receipts, not `{forbidden}`"
        );
    }
}

fn conditional_row_source() -> String {
    phase34_4_support::contact_submit_source("data_payload_values").replace(
        "target button_proof",
        "target button_proof\n    flow_kind row",
    )
}
