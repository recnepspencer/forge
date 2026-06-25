#[allow(dead_code)]
mod support;

use support::live_view_layout_allocation::{
    allocate, measured_view_with_observations, mounted_product_view,
    prepared_app_with_live_view_source,
};
use support::live_view_viewport_fixtures::{
    card_clip_boundary_source, local_scroll_boundary_source, nested_scroll_boundary_source,
    unsupported_viewport_policy_source,
};
use worth_ui::facade::{WorthUiRuntimeFactFamily, WorthUiViewportBoundaryDenialReason};

#[test]
fn local_scroll_boundary_declares_clip_hit_focus_and_accessibility_participation() {
    let app = prepared_app_with_live_view_source(local_scroll_boundary_source());
    let mounted = mounted_product_view(&app);
    let measured =
        measured_view_with_observations(&app, &mounted, "input_stack", 420.0, 180.0, |draft| {
            draft.observe_scroll_viewport("input_stack", 0.0, 0.0, 420.0, 44.0)
        });
    let allocation = allocate(&app, &measured, "input_stack");

    let viewport = app
        .workbench()
        .runtime()
        .resolve_viewport_boundaries(&measured, &allocation)
        .expect("viewport boundary admits");
    let boundary = viewport
        .boundary_for_node("input_stack")
        .expect("input stack boundary");

    assert_eq!(boundary.policy().scroll_owner().token(), "composition");
    assert_eq!(boundary.policy().clip_posture().token(), "clip_to_viewport");
    assert!(boundary.consumed_facts().iter().any(|fact| {
        matches!(
            fact.family(),
            WorthUiRuntimeFactFamily::MosaicPlacementLegality
        )
    }));
    assert!(
        boundary
            .descendants()
            .iter()
            .any(|row| row.node_id() == "live_view.control.first_name_input"
                && row.hit_participates())
    );
    assert!(boundary.descendants().iter().any(|row| {
        row.node_id() == "live_view.control.contact_mode_input"
            && !row.visible()
            && !row.hit_participates()
            && !row.focus_participates()
            && !row.accessibility_participates()
            && !row.measurement_participates()
    }));
    assert_eq!(viewport.counters().source_reparse_count(), 0);
    assert_eq!(viewport.counters().renderer_parse_count(), 0);
}

#[test]
fn clipped_boundary_participation_applies_to_nested_descendants() {
    let app = prepared_app_with_live_view_source(card_clip_boundary_source());
    let mounted = mounted_product_view(&app);
    let measured = measured_view_with_observations(
        &app,
        &mounted,
        "live_view.form_card",
        420.0,
        70.0,
        |draft| draft,
    );
    let allocation = allocate(&app, &measured, "live_view.form_card");

    let viewport = app
        .workbench()
        .runtime()
        .resolve_viewport_boundaries(&measured, &allocation)
        .expect("card clip viewport admits");
    let boundary = viewport
        .boundary_for_node("live_view.form_card")
        .expect("card viewport boundary");

    assert!(boundary
        .descendants()
        .iter()
        .any(|row| row.node_id() == "live_view.control.first_name_input"));
    assert!(boundary.descendants().iter().any(|row| {
        row.node_id() == "live_view.interaction.contact_submit"
            && !row.visible()
            && !row.hit_participates()
            && !row.focus_participates()
            && !row.accessibility_participates()
            && !row.measurement_participates()
    }));
    let effective = app
        .workbench()
        .runtime()
        .resolve_effective_viewport_participation(&mounted, &viewport);
    let submit = effective
        .row_for_node("live_view.interaction.contact_submit")
        .expect("nested submit has effective viewport row");
    assert_eq!(submit.governing_boundary_count(), 1);
    assert!(!submit.visible());
    assert!(!submit.hit_participates());
    assert!(!submit.focus_participates());
    assert!(!submit.accessibility_participates());
    assert!(!submit.measurement_participates());
    assert_eq!(effective.counters().source_reparse_count(), 0);
    assert_eq!(effective.counters().renderer_parse_count(), 0);
}

#[test]
fn unsupported_viewport_policy_rejects_before_renderer_participation() {
    let app = prepared_app_with_live_view_source(unsupported_viewport_policy_source());
    let mounted = mounted_product_view(&app);
    let measured =
        measured_view_with_observations(&app, &mounted, "input_stack", 420.0, 180.0, |draft| {
            draft.observe_scroll_viewport("input_stack", 0.0, 0.0, 420.0, 44.0)
        });
    let allocation = allocate(&app, &measured, "input_stack");

    let denials = app
        .workbench()
        .runtime()
        .resolve_viewport_boundaries(&measured, &allocation)
        .expect_err("unsupported viewport policy rejects");

    assert_eq!(
        denials[0].reason(),
        WorthUiViewportBoundaryDenialReason::UnsupportedPolicyIdentity
    );
}

#[test]
fn composition_scroll_requires_host_scroll_viewport_observation() {
    let app = prepared_app_with_live_view_source(local_scroll_boundary_source());
    let mounted = mounted_product_view(&app);
    let measured =
        measured_view_with_observations(&app, &mounted, "input_stack", 420.0, 180.0, |draft| draft);
    let allocation = allocate(&app, &measured, "input_stack");

    let denials = app
        .workbench()
        .runtime()
        .resolve_viewport_boundaries(&measured, &allocation)
        .expect_err("composition scroll without host scroll viewport rejects");

    assert!(denials.iter().any(|denial| {
        denial.reason() == WorthUiViewportBoundaryDenialReason::MissingHostViewportObservation
            && denial.subject() == "input_stack"
    }));
}

#[test]
fn nested_composition_scroll_ownership_rejects_structurally() {
    let app = prepared_app_with_live_view_source(nested_scroll_boundary_source());
    let mounted = mounted_product_view(&app);
    let measured = measured_view_with_observations(
        &app,
        &mounted,
        "live_view.form_card",
        420.0,
        180.0,
        |draft| {
            draft
                .observe_scroll_viewport("live_view.form_card", 0.0, 0.0, 420.0, 120.0)
                .observe_scroll_viewport("input_stack", 0.0, 0.0, 420.0, 44.0)
        },
    );
    let allocation = allocate(&app, &measured, "live_view.form_card");

    let denials = app
        .workbench()
        .runtime()
        .resolve_viewport_boundaries(&measured, &allocation)
        .expect_err("nested composition scroll rejects");

    assert!(denials.iter().any(|denial| {
        denial.reason() == WorthUiViewportBoundaryDenialReason::NestedCompositionScrollOwner
            && denial.subject() == "input_stack"
    }));
}

#[test]
fn scroll_offset_rebind_names_only_viewport_families() {
    let app = prepared_app_with_live_view_source(local_scroll_boundary_source());
    let mounted = mounted_product_view(&app);
    let first =
        measured_view_with_observations(&app, &mounted, "input_stack", 420.0, 180.0, |draft| {
            draft.observe_scroll_viewport("input_stack", 0.0, 0.0, 420.0, 44.0)
        });
    let second =
        measured_view_with_observations(&app, &mounted, "input_stack", 420.0, 180.0, |draft| {
            draft.observe_scroll_viewport("input_stack", 0.0, 20.0, 420.0, 44.0)
        });
    let first_allocation = allocate(&app, &first, "input_stack");
    let second_allocation = allocate(&app, &second, "input_stack");
    let first_viewport = app
        .workbench()
        .runtime()
        .resolve_viewport_boundaries(&first, &first_allocation)
        .expect("first viewport admits");
    let second_viewport = app
        .workbench()
        .runtime()
        .resolve_viewport_boundaries(&second, &second_allocation)
        .expect("second viewport admits");

    let rebind = app
        .workbench()
        .runtime()
        .rebind_viewport_boundaries(&first_viewport, &second_viewport);

    assert!(rebind.counters().changed_viewport_fact_count() > 0);
    assert!(rebind.changed_facts().iter().all(|fact| matches!(
        fact.family(),
        WorthUiRuntimeFactFamily::ViewportBoundary
            | WorthUiRuntimeFactFamily::ClipBoundary
            | WorthUiRuntimeFactFamily::ScrollRestoration
            | WorthUiRuntimeFactFamily::ViewportEventParticipation
    )));
    assert_eq!(rebind.counters().source_reparse_count(), 0);
    assert_eq!(rebind.counters().renderer_parse_count(), 0);
}
