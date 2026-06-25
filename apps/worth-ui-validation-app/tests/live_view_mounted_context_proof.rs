use worth_ui::facade::{
    admit_composition_context_propagation, WorthUiCompositionContextDefinition,
    WorthUiLiveViewInteractionActivationDenial, WorthUiMountedNodeReceipt,
    WorthUiPrimitiveEventCursor, WorthUiPrimitiveFocusPosture, WorthUiRuntimeFactFamily,
};
use worth_ui_validation_app::reload::ValidationLiveViewSource;
use worth_ui_validation_app::{ValidationWorkbenchAuthoredInputs, ValidationWorkbenchLaunch};

#[path = "support/live_view_product_fixtures.rs"]
#[allow(dead_code)]
mod live_view_product_fixtures;

#[test]
fn mounted_product_view_accepts_explicit_context_propagation_receipt() {
    let app = prepared_app_with_live_view_source(live_view_product_fixtures::contact_form_source());
    let proof = app
        .live_view_projection_proof()
        .expect("projection admits through runtime");
    let graph = proof.graph_backed_projection().composition_graph();
    let context = admit_composition_context_propagation(
        graph,
        &[WorthUiCompositionContextDefinition::root().disabled(true)],
    )
    .expect("disabled root context should propagate");

    let mounted = app
        .workbench()
        .runtime()
        .mount_live_view_product_projection_for_page_with_context(
            app.workbench().page_host_plan(),
            proof.graph_backed_projection(),
            context.clone(),
        )
        .expect("context-backed mounted product should resolve through page authority");

    assert!(mounted
        .graph_obligation_execution_digests()
        .contains(&context.query_graph_execution().execution_digest()));
    assert!(mounted
        .consumed_facts()
        .iter()
        .any(|fact| fact.family() == WorthUiRuntimeFactFamily::CompositionContext));
    assert_ne!(
        mounted.receipt_digest(),
        proof.mounted_product_view().receipt_digest()
    );
}

#[test]
fn mounted_context_suppression_blocks_live_view_activation() {
    let app = prepared_app_with_live_view_source(live_view_product_fixtures::contact_form_source());
    let proof = app
        .live_view_projection_proof()
        .expect("projection admits through runtime");
    let context = admit_composition_context_propagation(
        proof.graph_backed_projection().composition_graph(),
        &[WorthUiCompositionContextDefinition::root().disabled(true)],
    )
    .expect("disabled root context should propagate");
    let mounted = app
        .workbench()
        .runtime()
        .mount_live_view_product_projection_for_page_with_context(
            app.workbench().page_host_plan(),
            proof.graph_backed_projection(),
            context,
        )
        .expect("context-backed mounted product should resolve through page authority");
    let submit = mounted_interaction_child(mounted.composition_tree())
        .expect("mounted composition tree includes submit interaction");

    let denial = app
        .workbench()
        .runtime()
        .activate_mounted_live_view_interaction(submit)
        .expect_err("context-suppressed mounted interaction must not activate");

    assert!(matches!(
        denial,
        WorthUiLiveViewInteractionActivationDenial::ContextSuppressed {
            disabled: true,
            inert: false,
            ..
        }
    ));
}

#[test]
fn authored_source_context_suppresses_mounted_event_posture() {
    let app = prepared_app_with_live_view_source(disabled_root_context_source());
    let proof = app
        .live_view_projection_proof()
        .expect("authored source context should admit through projection");
    let mounted = proof.mounted_product_view();
    let submit = mounted_interaction_child(mounted.composition_tree())
        .expect("mounted composition tree includes submit interaction");
    let event_posture = submit.contextual_event_posture();

    assert!(submit.is_context_suppressed());
    assert!(!submit.is_enabled());
    assert!(!event_posture.activation_enabled());
    assert!(!event_posture.hover_enabled());
    assert!(!event_posture.press_enabled());
    assert_eq!(event_posture.focus(), WorthUiPrimitiveFocusPosture::None);
    assert_eq!(event_posture.cursor(), WorthUiPrimitiveEventCursor::Default);
    assert!(submit
        .consumed_facts()
        .iter()
        .any(|fact| fact.family() == WorthUiRuntimeFactFamily::CompositionContext));

    let denial = app
        .workbench()
        .runtime()
        .activate_mounted_live_view_interaction(submit)
        .expect_err("authored context-suppressed interaction must not activate");

    assert!(matches!(
        denial,
        WorthUiLiveViewInteractionActivationDenial::ContextSuppressed {
            disabled: true,
            inert: false,
            ..
        }
    ));
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

fn disabled_root_context_source() -> String {
    live_view_product_fixtures::contact_form_source().replace(
        "        root page_content_slot button_proof",
        "        root page_content_slot button_proof
        context root {
            disabled true
        }",
    )
}
