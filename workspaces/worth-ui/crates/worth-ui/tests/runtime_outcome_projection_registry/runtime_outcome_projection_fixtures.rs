use worth_ui::facade::registry::{
    RuntimeOutcomeAffordance, RuntimeOutcomeDenialPosture, RuntimeOutcomeFamily,
    RuntimeOutcomePresentation, RuntimeOutcomeProjectionDescriptor, RuntimeOutcomeProjectionId,
    RuntimeOutcomeRecoveryPosture, RuntimeOutcomeSourceReference, RuntimeOutcomeTone,
};

pub(crate) fn denied_projection(id: &str) -> RuntimeOutcomeProjectionDescriptor {
    RuntimeOutcomeProjectionDescriptor::new(
        projection_id(id),
        RuntimeOutcomeFamily::denied(),
        denied_source_reference(),
    )
    .with_presentation(blocking_presentation("Needs permission"))
    .with_denial_posture(RuntimeOutcomeDenialPosture::structured_status())
}

pub(crate) fn failed_projection(id: &str) -> RuntimeOutcomeProjectionDescriptor {
    RuntimeOutcomeProjectionDescriptor::new(
        projection_id(id),
        RuntimeOutcomeFamily::failed(),
        failed_source_reference(),
    )
    .with_presentation(blocking_presentation("Failed"))
    .with_recovery_posture(RuntimeOutcomeRecoveryPosture::retry_hint())
}

pub(crate) fn ready_projection(id: &str) -> RuntimeOutcomeProjectionDescriptor {
    RuntimeOutcomeProjectionDescriptor::new(
        projection_id(id),
        RuntimeOutcomeFamily::ready(),
        RuntimeOutcomeSourceReference::new(RuntimeOutcomeFamily::ready()),
    )
    .with_presentation(
        RuntimeOutcomePresentation::new()
            .with_label("Ready")
            .with_tone(RuntimeOutcomeTone::positive())
            .with_affordance(RuntimeOutcomeAffordance::none()),
    )
}

pub(crate) fn denied_source_reference() -> RuntimeOutcomeSourceReference {
    RuntimeOutcomeSourceReference::new(RuntimeOutcomeFamily::denied())
}

pub(crate) fn failed_source_reference() -> RuntimeOutcomeSourceReference {
    RuntimeOutcomeSourceReference::new(RuntimeOutcomeFamily::failed())
}

pub(crate) fn blocking_presentation(label: &str) -> RuntimeOutcomePresentation {
    RuntimeOutcomePresentation::new()
        .with_label(label)
        .with_tone(RuntimeOutcomeTone::blocking())
        .with_affordance(RuntimeOutcomeAffordance::recoverable_action())
}

pub(crate) fn projection_id(raw_text: &str) -> RuntimeOutcomeProjectionId {
    RuntimeOutcomeProjectionId::new(raw_text).expect("valid runtime outcome projection id")
}
