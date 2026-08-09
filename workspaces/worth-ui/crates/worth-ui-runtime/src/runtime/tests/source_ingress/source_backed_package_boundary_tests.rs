use crate::facade::WorthUi;
use crate::runtime::tests::source_ingress_test_support::{empty_artifact, runtime_from_artifact};
use crate::runtime::{WorthUiSourceProvider, WorthUiWatcherEvent};

use super::source_backed_package_boundary_test_support::{
    graph_node_identity_for_provenance, mosaic_membership_name_for_provenance,
    prepare_source_backed_submission, sorted_mosaic_membership_names,
    source_backed_boundary_component, source_backed_boundary_region, source_backed_boundary_sizing,
    source_backed_submission, source_backed_unconstrained_sizing, support_app_with_sizing,
    two_component_source_backed_builder,
};

#[test]
fn file_source_ingress_derives_sealed_source_backed_package_without_helper_sidecar() {
    let support_app = support_app_with_sizing(source_backed_boundary_sizing());
    let submission = runtime_from_artifact(empty_artifact())
        .source_event_ingress(
            WorthUiSourceProvider::in_memory("source-backed-package-boundary").with_file(
                "app/source_backed_package_boundary.wui",
                "component workspace.component.source_backed_boundary { region workspace.region.primary { sizing workspace.sizing.source_backed_boundary; } }",
            ),
        )
        .start()
        .ingest([WorthUiWatcherEvent::provider_revision(
            "source-backed-package-boundary",
        )])
        .expect("source-backed provider should debounce")
        .attempt_candidate_for_certification(support_app.capabilities())
        .expect("source-backed provider should lower through ingress");
    let app = prepare_source_backed_submission(submission, source_backed_boundary_sizing());

    let source_artifacts = app
        .declaration_artifacts()
        .iter()
        .filter(|artifact| {
            artifact.provenance().source_provenance().module_path()
                == "app/source_backed_package_boundary.wui"
        })
        .collect::<Vec<_>>();
    assert_eq!(source_artifacts.len(), 1);
    assert_eq!(
        source_artifacts[0]
            .graph_handoff()
            .expect("source-backed structural claims should be admitted")
            .mosaic_sizing_contract_id()
            .map(|identity| identity.as_str()),
        Some("workspace.sizing.source_backed_boundary")
    );
}

#[test]
fn source_backed_membership_identity_uses_full_module_path_not_same_stem_heuristics() {
    let support_app = support_app_with_sizing(source_backed_boundary_sizing());
    let left = source_backed_submission(
        &support_app,
        "source-backed-left",
        "app/panels/editor.wui",
        "workspace.component.source_backed_boundary",
        "workspace.sizing.source_backed_boundary",
    );
    let right = source_backed_submission(
        &support_app,
        "source-backed-right",
        "app/dialogs/editor.wui",
        "workspace.component.source_backed_boundary",
        "workspace.sizing.source_backed_boundary",
    );
    let left = prepare_source_backed_submission(left, source_backed_boundary_sizing());
    let right = prepare_source_backed_submission(right, source_backed_boundary_sizing());
    let left_name = mosaic_membership_name_for_provenance(&left, "app/panels/editor.wui", 0);
    let right_name = mosaic_membership_name_for_provenance(&right, "app/dialogs/editor.wui", 0);

    assert_ne!(left_name, right_name);
}

#[test]
fn same_file_source_backed_declarations_do_not_collapse_into_one_mosaic_membership() {
    let support_app = two_component_source_backed_builder()
        .freeze()
        .expect("application preparation should succeed");
    let submission = runtime_from_artifact(empty_artifact())
        .source_event_ingress(
            WorthUiSourceProvider::in_memory("source-backed-same-file").with_file(
                "app/source_backed_same_file.wui",
                r#"
component workspace.component.source_backed_boundary {
    region workspace.region.primary {
        sizing workspace.sizing.source_backed_boundary;
    }
}
component workspace.component.source_backed_boundary.peer {
    region workspace.region.primary {
        sizing workspace.sizing.source_backed_boundary;
    }
}
"#,
            ),
        )
        .start()
        .ingest([WorthUiWatcherEvent::provider_revision(
            "source-backed-same-file",
        )])
        .expect("same-file source-backed provider should debounce")
        .attempt_candidate_for_certification(support_app.capabilities())
        .expect("same-file source-backed provider should lower through ingress");
    let app = two_component_source_backed_builder()
        .with_candidate_submission(submission)
        .freeze()
        .expect("complete same-file composition should prepare");
    let left_name =
        mosaic_membership_name_for_provenance(&app, "app/source_backed_same_file.wui", 0);
    let right_name =
        mosaic_membership_name_for_provenance(&app, "app/source_backed_same_file.wui", 1);

    assert_ne!(left_name, right_name);

    assert_eq!(
        app.graph_snapshot()
            .lookup()
            .mosaic_members(&left_name)
            .value()
            .len(),
        1
    );
    assert_eq!(
        app.graph_snapshot()
            .lookup()
            .mosaic_members(&right_name)
            .value()
            .len(),
        1
    );
}

#[test]
fn same_file_equivalent_declaration_reorder_preserves_membership_identity_set() {
    let support_app = two_component_source_backed_builder()
        .freeze()
        .expect("application preparation should succeed");
    let first = runtime_from_artifact(empty_artifact())
        .source_event_ingress(
            WorthUiSourceProvider::in_memory("source-backed-reorder-a").with_file(
                "app/source_backed_reorder.wui",
                r#"
component workspace.component.source_backed_boundary {
    region workspace.region.primary {
        sizing workspace.sizing.source_backed_boundary;
    }
}
component workspace.component.source_backed_boundary.peer {
    region workspace.region.primary {
        sizing workspace.sizing.source_backed_boundary;
    }
}
"#,
            ),
        )
        .start()
        .ingest([WorthUiWatcherEvent::provider_revision(
            "source-backed-reorder-a",
        )])
        .expect("reorder-a provider should debounce")
        .attempt_candidate_for_certification(support_app.capabilities())
        .expect("reorder-a provider should lower through ingress");
    let second = runtime_from_artifact(empty_artifact())
        .source_event_ingress(
            WorthUiSourceProvider::in_memory("source-backed-reorder-b").with_file(
                "app/source_backed_reorder.wui",
                r#"
component workspace.component.source_backed_boundary.peer {
    region workspace.region.primary {
        sizing workspace.sizing.source_backed_boundary;
    }
}
component workspace.component.source_backed_boundary {
    region workspace.region.primary {
        sizing workspace.sizing.source_backed_boundary;
    }
}
"#,
            ),
        )
        .start()
        .ingest([WorthUiWatcherEvent::provider_revision(
            "source-backed-reorder-b",
        )])
        .expect("reorder-b provider should debounce")
        .attempt_candidate_for_certification(support_app.capabilities())
        .expect("reorder-b provider should lower through ingress");
    let first = two_component_source_backed_builder()
        .with_candidate_submission(first)
        .freeze()
        .expect("first complete composition should prepare");
    let second = two_component_source_backed_builder()
        .with_candidate_submission(second)
        .freeze()
        .expect("second complete composition should prepare");

    assert_eq!(
        sorted_mosaic_membership_names(&first),
        sorted_mosaic_membership_names(&second)
    );
}

#[test]
fn unconstrained_source_backed_sizing_does_not_synthesize_bounded_measurement_posture() {
    let support_app = support_app_with_sizing(source_backed_unconstrained_sizing());
    let submission = source_backed_submission(
        &support_app,
        "source-backed-unconstrained",
        "app/source_backed_unconstrained.wui",
        "workspace.component.source_backed_boundary",
        "workspace.sizing.source_backed_unconstrained",
    );
    let app = WorthUi::app()
        .bind_certification_host()
        .with_change_profile(crate::runtime::rebind::UiChangeProfile::platform_pulse())
        .with_candidate_submission(submission)
        .register_component(source_backed_boundary_component())
        .register_mosaic_region_kind(source_backed_boundary_region())
        .register_mosaic_sizing_contract(source_backed_unconstrained_sizing())
        .freeze()
        .expect("application preparation should succeed");
    let node = graph_node_identity_for_provenance(&app, "app/source_backed_unconstrained.wui", 0);
    let graph_node = app
        .graph_snapshot()
        .nodes()
        .iter()
        .find(|candidate| candidate.graph_node_identity() == node)
        .expect("graph node should exist");

    assert_eq!(graph_node.measurement_constraint_modifier(), None);
}
