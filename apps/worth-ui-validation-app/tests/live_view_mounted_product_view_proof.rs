use worth_ui::facade::{
    WorthUiCompositionGraphDefinition, WorthUiCompositionNodeKind,
    WorthUiCompositionRootDefinition, WorthUiCompositionRootKind,
    WorthUiCompositionRootMountDenialCode, WorthUiLiveViewStateValue, WorthUiMountedNodeReceipt,
    WorthUiMountedProductViewSemanticSlice, WorthUiRuntimeFactFamily,
};
use worth_ui_validation_app::reload::ValidationLiveViewSource;
use worth_ui_validation_app::{
    ValidationLiveViewCompositionRebindDecision, ValidationWorkbenchAuthoredInputs,
    ValidationWorkbenchLaunch,
};

#[path = "support/live_view_product_fixtures.rs"]
mod live_view_product_fixtures;

#[test]
fn mounted_product_view_carries_runtime_graph_and_dependency_proof() {
    let app = prepared_app_with_live_view_source(live_view_product_fixtures::contact_form_source());
    let proof = app
        .live_view_projection_proof()
        .expect("projection admits through runtime");
    let mounted = proof.mounted_product_view();
    let counters = mounted.counters();

    assert_eq!(
        mounted.semantic_slice(),
        WorthUiMountedProductViewSemanticSlice::LiveView
    );
    assert!(mounted.receipt_digest() != 0);
    assert!(mounted
        .consumed_facts()
        .iter()
        .any(|fact| { fact.family() == WorthUiRuntimeFactFamily::LiveViewDeclaration }));
    assert!(counters.selected_graph_obligation_count() > 0);
    assert_eq!(counters.source_reparse_count(), 0);
    assert_eq!(counters.renderer_parse_count(), 0);
    let composition_tree = mounted.composition_tree();
    assert_eq!(mounted.root_entries().len(), 1);
    assert_eq!(counters.root_entry_count(), 1);
    let root_entry = &mounted.root_entries()[0];
    assert_eq!(
        root_entry.root_mount().root_kind(),
        WorthUiCompositionRootKind::PageContentSlot
    );
    assert_eq!(
        root_entry.root_mount().resolved_authority().slot_name(),
        Some("button_proof")
    );
    assert_eq!(
        root_entry.root_mount().counters().page_slot_lookup_count(),
        1
    );
    assert_eq!(root_entry.root_mount().counters().page_slot_scan_count(), 0);
    assert_eq!(
        root_entry.composition_tree_digest(),
        composition_tree.receipt_digest()
    );
    assert!(root_entry
        .root_mount()
        .consumed_facts()
        .iter()
        .any(|fact| fact.family() == WorthUiRuntimeFactFamily::PageContentSlot));
    assert!(root_entry
        .root_mount()
        .consumed_facts()
        .iter()
        .any(|fact| fact.family() == WorthUiRuntimeFactFamily::SurfaceMount));
    let traversal_counters = composition_tree.counters();
    let graph_access_counters = composition_tree.graph_access().counters();
    assert_eq!(graph_access_counters.caller_owned_recursive_walk_count(), 0);
    assert_eq!(graph_access_counters.source_reparse_count(), 0);
    assert_eq!(graph_access_counters.renderer_parse_count(), 0);
    assert_eq!(
        composition_tree
            .graph_access()
            .plan()
            .query_graph_execution()
            .selected_obligation_count(),
        5
    );
    assert_eq!(
        traversal_counters.mounted_node_index_entry_count(),
        counters.mounted_node_count()
    );
    assert_eq!(
        traversal_counters.child_index_entry_count(),
        counters.composition_edge_count()
    );
    assert_eq!(traversal_counters.flat_node_scan_count(), 0);
    assert_eq!(traversal_counters.source_reparse_count(), 0);
    assert_eq!(traversal_counters.renderer_parse_count(), 0);
    assert_eq!(
        mounted.graph_obligation_execution_digests().len(),
        proof.controls().len()
            + proof.conditionals().len()
            + proof.readinesses().len()
            + proof.payloads().len()
            + proof.interactions().len()
            + 4
    );
    let root_children = composition_tree.root_children();
    assert_eq!(root_children.len(), 1);
    assert_eq!(
        root_children[0].composition_node().kind(),
        WorthUiCompositionNodeKind::Surface
    );
    let surface_children = composition_tree.ordered_children(root_children[0].node_id());
    assert_eq!(surface_children.len(), 3);
    let input_stack = surface_children
        .iter()
        .find(|child| child.node_id() == "input_stack")
        .expect("surface mounts authored input stack through composition tree");
    let input_children = composition_tree.ordered_children(input_stack.node_id());
    assert_eq!(
        input_children
            .iter()
            .map(|child| child.composition_node().kind())
            .collect::<Vec<_>>(),
        vec![
            WorthUiCompositionNodeKind::Control,
            WorthUiCompositionNodeKind::Control
        ]
    );
    assert!(input_children.iter().any(|child| {
        matches!(child.mounted_node(), WorthUiMountedNodeReceipt::Control(frame) if frame.control_id() == "first_name_input")
    }));
    let action_row = surface_children
        .iter()
        .find(|child| child.node_id() == "action_row")
        .expect("surface mounts authored action row through composition tree");
    let action_children = composition_tree.ordered_children(action_row.node_id());
    assert!(action_children.iter().any(|child| {
        matches!(child.mounted_node(), WorthUiMountedNodeReceipt::Interaction(interaction) if interaction.interaction().interaction_id() == "contact_submit")
    }));
    let evidence = surface_children
        .iter()
        .find_map(|child| match child.mounted_node() {
            WorthUiMountedNodeReceipt::Evidence(evidence) => Some(evidence),
            _ => None,
        })
        .expect("mounted product view includes evidence");
    assert!(evidence
        .rows()
        .iter()
        .any(|row| row.label() == "projection"));
    assert!(evidence
        .rows()
        .iter()
        .any(|row| { row.label() == "control" && row.value().contains("first_name_input") }));
    assert!(evidence
        .rows()
        .iter()
        .any(|row| row.value().contains("graph=")));
}

#[test]
fn live_view_observation_evidence_is_runtime_mounted() {
    let mut app =
        prepared_app_with_live_view_source(live_view_product_fixtures::contact_form_source());
    let edit_intent = app
        .live_view_control_edit_intent("first_name", WorthUiLiveViewStateValue::text("Ada"))
        .expect("edit intent should resolve through live-view binding");
    let edit_receipt = app
        .workbench_mut()
        .runtime_mut()
        .apply_live_view_state_edit(edit_intent)
        .expect("edit should apply through runtime state");
    let projection = app
        .live_view_projection_proof()
        .expect("projection should still admit");
    let submit_interaction =
        mounted_interaction_child(projection.mounted_product_view().composition_tree())
            .expect("submit interaction is mounted");
    let submit_denial = app
        .workbench()
        .runtime()
        .activate_mounted_live_view_interaction(submit_interaction)
        .expect_err("missing contact mode keeps submit readiness denied");

    let evidence = app
        .workbench()
        .runtime()
        .mount_live_view_observation_evidence(
            Some(&edit_receipt),
            None,
            None,
            Some(&submit_denial),
            Some("parse denied"),
        );

    assert_eq!(evidence.semantic_slice(), "LiveViewObservations");
    assert!(evidence.rows().iter().any(|row| {
        row.label() == "live_view_edit"
            && row.value().contains("binding=first_name")
            && row.value().contains("graph=")
    }));
    assert!(evidence.rows().iter().any(|row| {
        row.label() == "live_view_submission_denial"
            && row
                .value()
                .contains("code=live_view.submit.readiness_denied")
    }));
    assert!(evidence.rows().iter().any(|row| {
        row.label() == "live_view_source_denial"
            && row.value().contains("code=live_view.source.rejected")
    }));
}

#[test]
fn live_view_source_reload_proves_composition_topology_rebind() {
    let mut app =
        prepared_app_with_live_view_source(live_view_product_fixtures::contact_form_source());
    let proof = app
        .hot_reload_live_view_source_with_composition_proof(
            live_view_product_fixtures::contact_form_source_with_action_before_inputs(),
        )
        .expect("source reload should admit and produce composition proof");

    assert_ne!(
        proof.prior_product_view().composition_graph_digest(),
        proof.next_product_view().composition_graph_digest()
    );
    assert_eq!(
        proof.projection_rebind().counters().source_reparse_count(),
        0
    );
    assert_eq!(
        proof.projection_rebind().counters().renderer_parse_count(),
        0
    );
    assert_eq!(proof.counters().source_reparse_count(), 0);
    assert_eq!(proof.counters().renderer_parse_count(), 0);
    assert!(proof.counters().compared_child_row_count() > 0);
    assert!(proof.counters().rebind_row_count() > 0);
    assert!(proof.counters().preserve_row_count() > 0);
    assert!(proof.rows().iter().any(|row| {
        row.semantic_slice() == "MountedCompositionTree"
            && row.decision() == ValidationLiveViewCompositionRebindDecision::Rebind
    }));
}

#[test]
fn stale_page_slot_root_rejects_before_product_mount() {
    let app = prepared_app_with_live_view_source(live_view_product_fixtures::contact_form_source());
    let graph = WorthUiCompositionGraphDefinition::for_root(WorthUiCompositionRootDefinition::new(
        WorthUiCompositionRootKind::PageContentSlot,
        "missing_slot",
    ))
    .admit()
    .expect("empty graph still admits enough to test root mount authority");
    let report = app
        .workbench()
        .runtime()
        .admit_composition_root_mount(app.workbench().page_host_plan(), graph.root())
        .expect_err("stale page slot roots must deny before mounted product construction");

    assert_eq!(report.denials().len(), 1);
    assert_eq!(
        report.denials()[0].code(),
        WorthUiCompositionRootMountDenialCode::MissingPageSlot
    );
    assert_eq!(report.denials()[0].subject(), "missing_slot");
    assert_ne!(report.denial_set_digest(), 0);
}

#[test]
fn unmounted_component_root_rejects_through_root_mount_report() {
    let app = prepared_app_with_live_view_source(live_view_product_fixtures::contact_form_source());
    let graph = WorthUiCompositionGraphDefinition::for_root(WorthUiCompositionRootDefinition::new(
        WorthUiCompositionRootKind::ComponentInstance,
        "validation.component.instance",
    ))
    .admit()
    .expect("unsupported root kind still has a typed graph root");
    let report = app
        .workbench()
        .runtime()
        .admit_composition_root_mount(app.workbench().page_host_plan(), graph.root())
        .expect_err("unmounted component roots must deny as root mount posture");

    assert_eq!(
        report.denials()[0].code(),
        WorthUiCompositionRootMountDenialCode::MissingComponentInstance
    );
}

#[test]
fn unmounted_future_root_kinds_reject_with_specific_authority_denials() {
    let app = prepared_app_with_live_view_source(live_view_product_fixtures::contact_form_source());
    for (kind, expected_code, identity) in [
        (
            WorthUiCompositionRootKind::PortalEntry,
            WorthUiCompositionRootMountDenialCode::MissingPortalEntry,
            "validation.portal.entry",
        ),
        (
            WorthUiCompositionRootKind::CollectionItem,
            WorthUiCompositionRootMountDenialCode::MissingCollectionItem,
            "validation.collection.item",
        ),
        (
            WorthUiCompositionRootKind::DiagnosticPanel,
            WorthUiCompositionRootMountDenialCode::MissingDiagnosticPanel,
            "validation.diagnostic.panel",
        ),
    ] {
        let graph = WorthUiCompositionGraphDefinition::for_root(
            WorthUiCompositionRootDefinition::new(kind, identity),
        )
        .admit()
        .expect("known root kind has a typed graph root");
        let report = app
            .workbench()
            .runtime()
            .admit_composition_root_mount(app.workbench().page_host_plan(), graph.root())
            .expect_err("unmounted known roots must deny through root mount authority");

        assert_eq!(report.denials()[0].code(), expected_code);
        assert_eq!(report.denials()[0].subject(), identity);
    }
}

fn prepared_app_with_live_view_source(
    source_text: String,
) -> worth_ui_validation_app::ValidationWorkbenchApp {
    let launch = ValidationWorkbenchLaunch::new()
        .prepare_from_authored_inputs(
            ValidationWorkbenchAuthoredInputs::sample()
                .with_live_view_source(ValidationLiveViewSource::new(source_text)),
        )
        .expect("validation app should prepare");
    worth_ui_validation_app::ValidationWorkbenchApp::new(launch)
}

fn mounted_interaction_child(
    tree: &worth_ui::facade::WorthUiMountedCompositionTreeReceipt,
) -> Option<&worth_ui::facade::WorthUiMountedInteractionNodeReceipt> {
    fn visit<'a>(
        tree: &'a worth_ui::facade::WorthUiMountedCompositionTreeReceipt,
        parent_id: &str,
    ) -> Option<&'a worth_ui::facade::WorthUiMountedInteractionNodeReceipt> {
        for child in tree.ordered_children(parent_id) {
            if let WorthUiMountedNodeReceipt::Interaction(node) = child.mounted_node() {
                return Some(node);
            }
            if let Some(node) = visit(tree, child.node_id()) {
                return Some(node);
            }
        }
        None
    }
    visit(tree, tree.root().root_id().as_str())
}
