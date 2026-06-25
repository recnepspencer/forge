use crate::runtime::{
    WorthUiAuthoredLiveViewProjectionDeclaration, WorthUiLiveViewConditionalProjectionDenial,
    WorthUiLiveViewControlProjectionDenial, WorthUiLiveViewInteractionIntentDenial,
    WorthUiLiveViewPayloadProjectionDenial, WorthUiLiveViewReadinessProjectionDenial,
};

use super::WorthUiLiveViewProjectionAdmissionDenial;

pub(in crate::runtime::live_view) fn projection_denials(
    control_denials: Vec<WorthUiLiveViewControlProjectionDenial>,
    conditional_denials: Vec<WorthUiLiveViewConditionalProjectionDenial>,
) -> Vec<WorthUiLiveViewProjectionAdmissionDenial> {
    control_denials
        .into_iter()
        .map(WorthUiLiveViewProjectionAdmissionDenial::Control)
        .chain(
            conditional_denials
                .into_iter()
                .map(WorthUiLiveViewProjectionAdmissionDenial::Conditional),
        )
        .collect()
}

pub(in crate::runtime::live_view) fn authored_projection_denials(
    projections: &[WorthUiAuthoredLiveViewProjectionDeclaration],
    control_denials: Vec<WorthUiLiveViewControlProjectionDenial>,
    conditional_denials: Vec<WorthUiLiveViewConditionalProjectionDenial>,
    readiness_denials: Vec<WorthUiLiveViewReadinessProjectionDenial>,
    payload_denials: Vec<WorthUiLiveViewPayloadProjectionDenial>,
    interaction_denials: Vec<WorthUiLiveViewInteractionIntentDenial>,
) -> Vec<WorthUiLiveViewProjectionAdmissionDenial> {
    let mut denials = Vec::new();
    for projection in projections {
        match projection {
            WorthUiAuthoredLiveViewProjectionDeclaration::Control(control) => denials.extend(
                control_denials
                    .iter()
                    .filter(|denial| control_denial_matches(denial, control.control_id()))
                    .cloned()
                    .map(WorthUiLiveViewProjectionAdmissionDenial::Control),
            ),
            WorthUiAuthoredLiveViewProjectionDeclaration::Conditional(conditional) => {
                denials.extend(
                    conditional_denials
                        .iter()
                        .filter(|denial| {
                            conditional_denial_matches(denial, conditional.control_id())
                        })
                        .cloned()
                        .map(WorthUiLiveViewProjectionAdmissionDenial::Conditional),
                );
            }
            WorthUiAuthoredLiveViewProjectionDeclaration::Readiness(readiness) => denials.extend(
                readiness_denials
                    .iter()
                    .filter(|denial| readiness_denial_matches(denial, readiness.readiness_id()))
                    .cloned()
                    .map(WorthUiLiveViewProjectionAdmissionDenial::Readiness),
            ),
            WorthUiAuthoredLiveViewProjectionDeclaration::Payload(payload) => denials.extend(
                payload_denials
                    .iter()
                    .filter(|denial| payload_denial_matches(denial, payload.payload_id()))
                    .cloned()
                    .map(WorthUiLiveViewProjectionAdmissionDenial::Payload),
            ),
            WorthUiAuthoredLiveViewProjectionDeclaration::Interaction(interaction) => {
                denials.extend(
                    interaction_denials
                        .iter()
                        .filter(|denial| {
                            interaction_denial_matches(denial, interaction.interaction_id())
                        })
                        .cloned()
                        .map(WorthUiLiveViewProjectionAdmissionDenial::Interaction),
                );
            }
        }
    }
    denials
}

pub(in crate::runtime::live_view) fn readiness_has_denial(
    denials: &[WorthUiLiveViewReadinessProjectionDenial],
    readiness_id: &str,
) -> bool {
    denials
        .iter()
        .any(|denial| readiness_denial_matches(denial, readiness_id))
}

pub(in crate::runtime::live_view) fn conditional_has_denial(
    denials: &[WorthUiLiveViewConditionalProjectionDenial],
    control_id: &str,
) -> bool {
    denials
        .iter()
        .any(|denial| conditional_denial_matches(denial, control_id))
}

fn control_denial_matches(
    denial: &WorthUiLiveViewControlProjectionDenial,
    control_id: &str,
) -> bool {
    match denial {
        WorthUiLiveViewControlProjectionDenial::InvalidControlId { control_id: id }
        | WorthUiLiveViewControlProjectionDenial::DuplicateControlId { control_id: id }
        | WorthUiLiveViewControlProjectionDenial::UnknownBinding { control_id: id, .. }
        | WorthUiLiveViewControlProjectionDenial::UnsupportedProjectionKind {
            control_id: id,
            ..
        }
        | WorthUiLiveViewControlProjectionDenial::UnregisteredComponent {
            control_id: id, ..
        }
        | WorthUiLiveViewControlProjectionDenial::MissingOptions { control_id: id }
        | WorthUiLiveViewControlProjectionDenial::UnsupportedOptionSource {
            control_id: id, ..
        }
        | WorthUiLiveViewControlProjectionDenial::PrimitiveFlowLayout { control_id: id, .. }
        | WorthUiLiveViewControlProjectionDenial::PrimitiveAppearanceState {
            control_id: id, ..
        }
        | WorthUiLiveViewControlProjectionDenial::PrimitiveEventGeometry {
            control_id: id, ..
        } => id == control_id,
    }
}

fn readiness_denial_matches(
    denial: &WorthUiLiveViewReadinessProjectionDenial,
    readiness_id: &str,
) -> bool {
    match denial {
        WorthUiLiveViewReadinessProjectionDenial::InvalidReadinessId { readiness_id: id }
        | WorthUiLiveViewReadinessProjectionDenial::EmptyRequiredSet { readiness_id: id }
        | WorthUiLiveViewReadinessProjectionDenial::UnknownRequiredBinding {
            readiness_id: id,
            ..
        } => id == readiness_id,
    }
}

fn payload_denial_matches(
    denial: &WorthUiLiveViewPayloadProjectionDenial,
    payload_id: &str,
) -> bool {
    match denial {
        WorthUiLiveViewPayloadProjectionDenial::InvalidPayloadId { payload_id: id }
        | WorthUiLiveViewPayloadProjectionDenial::UnsupportedPayloadShape {
            payload_id: id, ..
        } => id == payload_id,
    }
}

fn interaction_denial_matches(
    denial: &WorthUiLiveViewInteractionIntentDenial,
    interaction_id: &str,
) -> bool {
    match denial {
        WorthUiLiveViewInteractionIntentDenial::InvalidInteractionId { interaction_id: id }
        | WorthUiLiveViewInteractionIntentDenial::UnsupportedKind {
            interaction_id: id, ..
        }
        | WorthUiLiveViewInteractionIntentDenial::UnsupportedEffect {
            interaction_id: id, ..
        }
        | WorthUiLiveViewInteractionIntentDenial::UnknownReadiness {
            interaction_id: id, ..
        }
        | WorthUiLiveViewInteractionIntentDenial::UnknownPayload {
            interaction_id: id, ..
        }
        | WorthUiLiveViewInteractionIntentDenial::PrimitiveFlowLayout {
            interaction_id: id, ..
        }
        | WorthUiLiveViewInteractionIntentDenial::PrimitiveAppearanceState {
            interaction_id: id,
            ..
        }
        | WorthUiLiveViewInteractionIntentDenial::PrimitiveEventGeometry {
            interaction_id: id,
            ..
        } => id == interaction_id,
    }
}

fn conditional_denial_matches(
    denial: &WorthUiLiveViewConditionalProjectionDenial,
    control_id: &str,
) -> bool {
    match denial {
        WorthUiLiveViewConditionalProjectionDenial::UnknownControl { control_id: id }
        | WorthUiLiveViewConditionalProjectionDenial::UnknownConditionBinding {
            control_id: id,
            ..
        }
        | WorthUiLiveViewConditionalProjectionDenial::UnsupportedCondition {
            control_id: id, ..
        }
        | WorthUiLiveViewConditionalProjectionDenial::UnsupportedParticipation {
            control_id: id,
            ..
        } => id == control_id,
    }
}
