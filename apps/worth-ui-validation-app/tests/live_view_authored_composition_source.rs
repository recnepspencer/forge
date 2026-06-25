use worth_ui::facade::{
    WorthUiAuthoredLiveViewDocument, WorthUiCompositionChildSizing,
    WorthUiCompositionContextDenialCode, WorthUiCompositionGraphDefinition,
    WorthUiCompositionNodeDefinition, WorthUiCompositionNodeKind, WorthUiCompositionPolicyKind,
    WorthUiCompositionRootDefinition, WorthUiCompositionRootKind,
    WorthUiCompositionSourceDenialCode, WorthUiLiveViewControlProjectionCompatibilityReceipt,
    WorthUiLiveViewProjectionAdmissionDenial, WorthUiRuntimeFactFamily,
};
use worth_ui_validation_app::reload::ValidationLiveViewSource;
use worth_ui_validation_app::{ValidationWorkbenchAuthoredInputs, ValidationWorkbenchLaunch};

#[test]
fn authored_composition_controls_nested_card_topology() {
    let app = prepared_app_with_live_view_source(contact_source());
    let proof = app
        .live_view_projection_proof()
        .expect("authored composition should admit");
    let mounted = proof.mounted_product_view();
    let tree = mounted.composition_tree();
    let root = tree.root_children();
    assert_eq!(root.len(), 1);
    assert_eq!(root[0].node_id(), "live_view.form_card");

    let surface_children = tree.ordered_children(root[0].node_id());
    assert_eq!(
        surface_children
            .iter()
            .map(|child| child.node_id())
            .collect::<Vec<_>>(),
        vec!["input_stack", "action_row", "live_view.evidence"]
    );
    let input_children = tree.ordered_children("input_stack");
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
    let action_children = tree.ordered_children("action_row");
    assert_eq!(action_children.len(), 1);
    assert_eq!(
        action_children[0].composition_node().kind(),
        WorthUiCompositionNodeKind::Interaction
    );
    assert!(mounted
        .consumed_facts()
        .iter()
        .any(|fact| fact.family() == WorthUiRuntimeFactFamily::CompositionEdge));
    let edges = proof
        .projection()
        .composition_graph()
        .expect("projection carries admitted authored graph")
        .edges();
    assert!(edges.iter().any(
        |edge| edge.child().as_str() == "live_view.control.title_input"
            && edge.sizing() == WorthUiCompositionChildSizing::Fill(1)
    ));
    assert!(edges.iter().any(
        |edge| edge.child().as_str() == "live_view.interaction.proof_submit"
            && edge.sizing() == WorthUiCompositionChildSizing::Hug
    ));
}

#[test]
fn changing_authored_composition_order_changes_composition_graph_digest() {
    let mut app = prepared_app_with_live_view_source(contact_source());
    let first = app
        .live_view_projection_proof()
        .expect("initial source admits")
        .mounted_product_view()
        .composition_graph_digest();
    let next = app
        .hot_reload_live_view_source(contact_source().replace(
            "child control title_input sizing fill(1)\n                child control details_input sizing fill(1)",
            "child control details_input sizing fill(1)\n                child control title_input sizing fill(1)",
        ))
        .expect("reordered authored composition should admit");
    assert_ne!(
        first,
        next.mounted_product_view().composition_graph_digest(),
        "sibling order is authored composition meaning"
    );
}

#[test]
fn file_authored_composition_matches_equivalent_rust_authored_graph() {
    let app = prepared_app_with_live_view_source(contact_source());
    let proof = app
        .live_view_projection_proof()
        .expect("authored composition should admit");
    let authored_graph = proof
        .projection()
        .composition_graph()
        .expect("projection carries authored composition graph");
    let rust_graph = equivalent_rust_graph()
        .admit()
        .expect("equivalent Rust graph should admit");
    assert_eq!(authored_graph.receipt_digest(), rust_graph.receipt_digest());
    assert_eq!(authored_graph.nodes(), rust_graph.nodes());
    assert_eq!(authored_graph.edges(), rust_graph.edges());
}

#[test]
fn invalid_authored_composition_rejects_before_mounting() {
    let denial = WorthUiAuthoredLiveViewDocument::parse(&contact_source().replace(
        "child control title_input sizing fill(1)",
        "child widget title_input sizing fill(1)",
    ))
    .expect_err("unknown composition child kind should deny before projection admission");
    assert!(
        denial.message().contains("composition child kind"),
        "denial should name composition source syntax: {}",
        denial.message()
    );
}

#[test]
fn invalid_composition_sizing_rejects_at_source_boundary() {
    let denial = WorthUiAuthoredLiveViewDocument::parse(&contact_source().replace(
        "child control title_input sizing fill(1)",
        "child control title_input sizing stretchy",
    ))
    .expect_err("invalid child sizing must deny before graph admission");
    assert!(
        denial.message().contains("sizing"),
        "denial should name child sizing syntax: {}",
        denial.message()
    );
}

#[test]
fn stale_composition_control_reference_rejects_before_mounting() {
    let app = prepared_app_with_live_view_source(contact_source().replace(
        "child control title_input sizing fill(1)",
        "child control missing_input sizing fill(1)",
    ));
    let report = app
        .live_view_projection_proof_typed()
        .expect_err("composition child must reference an authored control");
    assert_eq!(report.counters().denial_count(), 1);
    let [WorthUiLiveViewProjectionAdmissionDenial::CompositionSource(denial)] = report.denials()
    else {
        panic!("expected one typed composition source denial: {report:?}");
    };
    assert_eq!(
        denial.code(),
        WorthUiCompositionSourceDenialCode::StaleControlReference
    );
    assert_eq!(denial.subject(), "live_view.control.missing_input");
    assert_eq!(
        denial.expected_syntax(),
        "child control <authored-control-id>"
    );
    assert!(
        denial.source_span().is_some(),
        "composition source denials must carry source span readiness"
    );
    assert_ne!(denial.denial_digest(), 0);
}

#[test]
fn multiple_stale_composition_references_report_in_source_order() {
    let app = prepared_app_with_live_view_source(
        contact_source()
            .replace(
                "child control title_input sizing fill(1)",
                "child control missing_input sizing fill(1)",
            )
            .replace(
                "child interaction proof_submit sizing hug",
                "child interaction missing_submit sizing hug",
            ),
    );
    let report = app
        .live_view_projection_proof_typed()
        .expect_err("composition source should batch stale references");
    assert_eq!(report.counters().denial_count(), 2);
    assert_ne!(report.denial_set_digest(), 0);
    let denials = report
        .denials()
        .iter()
        .map(|denial| match denial {
            WorthUiLiveViewProjectionAdmissionDenial::CompositionSource(denial) => denial,
            other => panic!("expected only composition source denials: {other:?}"),
        })
        .collect::<Vec<_>>();
    assert_eq!(
        denials
            .iter()
            .map(|denial| denial.code())
            .collect::<Vec<_>>(),
        vec![
            WorthUiCompositionSourceDenialCode::StaleControlReference,
            WorthUiCompositionSourceDenialCode::StaleInteractionReference
        ]
    );
    let first_span = denials[0].source_span().expect("first denial span");
    let second_span = denials[1].source_span().expect("second denial span");
    assert!(
        first_span.start_byte() < second_span.start_byte(),
        "batched source denials must remain in authored source order"
    );
}

#[test]
fn authored_composition_context_denials_join_projection_report() {
    let app = prepared_app_with_live_view_source(contact_source().replace(
        "root page_content_slot button_proof",
        "root page_content_slot button_proof
        context root {
            disabled true
        }
        context node live_view.form_card {
            disabled false
        }",
    ));
    let report = app
        .live_view_projection_proof_typed()
        .expect_err("invalid authored composition context should deny projection");
    assert_eq!(report.counters().denial_count(), 1);
    let [WorthUiLiveViewProjectionAdmissionDenial::CompositionContext(denial)] = report.denials()
    else {
        panic!("expected one typed composition context denial: {report:?}");
    };
    assert_eq!(
        denial.code(),
        WorthUiCompositionContextDenialCode::OverrideWithoutEligibility
    );
    assert_eq!(denial.context_kind(), "disabled");
    assert_eq!(denial.attempted_value(), Some("false"));
    assert!(denial.source_span_ready());
    assert!(denial
        .affected_descendants()
        .contains(&"live_view.interaction.proof_submit".to_owned()));
    assert_ne!(denial.denial_digest(), 0);
    assert_ne!(report.denial_set_digest(), 0);
}

#[test]
fn hot_reloading_text_input_to_dropdown_preserves_control_identity() {
    let mut app = prepared_app_with_live_view_source(contact_source());
    let next = app
        .hot_reload_live_view_source(contact_source().replace(
            "projection text_input\n        label \"Details\"",
            "projection select\n        label \"Details\"\n        options yes:Yes,no:No",
        ))
        .expect("text input to dropdown should admit through hot reload");
    let rebind = next.last_rebind().expect("hot reload should carry rebind");
    let control_rebind = rebind.control_rebind();
    assert_eq!(control_rebind.counters().changed_control_count(), 1);
    assert_eq!(control_rebind.counters().source_reparse_count(), 0);
    assert_eq!(control_rebind.counters().renderer_parse_count(), 0);
    assert_eq!(
        control_rebind.compatibility(),
        WorthUiLiveViewControlProjectionCompatibilityReceipt::Preserved
    );
    let row = control_rebind
        .compatibility_rows()
        .iter()
        .find(|row| row.control_id() == "details_input")
        .expect("details_input compatibility row");
    assert_eq!(row.prior_kind(), Some("text_input"));
    assert_eq!(row.next_kind(), Some("select"));
    assert_eq!(
        row.compatibility(),
        WorthUiLiveViewControlProjectionCompatibilityReceipt::Preserved
    );
    assert!(control_rebind
        .changed_facts()
        .facts()
        .facts()
        .any(|fact| fact.identity().contains("details_input")));
}

fn prepared_app_with_live_view_source(
    source: impl Into<String>,
) -> worth_ui_validation_app::ValidationWorkbenchApp {
    let authored_inputs = ValidationWorkbenchAuthoredInputs::sample()
        .with_live_view_source(ValidationLiveViewSource::new(source.into()));
    let launch = ValidationWorkbenchLaunch::new()
        .prepare_from_authored_inputs(authored_inputs)
        .expect("validation app should prepare");
    worth_ui_validation_app::ValidationWorkbenchApp::new(launch)
}

fn contact_source() -> String {
    worth_ui_validation_app::reload::VALIDATION_SAMPLE_LIVE_VIEW_SOURCE.to_owned()
}

fn equivalent_rust_graph() -> WorthUiCompositionGraphDefinition {
    WorthUiCompositionGraphDefinition::for_root(WorthUiCompositionRootDefinition::new(
        WorthUiCompositionRootKind::PageContentSlot,
        "button_proof",
    ))
    .with_node(WorthUiCompositionNodeDefinition::new(
        WorthUiCompositionNodeKind::Surface,
        "live_view.form_card",
        "live_view.form_card",
    ))
    .with_node(WorthUiCompositionNodeDefinition::container("input_stack"))
    .with_node(WorthUiCompositionNodeDefinition::new(
        WorthUiCompositionNodeKind::Control,
        "live_view.control.title_input",
        "title_input",
    ))
    .with_node(WorthUiCompositionNodeDefinition::new(
        WorthUiCompositionNodeKind::Control,
        "live_view.control.details_input",
        "details_input",
    ))
    .with_node(WorthUiCompositionNodeDefinition::container("action_row"))
    .with_node(WorthUiCompositionNodeDefinition::new(
        WorthUiCompositionNodeKind::Interaction,
        "live_view.interaction.proof_submit",
        "proof_submit",
    ))
    .with_node(WorthUiCompositionNodeDefinition::new(
        WorthUiCompositionNodeKind::DiagnosticPanel,
        "live_view.evidence",
        "live_view.evidence",
    ))
    .with_root_child("live_view.form_card")
    .with_parent("live_view.form_card", "input_stack")
    .with_parent_at_with_sizing(
        "input_stack",
        "live_view.control.title_input",
        0,
        WorthUiCompositionChildSizing::Fill(1),
    )
    .with_parent_at_with_sizing(
        "input_stack",
        "live_view.control.details_input",
        1,
        WorthUiCompositionChildSizing::Fill(1),
    )
    .with_parent("live_view.form_card", "action_row")
    .with_parent_at_with_sizing(
        "action_row",
        "live_view.interaction.proof_submit",
        0,
        WorthUiCompositionChildSizing::Hug,
    )
    .with_parent("live_view.form_card", "live_view.evidence")
    .with_policy_attachment(
        "live_view.form_card",
        WorthUiCompositionPolicyKind::LocalLayout,
        "validation.flow.form.card",
    )
    .with_policy_attachment(
        "input_stack",
        WorthUiCompositionPolicyKind::LocalLayout,
        "validation.flow.form.inputs",
    )
    .with_policy_attachment(
        "action_row",
        WorthUiCompositionPolicyKind::LocalLayout,
        "validation.flow.form.actions",
    )
}
