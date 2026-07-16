use worth_query::facade::foundation::worth_ui_query_binding_evidence_identity;
use worth_query::facade::runtime::{
    WorthQueryEvidenceIdentity, WorthQueryRuntimeAsyncResultState,
    WorthQueryRuntimeAsyncResultStateKind,
};
use worth_ui::facade::{
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
        RuntimeOutcomeSourceReference::from_query_async_result_state(&async_result_state(
            WorthQueryRuntimeAsyncResultStateKind::Current,
            "ready-causality",
        )),
    )
    .with_presentation(
        RuntimeOutcomePresentation::new()
            .with_label("Ready")
            .with_tone(RuntimeOutcomeTone::positive())
            .with_affordance(RuntimeOutcomeAffordance::none()),
    )
}

pub(crate) fn denied_source_reference() -> RuntimeOutcomeSourceReference {
    RuntimeOutcomeSourceReference::from_query_async_result_state(&async_result_state(
        WorthQueryRuntimeAsyncResultStateKind::Denied,
        "denied-causality",
    ))
}

pub(crate) fn failed_source_reference() -> RuntimeOutcomeSourceReference {
    RuntimeOutcomeSourceReference::from_query_async_result_state(&async_result_state(
        WorthQueryRuntimeAsyncResultStateKind::Failed,
        "failed-causality",
    ))
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

fn async_result_state(
    kind: WorthQueryRuntimeAsyncResultStateKind,
    causality_digest: &str,
) -> WorthQueryRuntimeAsyncResultState {
    WorthQueryRuntimeAsyncResultState::new(
        kind,
        &fixture_async_evidence_identity(causality_digest),
        &fixture_async_evidence_identity("basis.digest"),
        &fixture_async_evidence_identity("generation.digest"),
    )
}

fn fixture_async_evidence_identity(label: &str) -> WorthQueryEvidenceIdentity {
    worth_ui_query_binding_evidence_identity("runtime-outcome-fixture", &[label.to_string()])
}
