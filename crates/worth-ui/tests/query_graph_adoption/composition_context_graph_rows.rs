use worth_ui::facade::{
    WorthUiQueryGraphObligationSemantic, WorthUiRuntimeFactId, WorthUiRuntimeGraphAuthority,
};

use super::support::support_status_for;

#[test]
fn composition_context_propagation_uses_query_graph_execution_rows() {
    let root = "composition.root.surface.validation.context";
    let graph_authority = WorthUiRuntimeGraphAuthority::new();
    let receipt = graph_authority
        .plan_composition_context_graph_operation(
            root,
            [
                WorthUiRuntimeFactId::composition_root(root),
                WorthUiRuntimeFactId::composition_context("validation.node.card"),
                WorthUiRuntimeFactId::composition_context_propagation(root),
            ],
        )
        .into_execution_receipt();
    let semantics = receipt
        .rows()
        .iter()
        .map(|row| row.semantic())
        .collect::<Vec<_>>();

    assert_eq!(receipt.selected_obligation_count(), 5);
    for expected in WorthUiQueryGraphObligationSemantic::COMPOSITION_CONTEXT {
        assert!(
            semantics.contains(&expected),
            "missing composition context graph semantic {expected:?}"
        );
    }
    assert_eq!(
        support_status_for(
            &receipt,
            WorthUiQueryGraphObligationSemantic::CompositionContextDisabledSuppression
        ),
        "supported"
    );
    assert!(receipt.execution_digest() > 0);
}
