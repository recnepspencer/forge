use worth_ui::facade::{
    admit_composition_context_propagation, compare_composition_context_propagation,
    WorthUiCompositionContextDefinition, WorthUiCompositionContextDenialCode,
    WorthUiCompositionGraphDefinition, WorthUiCompositionLocalePosture,
    WorthUiCompositionNodeDefinition, WorthUiCompositionNodeKind, WorthUiCompositionPolicyKind,
    WorthUiCompositionRootDefinition, WorthUiQueryGraphObligationSemantic,
    WorthUiRuntimeFactFamily,
};

#[test]
fn root_disabled_context_suppresses_descendant_interactions() {
    let definition = contact_card_graph().with_context(
        WorthUiCompositionContextDefinition::root()
            .disabled(true)
            .theme("validation.theme.dark"),
    );
    let graph = definition.clone().admit().expect("graph should admit");
    let propagation =
        admit_composition_context_propagation(&graph, definition.context_definitions())
            .expect("context propagation should admit");

    let submit = propagation
        .context_for_node("submit")
        .expect("submit node context should exist");
    assert!(submit.disabled());
    assert!(submit.suppresses_interaction());
    assert_eq!(submit.theme(), Some("validation.theme.dark"));
    assert_eq!(propagation.node_contexts().len(), graph.nodes().len());
    assert_eq!(propagation.counters().selected_graph_obligation_count(), 5);
    assert_eq!(propagation.counters().source_reparse_count(), 0);
    assert_eq!(propagation.counters().renderer_parse_count(), 0);
    assert!(propagation
        .affected_consumers()
        .iter()
        .any(|row| row.changed_fact().family() == WorthUiRuntimeFactFamily::CompositionContext));
}

#[test]
fn disabled_context_override_requires_explicit_policy() {
    let definition = contact_card_graph()
        .with_context(WorthUiCompositionContextDefinition::root().disabled(true))
        .with_context(WorthUiCompositionContextDefinition::for_node("card").disabled(false));
    let graph = definition.clone().admit().expect("graph should admit");

    let report = admit_composition_context_propagation(&graph, definition.context_definitions())
        .expect_err("breaking disabled inheritance needs override policy");

    assert_eq!(report.denials().len(), 1);
    assert_eq!(
        report.denials()[0].code(),
        WorthUiCompositionContextDenialCode::OverrideWithoutEligibility
    );
    assert_eq!(report.denials()[0].context_kind(), "disabled");
    assert_eq!(report.denials()[0].attempted_value(), Some("false"));
    assert_eq!(
        report.denials()[0].expected_policy(),
        "breaking disabled or inert inheritance requires allow_local_override"
    );
    assert_eq!(
        report.denials()[0].affected_descendants(),
        &vec![
            "card".to_owned(),
            "first_name".to_owned(),
            "submit".to_owned()
        ]
    );
    assert!(report
        .presentation_rows()
        .iter()
        .any(|row| { row.label() == "affected_descendants" && row.value().contains("submit") }));
    assert_ne!(report.report_digest(), 0);
}

#[test]
fn multiple_context_denials_batch_with_typed_context_details() {
    let definition = contact_card_graph()
        .with_context(
            WorthUiCompositionContextDefinition::for_node("missing").theme("validation.theme.dark"),
        )
        .with_context(
            WorthUiCompositionContextDefinition::for_node("card")
                .disabled(false)
                .inert(false),
        );
    let graph = definition.clone().admit().expect("graph should admit");

    let report = admit_composition_context_propagation(&graph, definition.context_definitions())
        .expect_err("all context denials should batch");

    assert_eq!(report.denials().len(), 3);
    assert!(report
        .denials()
        .iter()
        .any(|denial| denial.code() == WorthUiCompositionContextDenialCode::MissingScopeNode));
    assert!(report.denials().iter().any(|denial| {
        denial.code() == WorthUiCompositionContextDenialCode::OverrideWithoutEligibility
            && denial.context_kind() == "disabled"
    }));
    assert!(report.denials().iter().any(|denial| {
        denial.code() == WorthUiCompositionContextDenialCode::OverrideWithoutEligibility
            && denial.context_kind() == "inert"
    }));
    assert_ne!(report.report_digest(), 0);
}

#[test]
fn explicit_disabled_override_reenables_descendant_context() {
    let definition = contact_card_graph()
        .with_context(WorthUiCompositionContextDefinition::root().disabled(true))
        .with_context(
            WorthUiCompositionContextDefinition::for_node("card")
                .allow_local_override()
                .disabled(false),
        );
    let graph = definition.clone().admit().expect("graph should admit");
    let propagation =
        admit_composition_context_propagation(&graph, definition.context_definitions())
            .expect("override policy should admit");

    let submit = propagation
        .context_for_node("submit")
        .expect("submit node context should exist");
    assert!(!submit.disabled());
    assert!(!submit.suppresses_interaction());
    assert_eq!(propagation.overrides().len(), 1);
    assert_eq!(propagation.overrides()[0].context_kind(), "disabled");
}

#[test]
fn scoped_theme_density_and_locale_changes_touch_only_descendant_contexts() {
    let base_definition = contact_card_graph()
        .with_node(WorthUiCompositionNodeDefinition::container("sidebar"))
        .with_root_child("sidebar")
        .with_context(
            WorthUiCompositionContextDefinition::for_node("card")
                .theme("validation.theme.light")
                .density("validation.density.default")
                .locale(WorthUiCompositionLocalePosture::Limited("en-US".to_owned())),
        );
    let changed_definition = contact_card_graph()
        .with_node(WorthUiCompositionNodeDefinition::container("sidebar"))
        .with_root_child("sidebar")
        .with_context(
            WorthUiCompositionContextDefinition::for_node("card")
                .theme("validation.theme.dark")
                .density("validation.density.compact")
                .locale(WorthUiCompositionLocalePosture::Limited("ar".to_owned())),
        );
    let base_graph = base_definition.clone().admit().expect("graph should admit");
    let changed_graph = changed_definition
        .clone()
        .admit()
        .expect("graph should admit");
    let base =
        admit_composition_context_propagation(&base_graph, base_definition.context_definitions())
            .expect("base context should admit");
    let changed = admit_composition_context_propagation(
        &changed_graph,
        changed_definition.context_definitions(),
    )
    .expect("changed context should admit");

    for node_id in ["card", "first_name", "submit"] {
        assert_ne!(
            context_digest(&base, node_id),
            context_digest(&changed, node_id),
            "{node_id} should consume the scoped context change"
        );
    }
    assert_eq!(
        context_digest(&base, "sidebar"),
        context_digest(&changed, "sidebar"),
        "unrelated sibling context must preserve its receipt"
    );
    let submit_context = changed
        .context_for_node("submit")
        .expect("submit context exists");
    assert_eq!(submit_context.theme(), Some("validation.theme.dark"));
    assert_eq!(submit_context.density(), Some("validation.density.compact"));
    assert_eq!(
        submit_context.locale(),
        &WorthUiCompositionLocalePosture::Limited("ar".to_owned())
    );
}

#[test]
fn context_delta_names_changed_descendants_and_preserved_siblings() {
    let base_definition = contact_card_graph()
        .with_node(WorthUiCompositionNodeDefinition::container("sidebar"))
        .with_root_child("sidebar")
        .with_context(
            WorthUiCompositionContextDefinition::for_node("card")
                .density("validation.density.default"),
        );
    let changed_definition = contact_card_graph()
        .with_node(WorthUiCompositionNodeDefinition::container("sidebar"))
        .with_root_child("sidebar")
        .with_context(
            WorthUiCompositionContextDefinition::for_node("card")
                .density("validation.density.compact"),
        );
    let base_graph = base_definition.clone().admit().expect("graph admits");
    let changed_graph = changed_definition.clone().admit().expect("graph admits");
    let base =
        admit_composition_context_propagation(&base_graph, base_definition.context_definitions())
            .expect("base context admits");
    let changed = admit_composition_context_propagation(
        &changed_graph,
        changed_definition.context_definitions(),
    )
    .expect("changed context admits");

    let delta = compare_composition_context_propagation(&base, &changed);

    assert_eq!(delta.counters().source_reparse_count(), 0);
    assert_eq!(delta.counters().renderer_parse_count(), 0);
    assert!(delta
        .affected_descendant_nodes()
        .contains(&"submit".to_owned()));
    assert!(delta
        .preserved_sibling_nodes()
        .contains(&"sidebar".to_owned()));
    assert!(delta
        .consumer_intersections()
        .iter()
        .any(|row| row.node_id() == "submit"));
    assert_ne!(delta.query_graph_execution_digest(), 0);
}

#[test]
fn real_context_propagation_exposes_query_graph_obligations() {
    let definition = contact_card_graph()
        .with_context(WorthUiCompositionContextDefinition::root().disabled(true));
    let graph = definition.clone().admit().expect("graph admits");
    let propagation =
        admit_composition_context_propagation(&graph, definition.context_definitions())
            .expect("context propagation admits");
    let semantics = propagation
        .query_graph_execution()
        .rows()
        .iter()
        .map(|row| row.semantic())
        .collect::<Vec<_>>();

    for expected in WorthUiQueryGraphObligationSemantic::COMPOSITION_CONTEXT {
        assert!(
            semantics.contains(&expected),
            "missing real context graph obligation {expected:?}"
        );
    }
    assert!(propagation
        .query_graph_execution()
        .touch_descriptor()
        .surface_id()
        .contains("worth.surface.context.contact"));
    assert!(!propagation
        .query_graph_execution()
        .touch_descriptor()
        .descriptor_digest()
        .is_empty());
}

fn context_digest(
    propagation: &worth_ui::facade::WorthUiCompositionContextPropagationReceipt,
    node_id: &str,
) -> u64 {
    propagation
        .context_for_node(node_id)
        .expect("context exists")
        .receipt_digest()
}

fn contact_card_graph() -> WorthUiCompositionGraphDefinition {
    WorthUiCompositionGraphDefinition::for_root(WorthUiCompositionRootDefinition::surface(
        "worth.surface.context.contact",
    ))
    .with_node(WorthUiCompositionNodeDefinition::container("card"))
    .with_node(WorthUiCompositionNodeDefinition::new(
        WorthUiCompositionNodeKind::Control,
        "first_name",
        "first_name",
    ))
    .with_node(WorthUiCompositionNodeDefinition::new(
        WorthUiCompositionNodeKind::Interaction,
        "submit",
        "submit",
    ))
    .with_root_child("card")
    .with_parent("card", "first_name")
    .with_parent("card", "submit")
    .with_policy_attachment("card", WorthUiCompositionPolicyKind::LocalLayout, "stack")
}
