use worth_ui_inspection::{
    UiIntentInteractionEvidenceFamily, UiIntentInteractionEvidenceInput,
    UiIntentInteractionEvidenceTargetInput,
};

pub(crate) fn command_evidence_input(
    receipt: &crate::runtime::command_routing::UiCommandRouteReceipt,
    target: crate::runtime::interaction::UiPresentedInteractionTargetView,
) -> Option<UiIntentInteractionEvidenceInput> {
    let sequence = receipt.sequence()?.value();
    let presentation = target.presentation();
    let target = UiIntentInteractionEvidenceTargetInput::from_diagnostic_parts(
        presentation.frame().diagnostic_value(),
        presentation.epoch().diagnostic_value(),
        target.mounted_instance().diagnostic_value(),
        target.semantic_digest(),
    );
    Some(UiIntentInteractionEvidenceInput::from_diagnostic_parts(
        sequence,
        target,
        UiIntentInteractionEvidenceFamily::CommandRoute,
    ))
}
