use worth_ui_host_contract::{
    UiMountedHitTestProjection, UiMountedMechanicalRole, UiMountedNodeProjectionView,
    UiMountedNodeProjectionViewInput, UiMountedOmissionReason, UiMountedPaintProjection,
    UiMountedParticipation, UiMountedParticipationFact, UiMountedParticipationInput,
    UiMountedParticipationStatus, UiMountedSemanticTextMechanic, UiMountedSemanticTextReference,
};

pub(super) fn text_node(
    index: usize,
    row: &UiMountedSemanticTextMechanic,
) -> UiMountedNodeProjectionView {
    let admitted = UiMountedParticipationFact::new(UiMountedParticipationStatus::Admitted);
    let withheld = UiMountedParticipationFact::new(UiMountedParticipationStatus::Withheld);
    let omitted = UiMountedOmissionReason::NotDefinedByCurrentRuntime;
    let reference = UiMountedSemanticTextReference::from_runtime_mounting(
        u16::try_from(index).expect("fixture text row index"),
    );
    UiMountedNodeProjectionView::new(UiMountedNodeProjectionViewInput {
        mounted_instance: row.mounted_instance(),
        node_receipt: row.node_receipt(),
        authored_position: u64::try_from(index).expect("fixture authored position"),
        role: UiMountedMechanicalRole::Control,
        participation: UiMountedParticipation::new(UiMountedParticipationInput {
            paint: admitted,
            clip: admitted,
            input: withheld,
            focus: withheld,
            hit_test: withheld,
            accessibility: withheld,
            motion: withheld,
            diagnostic: withheld,
        }),
        allocation: worth_ui_host_contract::UiMountedAllocationProjection::Known {
            bounds: row.bounds(),
            basis: row.allocation_basis(),
        },
        preview: worth_ui_host_contract::UiMountedPreviewProjection::Omitted(omitted),
        paint: UiMountedPaintProjection::Omitted(omitted),
        hit_test: UiMountedHitTestProjection::Omitted(omitted),
        accessibility: worth_ui_host_contract::UiMountedAccessibilityProjection::Omitted(omitted),
        motion: worth_ui_host_contract::UiMountedMotionProjection::Omitted(omitted),
        diagnostic: worth_ui_host_contract::UiMountedDiagnosticProjection::Omitted(omitted),
        drawables: vec![
            worth_ui_host_contract::UiMountedDrawableReference::SemanticText(reference),
        ],
        semantic_text: vec![reference],
        portal_presentation: None,
    })
}
