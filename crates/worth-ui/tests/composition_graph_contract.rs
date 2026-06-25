use worth_ui::facade::{
    WorthUiCompositionGraphDefinition, WorthUiCompositionGraphDenialCode,
    WorthUiCompositionNodeDefinition, WorthUiCompositionNodeKind, WorthUiCompositionPolicyKind,
    WorthUiCompositionRootDefinition, WorthUiRuntimeFactFamily,
};

#[test]
fn equivalent_composition_graphs_lower_to_equivalent_receipts() {
    let first = contact_card_graph().admit().expect("graph should admit");
    let second = contact_card_graph().admit().expect("graph should admit");

    assert_eq!(first.receipt_digest(), second.receipt_digest());
    assert_eq!(first.root().root_id(), second.root().root_id());
    assert_eq!(first.nodes().len(), 4);
    assert_eq!(first.edges().len(), 4);
    assert_eq!(first.policy_attachments().len(), 1);
    assert_eq!(first.query_graph_execution().selected_obligation_count(), 5);
    assert_eq!(first.counters().selected_graph_obligation_count(), 5);
    assert_eq!(first.counters().policy_attachment_count(), 1);
    assert_eq!(first.counters().source_reparse_count(), 0);
    assert_eq!(first.counters().renderer_parse_count(), 0);
}

#[test]
fn declaration_order_does_not_change_equivalent_graph_receipts() {
    let canonical = contact_card_graph().admit().expect("graph should admit");
    let reordered = contact_card_graph_reordered_declarations()
        .admit()
        .expect("equivalent graph should admit");

    assert_eq!(canonical.receipt_digest(), reordered.receipt_digest());
    assert_eq!(canonical.nodes(), reordered.nodes());
    assert_eq!(canonical.edges(), reordered.edges());
    assert_eq!(
        canonical.policy_attachments(),
        reordered.policy_attachments()
    );
}

#[test]
fn sibling_order_is_semantic_and_changes_receipts() {
    let canonical = contact_card_graph().admit().expect("graph should admit");
    let reordered_children = contact_card_graph_reordered_children()
        .admit()
        .expect("graph with different child order should admit");

    assert_ne!(
        canonical.receipt_digest(),
        reordered_children.receipt_digest()
    );
    assert_ne!(canonical.edges(), reordered_children.edges());
}

#[test]
fn policy_identity_is_semantic_and_changes_receipts() {
    let canonical = contact_card_graph().admit().expect("graph should admit");
    let different_policy = contact_card_graph_with_layout_policy("dense-card-flow")
        .admit()
        .expect("graph with alternate policy identity should admit");

    assert_ne!(
        canonical.receipt_digest(),
        different_policy.receipt_digest()
    );
    assert_ne!(
        canonical.policy_attachments(),
        different_policy.policy_attachments()
    );
}

#[test]
fn graph_facts_are_typed_runtime_facts() {
    let receipt = contact_card_graph().admit().expect("graph should admit");
    let families = receipt
        .consumed_facts()
        .iter()
        .map(|fact| fact.family())
        .collect::<Vec<_>>();

    assert!(families.contains(&WorthUiRuntimeFactFamily::CompositionRoot));
    assert!(families.contains(&WorthUiRuntimeFactFamily::CompositionNode));
    assert!(families.contains(&WorthUiRuntimeFactFamily::CompositionEdge));
    assert!(families.contains(&WorthUiRuntimeFactFamily::CompositionPolicy));
}

#[test]
fn policy_attachments_reject_missing_nodes() {
    let denials = WorthUiCompositionGraphDefinition::for_root(
        WorthUiCompositionRootDefinition::surface("worth.surface.invalid.policy"),
    )
    .with_policy_attachment(
        "missing",
        WorthUiCompositionPolicyKind::LocalLayout,
        "layout-policy",
    )
    .admit()
    .expect_err("policy attachments must target admitted nodes");

    assert!(denials
        .iter()
        .any(|denial| denial.code() == WorthUiCompositionGraphDenialCode::MissingPolicyNode));
}

#[test]
fn duplicate_policy_attachment_kinds_reject() {
    let denials = WorthUiCompositionGraphDefinition::for_root(
        WorthUiCompositionRootDefinition::surface("worth.surface.invalid.policy.duplicate"),
    )
    .with_node(WorthUiCompositionNodeDefinition::container("card"))
    .with_root_child("card")
    .with_policy_attachment("card", WorthUiCompositionPolicyKind::LocalLayout, "first")
    .with_policy_attachment("card", WorthUiCompositionPolicyKind::LocalLayout, "second")
    .admit()
    .expect_err("a node cannot have two local layout authorities");

    assert!(denials.iter().any(
        |denial| denial.code() == WorthUiCompositionGraphDenialCode::DuplicatePolicyAttachment
    ));
}

#[test]
fn unsupported_policy_node_kind_rejects() {
    let denials = WorthUiCompositionGraphDefinition::for_root(
        WorthUiCompositionRootDefinition::surface("worth.surface.invalid.policy.kind"),
    )
    .with_node(WorthUiCompositionNodeDefinition::new(
        WorthUiCompositionNodeKind::Text,
        "title",
        "title",
    ))
    .with_root_child("title")
    .with_policy_attachment(
        "title",
        WorthUiCompositionPolicyKind::InteractionContainment,
        "interaction-policy",
    )
    .admit()
    .expect_err("interaction containment cannot attach to a text node");

    assert!(denials.iter().any(
        |denial| denial.code() == WorthUiCompositionGraphDenialCode::UnsupportedPolicyNodeKind
    ));
}

#[test]
fn invalid_graph_shapes_reject_before_receipts_exist() {
    let denials = WorthUiCompositionGraphDefinition::for_root(
        WorthUiCompositionRootDefinition::surface("worth.surface.invalid"),
    )
    .with_node(WorthUiCompositionNodeDefinition::new(
        WorthUiCompositionNodeKind::Text,
        "title",
        "title",
    ))
    .with_node(WorthUiCompositionNodeDefinition::new(
        WorthUiCompositionNodeKind::Control,
        "field",
        "field",
    ))
    .with_parent("title", "field")
    .admit()
    .expect_err("text nodes cannot parent controls");

    assert!(denials
        .iter()
        .any(|denial| denial.code() == WorthUiCompositionGraphDenialCode::UnsupportedParentKind));
    assert!(denials
        .iter()
        .any(|denial| denial.code() == WorthUiCompositionGraphDenialCode::UnmountedNode));
}

#[test]
fn duplicate_child_order_rejects() {
    let denials = WorthUiCompositionGraphDefinition::for_root(
        WorthUiCompositionRootDefinition::surface("worth.surface.invalid.order"),
    )
    .with_node(WorthUiCompositionNodeDefinition::container("card"))
    .with_node(WorthUiCompositionNodeDefinition::new(
        WorthUiCompositionNodeKind::Control,
        "first",
        "first",
    ))
    .with_node(WorthUiCompositionNodeDefinition::new(
        WorthUiCompositionNodeKind::Control,
        "second",
        "second",
    ))
    .with_root_child("card")
    .with_parent_at("card", "first", 0)
    .with_parent_at("card", "second", 0)
    .admit()
    .expect_err("duplicate order must reject");

    assert!(denials
        .iter()
        .any(|denial| denial.code() == WorthUiCompositionGraphDenialCode::DuplicateChildOrder));
}

#[test]
fn cycles_reject() {
    let denials = WorthUiCompositionGraphDefinition::for_root(
        WorthUiCompositionRootDefinition::surface("worth.surface.invalid.cycle"),
    )
    .with_node(WorthUiCompositionNodeDefinition::container("a"))
    .with_node(WorthUiCompositionNodeDefinition::container("b"))
    .with_root_child("a")
    .with_parent("a", "b")
    .with_parent("b", "a")
    .admit()
    .expect_err("cycles must reject");

    assert!(denials
        .iter()
        .any(|denial| denial.code() == WorthUiCompositionGraphDenialCode::Cycle));
}

fn contact_card_graph() -> WorthUiCompositionGraphDefinition {
    contact_card_graph_with_layout_policy("card-flow")
}

fn contact_card_graph_with_layout_policy(layout_policy: &str) -> WorthUiCompositionGraphDefinition {
    WorthUiCompositionGraphDefinition::for_root(WorthUiCompositionRootDefinition::surface(
        "worth.surface.contact",
    ))
    .with_node(WorthUiCompositionNodeDefinition::container("card"))
    .with_node(WorthUiCompositionNodeDefinition::new(
        WorthUiCompositionNodeKind::Control,
        "first_name",
        "first_name",
    ))
    .with_node(WorthUiCompositionNodeDefinition::new(
        WorthUiCompositionNodeKind::Control,
        "details",
        "details",
    ))
    .with_node(WorthUiCompositionNodeDefinition::new(
        WorthUiCompositionNodeKind::Interaction,
        "submit",
        "submit",
    ))
    .with_root_child("card")
    .with_parent("card", "first_name")
    .with_parent("card", "details")
    .with_parent("card", "submit")
    .with_policy_attachment(
        "card",
        WorthUiCompositionPolicyKind::LocalLayout,
        layout_policy,
    )
}

fn contact_card_graph_reordered_declarations() -> WorthUiCompositionGraphDefinition {
    WorthUiCompositionGraphDefinition::for_root(WorthUiCompositionRootDefinition::surface(
        "worth.surface.contact",
    ))
    .with_node(WorthUiCompositionNodeDefinition::new(
        WorthUiCompositionNodeKind::Interaction,
        "submit",
        "submit",
    ))
    .with_node(WorthUiCompositionNodeDefinition::new(
        WorthUiCompositionNodeKind::Control,
        "details",
        "details",
    ))
    .with_node(WorthUiCompositionNodeDefinition::container("card"))
    .with_node(WorthUiCompositionNodeDefinition::new(
        WorthUiCompositionNodeKind::Control,
        "first_name",
        "first_name",
    ))
    .with_policy_attachment(
        "card",
        WorthUiCompositionPolicyKind::LocalLayout,
        "card-flow",
    )
    .with_parent_at("card", "submit", 2)
    .with_root_child("card")
    .with_parent_at("card", "details", 1)
    .with_parent_at("card", "first_name", 0)
}

fn contact_card_graph_reordered_children() -> WorthUiCompositionGraphDefinition {
    WorthUiCompositionGraphDefinition::for_root(WorthUiCompositionRootDefinition::surface(
        "worth.surface.contact",
    ))
    .with_node(WorthUiCompositionNodeDefinition::container("card"))
    .with_node(WorthUiCompositionNodeDefinition::new(
        WorthUiCompositionNodeKind::Control,
        "first_name",
        "first_name",
    ))
    .with_node(WorthUiCompositionNodeDefinition::new(
        WorthUiCompositionNodeKind::Control,
        "details",
        "details",
    ))
    .with_node(WorthUiCompositionNodeDefinition::new(
        WorthUiCompositionNodeKind::Interaction,
        "submit",
        "submit",
    ))
    .with_root_child("card")
    .with_parent_at("card", "details", 0)
    .with_parent_at("card", "first_name", 1)
    .with_parent_at("card", "submit", 2)
    .with_policy_attachment(
        "card",
        WorthUiCompositionPolicyKind::LocalLayout,
        "card-flow",
    )
}
