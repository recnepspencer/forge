use worth_ui::facade::{
    admit_composition_graph_access, WorthUiCompositionGraphAccessDenialCode,
    WorthUiCompositionGraphAccessRequest, WorthUiCompositionGraphDefinition,
    WorthUiCompositionNodeDefinition, WorthUiCompositionNodeId, WorthUiCompositionNodeKind,
    WorthUiCompositionParticipation, WorthUiCompositionPolicyKind,
    WorthUiCompositionRootDefinition,
};

#[test]
fn mounted_product_tree_access_reads_indexed_children_without_local_traversal() {
    let graph = nested_card_graph().admit().expect("graph should admit");
    let access = admit_composition_graph_access(
        &graph,
        WorthUiCompositionGraphAccessRequest::mounted_product_tree(),
    )
    .expect("mounted tree access should admit");

    assert_eq!(access.root_children().len(), 1);
    assert_eq!(access.ordered_children("card").len(), 2);
    assert_eq!(access.ordered_children("field_row").len(), 2);
    assert_eq!(access.counters().child_lookup_count(), graph.edges().len());
    assert_eq!(
        access.counters().index_build_node_count(),
        graph.nodes().len()
    );
    assert_eq!(
        access.counters().index_build_edge_count(),
        graph.edges().len()
    );
    assert_eq!(
        access.counters().index_build_policy_count(),
        graph.policy_attachments().len()
    );
    assert_eq!(
        access.counters().materialized_row_count(),
        access.child_rows().len()
            + graph
                .nodes()
                .iter()
                .map(|node| access.ancestors_of(node.node_id().as_str()).len())
                .sum::<usize>()
            + access.affected_consumers().len()
            + access.participating_descendants().len()
    );
    assert_eq!(access.counters().caller_owned_recursive_walk_count(), 0);
    assert_eq!(access.counters().caller_owned_scan_count(), 0);
    assert_eq!(access.counters().source_reparse_count(), 0);
    assert_eq!(access.counters().renderer_parse_count(), 0);
    assert_eq!(
        access
            .plan()
            .query_graph_execution()
            .selected_obligation_count(),
        5
    );
}

#[test]
fn ancestor_and_participation_reads_use_access_receipt_indexes() {
    let graph = nested_card_graph().admit().expect("graph should admit");
    let access = admit_composition_graph_access(
        &graph,
        WorthUiCompositionGraphAccessRequest::mounted_product_tree(),
    )
    .expect("mounted tree access should admit");

    let ancestors = access.ancestors_of("first_name");
    assert_eq!(ancestors.len(), 3);
    assert_eq!(ancestors[0].ancestor_id(), "field_row");
    assert_eq!(ancestors[1].ancestor_id(), "card");

    let participating = access.participating_descendants();
    assert!(participating
        .iter()
        .any(|row| row.node().node_id().as_str() == "first_name"));
    assert!(!participating
        .iter()
        .any(|row| row.node().node_id().as_str() == "archived_note"));
    assert_eq!(
        access.counters().participation_filter_count(),
        graph
            .edges()
            .iter()
            .filter(|edge| edge.child().as_str() != "archived_note")
            .count()
    );
}

#[test]
fn affected_consumer_rows_are_receipt_backed() {
    let graph = nested_card_graph().admit().expect("graph should admit");
    let access = admit_composition_graph_access(
        &graph,
        WorthUiCompositionGraphAccessRequest::mounted_product_tree(),
    )
    .expect("mounted tree access should admit");

    assert_eq!(access.affected_consumers().len(), graph.edges().len());
    assert!(access
        .affected_consumers()
        .iter()
        .all(|row| row.semantic_slice() == "MountedCompositionTree"));
    assert_eq!(
        access.counters().affected_consumer_lookup_count(),
        graph.edges().len()
    );
}

#[test]
fn narrow_child_access_materializes_only_requested_parent_rows() {
    let graph = nested_card_graph().admit().expect("graph should admit");
    let access = admit_composition_graph_access(
        &graph,
        WorthUiCompositionGraphAccessRequest::ordered_children("field_row"),
    )
    .expect("field row child access should admit");

    assert!(access.root_children().is_empty());
    assert_eq!(access.child_rows().len(), 2);
    assert!(access
        .child_rows()
        .iter()
        .all(|row| row.parent_id() == "field_row"));
    assert!(access.ancestors_of("first_name").is_empty());
    assert!(access.affected_consumers().is_empty());
    assert_eq!(access.counters().child_lookup_count(), 2);
    assert_eq!(access.counters().materialized_row_count(), 2);
}

#[test]
fn parent_and_changed_edge_reads_are_narrow_receipt_backed_accesses() {
    let graph = nested_card_graph().admit().expect("graph should admit");
    let parent_access = admit_composition_graph_access(
        &graph,
        WorthUiCompositionGraphAccessRequest::parent_of(
            WorthUiCompositionNodeId::new("first_name").expect("valid node id"),
        ),
    )
    .expect("parent access should admit");

    assert_eq!(parent_access.parent_of("first_name"), Some("field_row"));
    assert_eq!(parent_access.ancestors_of("first_name").len(), 1);
    assert!(parent_access.child_rows().is_empty());
    assert_eq!(parent_access.counters().ancestor_lookup_count(), 1);

    let edge_identity = graph
        .edges()
        .iter()
        .find(|edge| edge.child().as_str() == "first_name")
        .expect("first name edge exists")
        .fact_id()
        .identity()
        .to_owned();
    let affected_access = admit_composition_graph_access(
        &graph,
        WorthUiCompositionGraphAccessRequest::affected_consumers_for_changed_edge(&edge_identity),
    )
    .expect("affected consumer access should admit");

    assert_eq!(affected_access.affected_consumers().len(), 1);
    assert_eq!(
        affected_access.affected_consumers()[0]
            .changed_fact()
            .identity(),
        edge_identity
    );
    assert!(affected_access.child_rows().is_empty());
    assert_eq!(
        affected_access.counters().affected_consumer_lookup_count(),
        1
    );
}

#[test]
fn missing_parent_access_rejects_instead_of_scanning() {
    let graph = nested_card_graph().admit().expect("graph should admit");
    let denial = admit_composition_graph_access(
        &graph,
        WorthUiCompositionGraphAccessRequest::ordered_children("missing_parent"),
    )
    .expect_err("missing parent access should deny");

    assert_ne!(denial.denial_set_digest(), 0);
    assert_eq!(denial.denials().len(), 1);
    assert_eq!(
        denial.denials()[0].code(),
        WorthUiCompositionGraphAccessDenialCode::MissingParent
    );
    assert_eq!(denial.denials()[0].subject(), "missing_parent");
}

#[test]
fn missing_node_edge_and_policy_access_reject_through_typed_denials() {
    let graph = nested_card_graph().admit().expect("graph should admit");

    let missing_node = admit_composition_graph_access(
        &graph,
        WorthUiCompositionGraphAccessRequest::ancestors_of(
            WorthUiCompositionNodeId::new("missing_node").expect("valid node id"),
        ),
    )
    .expect_err("missing node access should deny");
    assert_eq!(
        missing_node.denials()[0].code(),
        WorthUiCompositionGraphAccessDenialCode::MissingNode
    );

    let missing_edge = admit_composition_graph_access(
        &graph,
        WorthUiCompositionGraphAccessRequest::affected_consumers_for_changed_edge(
            "composition_edge:missing",
        ),
    )
    .expect_err("missing edge access should deny");
    assert_eq!(
        missing_edge.denials()[0].code(),
        WorthUiCompositionGraphAccessDenialCode::MissingEdge
    );

    let missing_policy = admit_composition_graph_access(
        &graph,
        WorthUiCompositionGraphAccessRequest::affected_consumers_for_changed_policy(
            "composition_policy:missing",
        ),
    )
    .expect_err("missing policy access should deny");
    assert_eq!(
        missing_policy.denials()[0].code(),
        WorthUiCompositionGraphAccessDenialCode::MissingPolicy
    );
    assert_ne!(
        missing_policy.denial_set_digest(),
        missing_edge.denial_set_digest()
    );
}

fn nested_card_graph() -> WorthUiCompositionGraphDefinition {
    WorthUiCompositionGraphDefinition::for_root(WorthUiCompositionRootDefinition::surface(
        "worth.surface.phase3",
    ))
    .with_node(WorthUiCompositionNodeDefinition::container("card"))
    .with_node(WorthUiCompositionNodeDefinition::container("field_row"))
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
    .with_node(
        WorthUiCompositionNodeDefinition::new(
            WorthUiCompositionNodeKind::Text,
            "archived_note",
            "archived_note",
        )
        .with_participation(WorthUiCompositionParticipation::AbsentRetainsState),
    )
    .with_root_child("card")
    .with_parent("card", "field_row")
    .with_parent("field_row", "first_name")
    .with_parent("field_row", "details")
    .with_parent("card", "archived_note")
    .with_policy_attachment("card", WorthUiCompositionPolicyKind::LocalLayout, "stack")
}
