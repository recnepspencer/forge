use worth_ui::facade::{
    DensityTokenId, WorthUiPrimitiveContentGraphPosture, WorthUiPrimitiveEventGraphDispatchPosture,
    WorthUiQueryGraphObligationSemantic, WorthUiRuntimeFactId, WorthUiRuntimeGraphAuthority,
};

use super::support::support_status_for;

#[test]
fn primitive_event_dispatch_uses_query_graph_execution_rows() {
    let graph_authority = WorthUiRuntimeGraphAuthority::new();
    let receipt = graph_authority
        .plan_primitive_event_dispatch_graph_operation(
            "worth.surface.preview.primitive.inner",
            "worth.interaction.primitive.inner",
            [
                WorthUiRuntimeFactId::primitive_event_region(
                    "worth.surface.preview.primitive.inner",
                ),
                WorthUiRuntimeFactId::primitive_event_geometry(
                    "worth.surface.preview.primitive.inner",
                ),
            ],
            WorthUiPrimitiveEventGraphDispatchPosture::DisabledHit,
        )
        .into_execution_receipt();
    let semantics = receipt
        .rows()
        .iter()
        .map(|row| row.semantic())
        .collect::<Vec<_>>();

    assert_eq!(receipt.selected_obligation_count(), 6);
    for expected in WorthUiQueryGraphObligationSemantic::PRIMITIVE_EVENT_DISPATCH {
        assert!(
            semantics.contains(&expected),
            "missing primitive event graph semantic {expected:?}"
        );
    }
    assert_eq!(
        support_status_for(
            &receipt,
            WorthUiQueryGraphObligationSemantic::EventDisabledBlock
        ),
        "diagnostic-only"
    );
    assert_eq!(
        support_status_for(
            &receipt,
            WorthUiQueryGraphObligationSemantic::EventCapturePolicy
        ),
        "not-applicable"
    );
    assert_eq!(
        support_status_for(
            &receipt,
            WorthUiQueryGraphObligationSemantic::EventPropagation
        ),
        "not-applicable"
    );
    assert!(receipt.execution_digest() > 0);
}

#[test]
fn primitive_content_anatomy_uses_query_graph_execution_rows() {
    let surface = "worth.surface.preview.primitive.content";
    let graph_authority = WorthUiRuntimeGraphAuthority::new();
    let receipt = graph_authority
        .plan_primitive_content_anatomy_graph_operation(
            surface,
            [
                WorthUiRuntimeFactId::primitive_content(surface),
                WorthUiRuntimeFactId::density_token(
                    &DensityTokenId::new("validation.density.primitive.content.icon.default")
                        .expect("fixture density token id is valid"),
                ),
            ],
            WorthUiPrimitiveContentGraphPosture::FallbackEligible,
        )
        .into_execution_receipt();
    let semantics = receipt
        .rows()
        .iter()
        .map(|row| row.semantic())
        .collect::<Vec<_>>();

    assert_eq!(receipt.selected_obligation_count(), 6);
    for expected in WorthUiQueryGraphObligationSemantic::PRIMITIVE_CONTENT_ANATOMY {
        assert!(
            semantics.contains(&expected),
            "missing primitive content graph semantic {expected:?}"
        );
    }
    assert_eq!(
        support_status_for(
            &receipt,
            WorthUiQueryGraphObligationSemantic::ContentVectorPosture
        ),
        "diagnostic-only"
    );
    assert_eq!(
        support_status_for(
            &receipt,
            WorthUiQueryGraphObligationSemantic::ContentSlotParticipation
        ),
        "supported"
    );
    assert!(receipt.execution_digest() > 0);
}
