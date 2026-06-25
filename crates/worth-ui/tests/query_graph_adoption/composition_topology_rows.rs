use worth_ui::facade::{
    WorthUiQueryGraphObligationSemantic, WorthUiRuntimeFactFamily, WorthUiRuntimeFactId,
    WorthUiRuntimeGraphAuthority,
};

#[test]
fn composition_topology_uses_query_graph_execution_rows_for_graph_facts() {
    let graph_authority = WorthUiRuntimeGraphAuthority::new();
    let dependency_facts = [
        WorthUiRuntimeFactId::composition_root("composition.root.surface.worth.surface.contact"),
        WorthUiRuntimeFactId::composition_node("card"),
        WorthUiRuntimeFactId::composition_node("submit"),
        WorthUiRuntimeFactId::composition_edge("node:card->0:submit"),
        WorthUiRuntimeFactId::composition_policy("card:local_layout:card-flow"),
    ];
    let receipt = graph_authority
        .plan_composition_topology_graph_operation(
            "composition.root.surface.worth.surface.contact",
            dependency_facts.clone(),
        )
        .into_execution_receipt();
    let semantics = receipt
        .rows()
        .iter()
        .map(|row| row.semantic())
        .collect::<Vec<_>>();
    let touched_descriptor = receipt.touch_descriptor().descriptor();
    let fact_families = dependency_facts
        .iter()
        .map(WorthUiRuntimeFactId::family)
        .collect::<Vec<_>>();

    assert_eq!(receipt.selected_obligation_count(), 5);
    for expected in WorthUiQueryGraphObligationSemantic::COMPOSITION_TOPOLOGY {
        assert!(
            semantics.contains(&expected),
            "missing composition topology graph semantic {expected:?}"
        );
    }
    assert!(fact_families.contains(&WorthUiRuntimeFactFamily::CompositionRoot));
    assert!(fact_families.contains(&WorthUiRuntimeFactFamily::CompositionNode));
    assert!(fact_families.contains(&WorthUiRuntimeFactFamily::CompositionEdge));
    assert!(fact_families.contains(&WorthUiRuntimeFactFamily::CompositionPolicy));
    assert!(!touched_descriptor.descriptor_digest().is_empty());
    assert!(receipt.execution_digest() > 0);
}
